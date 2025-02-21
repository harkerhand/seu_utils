pub mod utils;

use serde::{Deserialize, Serialize};

/// 学期
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Term {
    /// 起始年份
    pub start_year: u16,
    /// 结束年份
    pub end_year: u16,
    /// 学期
    pub term: u8,
}
impl From<&str> for Term {
    fn from(s: &str) -> Self {
        let mut iter = s.split('-');
        let start_year = iter.next().unwrap().parse().unwrap();
        let end_year = iter.next().unwrap().parse().unwrap();
        let term = iter.next().unwrap().parse().unwrap();
        Term {
            start_year,
            end_year,
            term,
        }
    }
}

/// 课程
#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    /// 课程名称
    pub name: String,
    /// 课程类型
    pub class_type: String,
    /// 课程分数
    pub score: u8,
    /// 课程学分
    pub credit: f32,
    /// 课程学期
    pub term: Term,
}
