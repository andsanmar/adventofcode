use std::collections::{HashMap, HashSet};

use aoc::input_file;

/// Represents a 2D coordinate with row and column indices
type Coord = (usize, usize);

/// Stores the puzzle input data
/// symbols: Set of symbol characters and their coordinates
/// nums: Set of numbers with their value, row, and start/end column positions
#[derive(Debug)]
struct Data {
    symbols: HashSet<(Coord, char)>,
    nums: HashSet<(u32, usize, (usize, usize))>,
}

/// Implementation to parse input string into Data structure
impl std::str::FromStr for Data {
    type Err = std::io::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut symbols = HashSet::new();
        let mut nums = HashSet::new();
        // Tracks the current number being parsed (value, row, (start_col, end_col))
        let mut current_number = (0, 0, (0, 0));

        for (row, line) in input.lines().enumerate() {
            for (col, character) in line.chars().enumerate() {
                match character {
                    digit if character.is_numeric() => {
                        if current_number.0 == 0 {
                            current_number.1 = row;
                            current_number.2 .0 = col;
                        }
                        current_number.0 = current_number.0 * 10 + digit.to_digit(10).unwrap();
                        current_number.2 .1 = col;
                    }
                    // Handle non-dot symbols
                    symbol if symbol != '.' => {
                        let _ = symbols.insert(((row, col), symbol));
                        if current_number.0 != 0 {
                            nums.insert(current_number);
                            current_number.0 = 0;
                        }
                    }
                    // Handle dots when we have a number to store
                    _ if current_number.0 != 0 => {
                        nums.insert(current_number);
                        current_number.0 = 0;
                    }
                    _ => {}
                }
            }
            // Store any number that reaches end of line
            if current_number.0 != 0 {
                nums.insert(current_number);
                current_number.0 = 0;
            }
        }
        Ok(Data { symbols, nums })
    }
}

/// Calculates solutions for both parts of the puzzle
/// Part 1: Sum of all numbers adjacent to any symbol
/// Part 2: Sum of gear ratios (product of exactly two numbers adjacent to '*')
fn stars(Data { symbols, nums }: &Data) {
    let mut adjacent_numbers: HashSet<(u32, usize, (usize, usize))> = HashSet::new();
    let mut gear_ratio_sum = 0;

    for ((symbol_row, symbol_col), symbol_char) in symbols {
        let mut current_ratio = 1;
        let mut adjacent_count = 0;

        for (number, number_row, (start_col, end_col)) in nums {
            // Check if number is adjacent to symbol (within 1 row and column)
            if (*number_row as i32 - *symbol_row as i32).abs() <= 1
                && (*symbol_col >= start_col.saturating_sub(1))
                && (*symbol_col <= end_col + 1)
            {
                adjacent_numbers.insert((*number, *number_row, (*start_col, *end_col)));
                if *symbol_char == '*' {
                    current_ratio *= *number;
                    adjacent_count += 1;
                }
            }
        }
        // Add gear ratio if at least two numbers are adjacent to '*'
        if *symbol_char == '*' && adjacent_count > 1 {
            gear_ratio_sum += current_ratio;
        }
    }
    println!(
        "Star1: {}",
        adjacent_numbers.iter().map(|n| n.0).sum::<u32>()
    );
    println!("Star2: {gear_ratio_sum}")
}

/// Main function: Read input file and solve both parts
fn main() -> Result<(), std::io::Error> {
    // Read input file and parse into Data structure
    let input_raw = std::fs::read_to_string(input_file(file!()))?;
    let parsed_data: Data = input_raw.parse()?;

    stars(&parsed_data);
    Ok(())
}
