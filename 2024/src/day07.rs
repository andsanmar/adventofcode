// Import required standard library modules and custom aoc module
use std::{cell::RefCell, collections::HashMap, collections::HashSet, rc::Rc};

use aoc::input_file;

// Data structure to hold puzzle input - each line contains a target number and list of operands
#[derive(Debug)]
struct Data(Vec<(u64, Vec<u64>)>);

// Implementation to parse input string into Data structure
impl std::str::FromStr for Data {
    type Err = std::io::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Data(
            input
                .lines()
                .map(|line| {
                    let mut parts = line.split(':');
                    (
                        parts
                            .next()
                            .expect("Missing target number")
                            .parse()
                            .unwrap(),
                        parts
                            .next()
                            .expect("Missing operands")
                            .split_whitespace()
                            .map(|num| num.parse().unwrap())
                            .collect(),
                    )
                })
                .collect(),
        ))
    }
}

// Recursively check if target number can be formed using arithmetic operations on operands
// Parameters:
// - target: The number we're trying to form
// - operands: Slice of numbers we can use
// - accumulated_sum: Current value from previous operations
// - allow_concatenation: Whether to allow digit concatenation operation
fn check_op(
    target: u64,
    operands: &[u64],
    accumulated_sum: u64,
    allow_concatenation: bool,
) -> bool {
    match operands {
        [current, remaining @ ..] => {
            check_op(
                target,
                remaining,
                accumulated_sum * *current,
                allow_concatenation,
            ) || check_op(
                target,
                remaining,
                accumulated_sum + current,
                allow_concatenation,
            ) || (allow_concatenation
                && check_op(
                    target,
                    remaining,
                    accumulated_sum * 10_u64.pow(current.ilog10() + 1) as u64 + current,
                    allow_concatenation,
                ))
        }
        [] => target == accumulated_sum,
    }
}

// Solve both parts of the puzzle
fn stars(Data(input_lines): Data) {
    let mut result = 0;

    // Part 1: Find sum of target numbers that can be formed using + and * operations
    result = input_lines
        .iter()
        .filter_map(|(target, operands)| {
            let is_possible = check_op(*target, operands, 0, false);
            if is_possible {
                Some(target)
            } else {
                None
            }
        })
        .sum();
    println!("Star1 {result}");

    // Part 2: Find sum of target numbers that can be formed using +, * and digit concatenation
    result = input_lines
        .iter()
        .filter_map(|(target, operands)| {
            let is_possible = check_op(*target, operands, 0, true);
            if is_possible {
                Some(target)
            } else {
                None
            }
        })
        .sum();
    println!("Star2 {result}");
}

// Main function: Read input file and solve both parts
fn main() -> Result<(), std::io::Error> {
    // Read input file and parse into Data structure
    let input_raw = std::fs::read_to_string(input_file(file!()))?;
    let parsed_data: Data = input_raw.parse()?;
    stars(parsed_data);
    Ok(())
}
