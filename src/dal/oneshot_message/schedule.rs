use sqlx::{query, Error, PgConnection, Row};
use uuid::Uuid;

use crate::dao::OneshotMessage;

static MESSAGE_DATA_CONSTRAINT: &str = "oneshot_message_data_nonempty_check";
static MESSAGE_SCHEDULED_AT_CONSTRAINT: &str = "oneshot_message_scheduled_at_future_check";

#[derive(Debug)]
pub enum QueryError {
    Generic(eyre::Report),
    ConstraintError(ConstraintError),
}

#[derive(Debug)]
pub enum ConstraintError {
    EmptyMessageData,
    InvalidMessageScheduleTime,
}

pub async fn schedule(conn: &mut PgConnection, data: &OneshotMessage) -> Result<Uuid, QueryError> {
    let result = query(
        r#"
insert into timer.oneshot_message_schedule(
    message_id, data, scheduled_at
) values ($1, $2, $3)
returning message_id
"#,
    )
    .bind(&data.message_id)
    .bind(&data.data)
    .bind(&data.scheduled_at)
    .fetch_one(conn)
    .await
    .map(|row| row.get("message_id"));

    match result {
        Ok(message_id) => Ok(message_id),
        Err(Error::Database(inner)) => {
            if let Some(constraint) = inner.constraint() {
                if constraint == MESSAGE_DATA_CONSTRAINT {
                    Err(QueryError::ConstraintError(
                        ConstraintError::EmptyMessageData,
                    ))
                } else if constraint == MESSAGE_SCHEDULED_AT_CONSTRAINT {
                    Err(QueryError::ConstraintError(
                        ConstraintError::InvalidMessageScheduleTime,
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
