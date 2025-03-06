use std::num::{BigInt, BigUint};

fn solve_math_problem(problem: String) -> Option<i32> {
    // Parse the problem string into a number and an operation
    let mut parts = problem.split(' ');
    let num1 = parts.next().unwrap().parse::<BigInt>();
    let op = parts.next().unwrap();
    let num2 = parts.next().unwrap().parse::<BigUint>();

    // Perform the operation on the numbers and return the result
    match op {
        "+" => Some(num1 + num2),
        "-" => Some(num1 - num2),
        "*" => Some(num1 * num2),
        "/" => Some(num1 / num2),
        _ => None, // Unsupported operation
    }
}
