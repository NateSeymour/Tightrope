#[derive(serde::Serialize, serde::Deserialize, Clone, sqlx::Type)]
pub enum MetricType {
    Mass,
    Speed,
    Time,
    Length,
    PsychScale,
    Binary,
}

#[derive(serde::Serialize, Clone)]
pub struct Metric {
    pub metric_id: u32,
    pub metric_type: MetricType,
    pub username: Option<String>,
    pub name: String,
    pub description: Option<String>,
}