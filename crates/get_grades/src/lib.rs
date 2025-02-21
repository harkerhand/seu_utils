use serde::Serialize;

/// 课程
#[derive(Debug, Serialize)]
pub struct Course {
    /// 课程名称
    pub name: String,
    /// 课程类型
    pub class_type: String,
    /// 课程密钥
    pub score: u8,
}
