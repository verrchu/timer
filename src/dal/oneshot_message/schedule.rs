use sqlx::{query, Error, PgConnection, Row};
use uuid::Uuid;

use crate::dao::OneshotMessage;

static MESSAGE_UNIQUE_CONSTRAINT: &str = "oneshot_message_schedule_pkey";
static MESSAGE_CONTENT_CONSTRAINT: &str = "oneshot_message_nonempty_content_check";
static MESSAGE_SCHEDULED_AT_CONSTRAINT: &str = "oneshot_message_scheduled_at_future_check";
static MESSAGE_USER_ID_CONSTRAINT: &str = "oneshot_message_schedule_user_id";

#[derive(Debug)]
pub enum QueryError {
    Generic(eyre::Report),
    ConstraintError(ConstraintError),
}

#[derive(Debug)]
pub enum ConstraintError {
    EmptyMessageContent,
    InvalidMessageScheduleTime,
    UserDoesNotExist,
    MessageAlreadyScheduled,
}

pub async fn schedule(
    conn: &mut PgConnection,
    message: OneshotMessage,
) -> Result<Uuid, QueryError> {
    let result = query(
        r#"
insert into timer.oneshot_message_schedule(
    user_id, content, scheduled_at
) values ($1, $2, $3)
returning message_id
"#,
    )
    .bind(&message.user_id)
    .bind(&message.content)
    .bind(&message.scheduled_at)
    .fetch_one(conn)
    .await
    .map(|row| row.get("message_id"));

    match result {
        Ok(message_id) => Ok(message_id),
        Err(Error::Database(inner)) => {
            if let Some(constraint) = inner.constraint() {
                if constraint == MESSAGE_CONTENT_CONSTRAINT {
                    Err(QueryError::ConstraintError(
                        ConstraintError::EmptyMessageContent,
                    ))
                } else if constraint == MESSAGE_SCHEDULED_AT_CONSTRAINT {
                    Err(QueryError::ConstraintError(
                        ConstraintError::InvalidMessageScheduleTime,
                    ))
                } else if constraint == MESSAGE_USER_ID_CONSTRAINT {
                    Err(QueryError::ConstraintError(
                        ConstraintError::UserDoesNotExist,
                    ))
                } else if constraint == MESSAGE_UNIQUE_CONSTRAINT {
                    Err(QueryError::ConstraintError(
                        ConstraintError::MessageAlreadyScheduled,
                    ))
                } else {
                    Err(Error::Database(inner))
                        .map_err(Into::<eyre::Report>::into)
                        .map_err(QueryError::Generic)
                }
            } else {
                Err(Error::Database(inner))
                    .map_err(Into::<eyre::Report>::into)
                    .map_err(QueryError::Generic)
            }
        }
        Err(err) => Err(err)
            .map_err(Into::<eyre::Report>::into)
            .map_err(QueryError::Generic),
    }
}
