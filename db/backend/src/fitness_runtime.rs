pub enum FitnessMode {
    Development,
    Production
}

pub struct FitnessRuntime {
    pub mode: FitnessMode,
    pub database_url: String,
}