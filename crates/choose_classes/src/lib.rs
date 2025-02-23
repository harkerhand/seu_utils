pub mod utils;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub token: String,
    pub batch_id: String,
    pub loginname: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WantCourse {
    pub name: String,
    pub course_type: String,
    pub course_id: String,
    pub course_secret: String,
}
