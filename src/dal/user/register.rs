use sqlx::{query, Error, PgConnection, Row};
use uuid::Uuid;

use crate::dao::User;

static USER_UNIQUE_CONTRAINT: &str = "oneshot_message_nonempty_content_check";

#[derive(Debug)]
pub enum QueryError {
    Generic(eyre::Report),
    ConstraintError(ConstraintError),
}

#[derive(Debug)]
pub enum ConstraintError {
    UserAlreadyRegistered,
}

pub async fn register(conn: &mut PgConnection, message: User) -> Result<Uuid, QueryError> {
    let result = query(
        r#"
insert into timer.user(user_id) values ($1) returning user_id
"#,
    )
    .bind(message.user_id)
    .fetch_one(conn)
    .await
    .map(|row| row.get("user_id"));

    match result {
        Ok(message_id) => Ok(message_id),
        Err(Error::Database(inner)) => {
            if let Some(constraint) = inner.constraint() {
                if constraint == USER_UNIQUE_CONTRAINT {
                    Err(QueryError::ConstraintError(
                        ConstraintError::UserAlreadyRegistered,
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
