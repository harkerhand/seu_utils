use get_grades::utils::calc_with_terms;
use get_grades::{Course, Term};
use std::fs;

fn main() {
    let grades: Vec<Course> =
        serde_json::from_str(&fs::read_to_string("resource/grades.json").unwrap()).unwrap();
    let terms: Vec<Term> = vec!["2024-2025-1".into(), "2024-2025-2".into()];
    calc_with_terms(grades, terms);
}
