use sqlx::{query, PgConnection, Row};
use uuid::Uuid;

pub async fn select_unqueued(conn: &mut PgConnection) -> eyre::Result<Vec<Uuid>> {
    query(
        r#"
select m.message_id from timer.oneshot_message M
left join timer.oneshot_message_progress P using(message_id)
where P.queued_at is null and M.scheduled_at < now()
"#,
    )
    .fetch_all(conn)
    .await
    .map(|rows| rows.iter().map(|row| row.get("message_id")).collect())
    .map_err(Into::<eyre::Report>::into)
}
