use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub token: String,
    pub batch_id: String,
    pub route: String,
    pub username: String,
    pub password: String,
}
