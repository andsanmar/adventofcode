// Import required standard library modules and custom aoc module
use std::collections::{HashMap, HashSet};

use aoc::input_file;

// Type alias for 2D coordinates represented as (row, column)
type Coord = (usize, usize);

/// Represents the garden layout with plants at different coordinates
#[derive(Debug)]
struct Data {
    /// Maps plant types (chars) to sets of coordinates where they are located
    plants: HashMap<char, HashSet<Coord>>,
}

impl std::str::FromStr for Data {
    type Err = std::io::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut plants: HashMap<char, HashSet<Coord>> = HashMap::new();

        // Parse each character position in the grid to build plant locations
        for (row_num, line) in input.lines().enumerate() {
            for (col_num, plant_type) in line.chars().enumerate() {
                plants
                    .entry(plant_type)
                    .or_default()
                    .insert((row_num, col_num));
            }
        }
        Ok(Data { plants })
    }
}

/// Recursively calculates the perimeter of a connected group of plants
///
/// # Arguments
/// * `visited` - Set of coordinates already processed
/// * `current_pos` - Current position being checked
/// * `plant_group` - Set of coordinates containing plants of the same type
///
/// # Returns
/// Size of the perimeter for this connected group
fn get_connected_plant(
    visited: &mut HashSet<Coord>,
    current_pos: Coord,
    plant_group: &HashSet<Coord>,
) -> usize {
    let mut perimeter_size = 0;

    // Check all adjacent positions (up, down, left, right)
    for adjacent_pos in [
        (current_pos.0 + 1, current_pos.1),
        (current_pos.0 - 1, current_pos.1),
        (current_pos.0, current_pos.1 + 1),
        (current_pos.0, current_pos.1 - 1),
    ] {
        if plant_group.contains(&adjacent_pos) {
            if !visited.contains(&adjacent_pos) {
                visited.insert(adjacent_pos);
                perimeter_size += get_connected_plant(visited, adjacent_pos, plant_group);
            }
        } else {
            perimeter_size += 1
        }
    }

    perimeter_size
}

/// Calculates total area and perimeter products for all plant groups
///
/// # Arguments
/// * `plant_group` - Set of coordinates for a specific plant type
///
/// # Returns
/// Sum of (perimeter * area) for all connected groups
/// Calculates total area and perimeter products for all plant groups
/// Returns the sum of (perimeter * area) for all connected groups
fn all_areas_and_sizes(plant_group: &mut HashSet<Coord>) -> usize {
    // Initialize total score accumulator
    let mut total_score = 0;
    // Process each connected group until no plants remain
    while !plant_group.is_empty() {
        // Track visited positions for current connected group
        let mut visited_positions = HashSet::new();
        // Get first unprocessed plant position as starting point
        let start_pos = plant_group.iter().next().unwrap();
        // Mark starting position as visited
        visited_positions.insert(*start_pos);
        // Calculate perimeter of connected group starting from this position
        let perimeter = get_connected_plant(&mut visited_positions, *start_pos, plant_group);
        // Debug output showing perimeter, visited positions and start position
        println!("Planted: {perimeter:?} {visited_positions:?} {start_pos:?}");
        // Add score for this group (perimeter * area) to total
        total_score += perimeter * visited_positions.len();
        // Remove processed positions from plant group for next iteration
        *plant_group = plant_group
            .difference(&visited_positions)
            .map(|x| *x)
            .collect();
    }
    total_score
}

/// Processes all plant types and calculates final score
fn stars(Data { mut plants }: Data) {
    println!("{plants:?}");
    println!(
        "Star1 {}",
        plants.values_mut().map(all_areas_and_sizes).sum::<usize>()
    );
}

/// Main function: Read input file and solve both parts
fn main() -> Result<(), std::io::Error> {
    let input_raw = std::fs::read_to_string(input_file(file!()))?;
    let parsed_data: Data = input_raw.parse()?;
    stars(parsed_data);
    Ok(())
}
