//! Utilities for get_grades
use crate::{Course, Term};

/// Get GPA from grade
fn get_gpa(grade: u8) -> f32 {
    match grade {
        96..=100 => 4.8,
        93..=95 => 4.5,
        90..=92 => 4.0,
        86..=89 => 3.8,
        83..=85 => 3.5,
        80..=82 => 3.0,
        76..=79 => 2.8,
        73..=75 => 2.5,
        70..=72 => 2.0,
        66..=69 => 1.8,
        63..=65 => 1.5,
        60..=62 => 1.0,
        _ => 0.0,
    }
}

/// Calculate GPA with terms
///
/// # Returns
/// - The GPA
///
/// # Note
/// - If `terms` is empty, all terms will be calculated
/// - The result will be printed
///
/// # Example
/// ```rust
/// use get_grades::{Course, Term};
/// use get_grades::utils::calc_with_terms;
/// let grades: Vec<Course> = vec![
///     Course {
///         name: "Course 1".into(),
///         class_type: "必修".into(),
///         score: 90,
///         credit: 3.0,
///         term: "2024-2025-1".into(),
///     },
///     Course {
///         name: "Course 2".into(),
///         class_type: "必修".into(),
///         score: 80,
///         credit: 2.0,
///         term: "2024-2025-2".into(),
///     },
/// ];
/// let terms: Vec<Term> = vec!["2024-2025-1".into(), "2024-2025-2".into()];
/// let gpa = calc_with_terms(grades, terms);
/// assert_eq!(gpa, 3.6);
/// ```
///
pub fn calc_with_terms(grades: Vec<Course>, terms: Vec<Term>) -> f32 {
    let mut total_gpa = 0.0;
    let mut total_credit = 0.0;
    if !grades.is_empty() {
        println!(
            "{:<8}{:<8}{:<5}{:<30} ",
            "Credit", "Score", "GPA", "CourseName"
        );
    }
    grades
        .iter()
        .filter(|course| course.class_type != "任选")
        .filter(|course| terms.is_empty() || terms.iter().any(|term| term == &course.term))
        .for_each(|course| {
            let gpa = get_gpa(course.score);
            println!(
                "{:<8}{:<8}{:<5.1}{:<30} ",
                course.credit, course.score, gpa, course.name
            );
            total_gpa += gpa * course.credit;
            total_credit += course.credit;
        });
    let gpa = total_gpa / total_credit;
    println!("Total GPA: {:.4}", total_gpa);
    println!("Total Credit: {:.4}", total_credit);
    println!("Your GPA is: {:.4}", gpa);
    gpa
}
