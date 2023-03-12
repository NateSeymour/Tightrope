#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Goal {
    pub goal_id: u32,
    pub metric_id: u32,
    pub username: String,
    pub target_value: f64,
    pub start_millis: u64,
    pub end_millis: u64,
    pub name: String,
    pub description: String,
}