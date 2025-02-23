use clap::Parser;
use get_grades::utils::calc_with_terms;
use get_grades::{Course, Term};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version, name = "calc_gpa")]
#[command(version, about, long_about = None)]
struct Cli {
    /// 保存导出成绩的json文件路径 IN_NEED
    #[clap(long, default_value = "resource/grades.json")]
    grades_json: PathBuf,
    /// 学期列表，逗号隔开，不填则默认计算所有学期 [example: 2024-2025-1, 2024-2025-2]
    #[clap(long, default_value = "")]
    terms: String,
}

fn main() {
    let cli = Cli::parse();
    let grades: Vec<Course> =
        serde_json::from_str(&fs::read_to_string(&cli.grades_json).unwrap()).unwrap();
    let terms = if cli.terms.is_empty() {
        vec![]
    } else {
        cli.terms
            .split(',')
            .map(|s| s.trim().into())
            .collect::<Vec<Term>>()
    };

    calc_with_terms(grades, terms);
}
