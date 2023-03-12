#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Measurement {
    pub measurement_id: u32,
    pub metric_id: u32,
    pub username: String,
    pub millis: u64,
    pub value: f64,
}