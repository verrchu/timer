use sqlx::{query, PgConnection, Row};
use uuid::Uuid;

use crate::dao::OneshotMessage;

pub async fn insert(conn: &mut PgConnection, data: &OneshotMessage) -> eyre::Result<Uuid> {
    query(
        r#"
insert into timer.oneshot_message(
    message_id, data, scheduled_at
) values (?, ?, ?)
returning message_id
"#,
    )
    .bind(&data.message_id)
    .bind(&data.data)
    .bind(&data.scheduled_at)
    .fetch_one(conn)
    .await
    .map(|row| row.get("message_id"))
    .map_err(Into::<eyre::Report>::into)
}
