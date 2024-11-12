pub struct Config {
    pub substrate_url: String,
    pub database_url: String,
}

impl Config {
    pub fn load() -> Self {
        Self {
            substrate_url: "ws://127.0.0.1:9944".to_string(),
            database_url: "mongodb://localhost:27017".to_string(),
        }
    }
}
