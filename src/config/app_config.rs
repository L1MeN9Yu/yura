use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub listen_address: String,
    pub database: Database,
    pub logs_directory: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database {
    pub url: String,
}
