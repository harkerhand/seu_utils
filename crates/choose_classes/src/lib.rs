pub mod utils;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    /// 选课系统的token
    pub token: String,
    /// 选课系统的batch_id
    pub batch_id: String,
    /// 一卡通号
    pub loginname: String,
    /// 选课系统的加密密码
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WantCourse {
    pub name: String,
    pub course_type: String,
    pub course_id: String,
    pub course_secret: String,
}
