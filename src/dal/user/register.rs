use sqlx::{query, Error, PgConnection, Row};
use uuid::Uuid;

use crate::dao::User;

static USER_ALIAS_UNIQUE_CONTRAINT: &str = "user_alias_key";

#[derive(Debug)]
pub enum QueryError {
    Generic(eyre::Report),
    ConstraintError(ConstraintError),
}

#[derive(Debug)]
pub enum ConstraintError {
    UserAliasAlreadyTaken,
}

pub async fn register(conn: &mut PgConnection, user: User) -> Result<Uuid, QueryError> {
    let result = query(
        r#"
insert into timer.user(alias) values ($1) returning user_id
"#,
    )
    .bind(user.alias)
    .fetch_one(conn)
    .await
    .map(|row| row.get("user_id"));

    match result {
        Ok(message_id) => Ok(message_id),
        Err(Error::Database(inner)) => {
            if let Some(constraint) = inner.constraint() {
                if constraint == USER_ALIAS_UNIQUE_CONTRAINT {
                    Err(QueryError::ConstraintError(
                        ConstraintError::UserAliasAlreadyTaken,
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
