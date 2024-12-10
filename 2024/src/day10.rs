// Import required standard library modules and custom aoc module
use std::collections::{HashMap, HashSet};

use aoc::input_file;

// Type alias for 2D coordinates represented as (row, column)
type Coord = (usize, usize);

/// Data structure representing a height map grid
/// Contains heights at each coordinate and ground level coordinates
#[derive(Debug)]
struct Data {
    /// Maps coordinates to their height values (0-9)
    heights: HashMap<Coord, u8>,
    /// Vector of coordinates that are at ground level (height 0)
    ground: Vec<Coord>,
}

/// Parses input string into Data structure
/// Input format is a grid of digits 0-9 representing heights
impl std::str::FromStr for Data {
    type Err = std::io::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut heights: HashMap<Coord, u8> = HashMap::new();
        let mut ground: Vec<Coord> = Vec::new();

        // Parse each character position in the grid
        for (row_num, line) in input.lines().enumerate() {
            for (col_num, char) in line.chars().enumerate() {
                if char.is_digit(10) {
                    let height = char.to_digit(10).unwrap() as u8;
                    heights.insert((row_num, col_num), height);
                    if height == 0 {
                        ground.push((row_num, col_num))
                    }
                }
            }
        }
        Ok(Data { heights, ground })
    }
}

/// Recursively finds paths from a starting coordinate to peak height (9)
/// Returns number of valid paths found
///
/// # Arguments
/// * `coord` - Starting coordinate (x,y)
/// * `height` - Current height being searched for
/// * `heights` - Reference to height map
/// * `tops_reached` - Set to track unique peak coordinates reached
fn find_path(
    (x, y): Coord,
    height: u8,
    heights: &HashMap<Coord, u8>,
    tops_reached: &mut HashSet<Coord>,
) -> u64 {
    // Check if coordinate exists in height map
    if let Some(&h) = heights.get(&(x, y)) {
        // Return 0 if height doesn't match what we're looking for
        if h != height {
            return 0;
        }

        // If we've reached a peak, record it and return 1 path found
        if height == 9 {
            tops_reached.insert((x, y));
            return 1;
        }

        // Recursively check all adjacent coordinates for next height
        return find_path((x + 1, y), height + 1, heights, tops_reached)
            + find_path((x - 1, y), height + 1, heights, tops_reached)
            + find_path((x, y + 1), height + 1, heights, tops_reached)
            + find_path((x, y - 1), height + 1, heights, tops_reached);
    }
    0
}

/// Solves both parts of the puzzle:
/// Star1: Number of unique peaks reachable from ground level
/// Star2: Total number of valid paths from ground to peaks
fn stars(Data { heights, ground }: Data) {
    let mut tops_reached = HashSet::new();
    let (r1, r2) = ground.iter().fold((0, 0), |(star1r, star2r), coord| {
        let r2 = find_path(*coord, 0, &heights, &mut tops_reached);
        let r = tops_reached.len();
        tops_reached.clear();
        (star1r + r, star2r + r2)
    });
    println!("Star1 {r1}"); // Print number of unique peaks reached
    println!("Star2 {r2}"); // Print total number of valid paths
}

/// Main function: Read input file and solve both parts
fn main() -> Result<(), std::io::Error> {
    let input_raw = std::fs::read_to_string(input_file(file!()))?;
    let parsed_data: Data = input_raw.parse()?;
    stars(parsed_data);
    Ok(())
}
