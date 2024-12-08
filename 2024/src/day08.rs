// Import required standard library modules and custom aoc module
use std::{collections::HashMap, collections::HashSet};

use aoc::input_file;

// Type alias for 2D coordinates represented as (row, column)
type Coord = (isize, isize);

// Data structure to store antenna frequencies and grid bounds
#[derive(Debug)]
struct Data {
    // Maps antenna frequencies (chars) to their coordinates in the grid
    antenna_freq: HashMap<char, Vec<Coord>>,
    // Grid dimensions as (num_rows, num_cols)
    bounds: Coord,
}

// Implementation to parse input string into Data structure
impl std::str::FromStr for Data {
    type Err = std::io::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut antenna_freq: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
        // Get grid dimensions from input
        let bounds = (
            input.lines().count() as isize,
            input.lines().next().expect("missing first line").len() as isize,
        );
        // Parse each character position in the grid
        for (row_num, line) in input.lines().enumerate() {
            for (col_num, char) in line.chars().enumerate() {
                if char.is_alphanumeric() {
                    antenna_freq
                        .entry(char)
                        .or_insert(Vec::new())
                        .push((row_num as isize, col_num as isize));
                }
            }
        }

        Ok(Data {
            antenna_freq,
            bounds,
        })
    }
}

// Calculate antinode positions for part 1 using reflection points
fn calculate_antinodes(antenna_coords: Vec<Coord>) -> HashSet<Coord> {
    let mut antinode_positions = HashSet::new();
    for i in 0..antenna_coords.len() {
        for j in i..antenna_coords.len() {
            if i == j {
                continue;
            }
            // Calculate reflection points for each antenna pair
            antinode_positions.insert((
                2 * antenna_coords[i].0 - antenna_coords[j].0,
                2 * antenna_coords[i].1 - antenna_coords[j].1,
            ));
            antinode_positions.insert((
                2 * antenna_coords[j].0 - antenna_coords[i].0,
                2 * antenna_coords[j].1 - antenna_coords[i].1,
            ));
        }
    }
    antinode_positions
}

// Calculate antinode positions for part 2 using vector extension
fn calculate_antinodes2(antenna_coords: Vec<Coord>, grid_bounds: Coord) -> HashSet<Coord> {
    let mut antinode_positions = HashSet::new();
    for i in 0..antenna_coords.len() {
        for j in i..antenna_coords.len() {
            if i == j {
                continue;
            }
            // Calculate vector between antenna pairs
            let (delta_x, delta_y) = (
                antenna_coords[i].0 - antenna_coords[j].0,
                antenna_coords[i].1 - antenna_coords[j].1,
            );

            // Extend vector in positive direction
            let mut multiplier: isize = 1;
            while (antenna_coords[i].0 + multiplier * delta_x >= 0
                && antenna_coords[i].0 + multiplier * delta_x < grid_bounds.0
                && antenna_coords[i].1 + multiplier * delta_y >= 0
                && antenna_coords[i].1 + multiplier * delta_y < grid_bounds.1)
            {
                antinode_positions.insert((
                    antenna_coords[i].0 + multiplier * delta_x,
                    antenna_coords[i].1 + multiplier * delta_y,
                ));
                multiplier += 1;
            }

            // Extend vector in negative direction
            multiplier = 1;
            while (antenna_coords[j].0 - multiplier * delta_x >= 0
                && antenna_coords[j].0 - multiplier * delta_x < grid_bounds.0
                && antenna_coords[j].1 - multiplier * delta_y >= 0
                && antenna_coords[j].1 - multiplier * delta_y < grid_bounds.1)
            {
                antinode_positions.insert((
                    antenna_coords[j].0 - multiplier * delta_x,
                    antenna_coords[j].1 - multiplier * delta_y,
                ));
                multiplier += 1;
            }
        }
    }
    antinode_positions
}

// Solve both parts of the puzzle
fn stars(
    Data {
        antenna_freq,
        bounds: grid_bounds,
    }: Data,
) {
    // Get set of all antenna positions
    let antenna_positions: HashSet<Coord> = antenna_freq.values().flatten().copied().collect();

    // Part 1: Calculate antinodes using reflection points
    let mut part1_antinodes: HashSet<Coord> = HashSet::new();
    for antenna_group in antenna_freq.values() {
        part1_antinodes = part1_antinodes
            .union(&calculate_antinodes(antenna_group.clone()))
            .filter_map(|(x, y)| {
                if *x >= 0 && *x < grid_bounds.0 && *y >= 0 && *y < grid_bounds.1 {
                    Some((*x, *y))
                } else {
                    None
                }
            })
            .collect();
    }
    println!("Star1: {}", part1_antinodes.len());

    // Part 2: Calculate antinodes using vector extension
    let mut part2_antinodes: HashSet<Coord> = HashSet::new();
    for antenna_group in antenna_freq.values() {
        part2_antinodes = part2_antinodes
            .union(&calculate_antinodes2(antenna_group.clone(), grid_bounds))
            .filter_map(|(x, y)| {
                if *x >= 0 && *x < grid_bounds.0 && *y >= 0 && *y < grid_bounds.1 {
                    Some((*x, *y))
                } else {
                    None
                }
            })
            .collect();
    }
    println!(
        "Star2: {}",
        part2_antinodes.union(&antenna_positions).count()
    );
}

// Main function: Read input file and solve both parts
fn main() -> Result<(), std::io::Error> {
    // Read input file and parse into Data structure
    let input_raw = std::fs::read_to_string(input_file(file!()))?;
    let parsed_data: Data = input_raw.parse()?;
    stars(parsed_data);
    Ok(())
}
