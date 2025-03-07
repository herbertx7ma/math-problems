use std::cmp::Ordering;

fn solve_math_problem(a: i32, b: i32) -> i32 {
    match a.cmp(&b) {
        Ordering::Less => a * 2,
        Ordering::Equal => a + b,
        Ordering::Greater => a / 2,
    }
}
