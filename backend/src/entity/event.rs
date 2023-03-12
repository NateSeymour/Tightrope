#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Event {
    pub event_id: u32,
    pub username: String,
    pub millis: u64,
    pub name: String,
    pub description: String,
}