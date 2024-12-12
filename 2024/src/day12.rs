// Import required standard library modules and custom aoc module for file handling
use std::collections::{HashMap, HashSet};

use aoc::input_file;

// Type alias for 2D coordinates represented as (row, column) tuple
type Coord = (usize, usize);

/// Represents the garden layout with plants at different coordinates in a 2D grid
#[derive(Debug)]
struct Data {
    /// Maps plant types (chars) to sets of coordinates where they are located
    /// Key: Plant type character
    /// Value: Set of (row,col) coordinates where that plant type exists
    plants: HashMap<char, HashSet<Coord>>,
}

impl std::str::FromStr for Data {
    type Err = std::io::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut plant_locations: HashMap<char, HashSet<Coord>> = HashMap::new();

        // Parse input grid - each character represents a plant type at that position
        for (row_idx, line) in input.lines().enumerate() {
            for (col_idx, plant_type) in line.chars().enumerate() {
                plant_locations
                    .entry(plant_type)
                    .or_default()
                    .insert((row_idx, col_idx));
            }
        }
        Ok(Data {
            plants: plant_locations,
        })
    }
}

/// Add corner points to the appropriate corner sets based on plant group layout
///
/// # Arguments
/// * `coord` - Current coordinate being checked
/// * `plant_group` - Set of coordinates containing plants of same type
/// * `corners` - Tuple of 4 corner sets (top-right, top-left, bottom-right, bottom-left)
fn add_corner_points(
    (x, y): Coord,
    plant_group: &HashSet<Coord>,
    corners: &mut (
        HashSet<Coord>,
        HashSet<Coord>,
        HashSet<Coord>,
        HashSet<Coord>,
    ),
) {
    // Check for external corners (no plants in adjacent positions)
    if !plant_group.contains(&(x + 1, y)) && !plant_group.contains(&(x, y + 1)) {
        corners.0.insert((x, y)); // Top-right corner
    }
    if !plant_group.contains(&(x - 1, y)) && !plant_group.contains(&(x, y + 1)) {
        corners.1.insert((x, y)); // Top-left corner
    }
    if !plant_group.contains(&(x + 1, y)) && !plant_group.contains(&(x, y - 1)) {
        corners.2.insert((x, y)); // Bottom-right corner
    }
    if !plant_group.contains(&(x - 1, y)) && !plant_group.contains(&(x, y - 1)) {
        corners.3.insert((x, y)); // Bottom-left corner
    }

    // Check for internal corners (plants adjacent but diagonal missing)
    if plant_group.contains(&(x + 1, y))
        && plant_group.contains(&(x, y + 1))
        && !plant_group.contains(&(x + 1, y + 1))
    {
        corners.0.insert((x, y)); // Top-right internal corner
    }
    if plant_group.contains(&(x - 1, y))
        && plant_group.contains(&(x, y + 1))
        && !plant_group.contains(&(x - 1, y + 1))
    {
        corners.1.insert((x, y)); // Top-left internal corner
    }
    if plant_group.contains(&(x + 1, y))
        && plant_group.contains(&(x, y - 1))
        && !plant_group.contains(&(x + 1, y - 1))
    {
        corners.2.insert((x, y)); // Bottom-right internal corner
    }
    if plant_group.contains(&(x - 1, y))
        && plant_group.contains(&(x, y - 1))
        && !plant_group.contains(&(x - 1, y - 1))
    {
        corners.3.insert((x, y)); // Bottom-left internal corner
    }
}

/// Recursively calculates the perimeter of a connected group of plants
///
/// # Arguments
/// * `visited` - Set of coordinates already processed in current group
/// * `corners` - Tuple of sets tracking corner points
/// * `current_pos` - Current coordinate being processed
/// * `plant_group` - Set of all coordinates containing this plant type
///
/// # Returns
/// Tuple containing:
/// - Size of the perimeter for this connected group
/// - Number of corner points (currently unused)
fn get_connected_plant(
    visited: &mut HashSet<Coord>,
    corners: &mut (
        HashSet<Coord>,
        HashSet<Coord>,
        HashSet<Coord>,
        HashSet<Coord>,
    ),
    current_pos: Coord,
    plant_group: &HashSet<Coord>,
) -> (usize, usize) {
    let mut perimeter_count = 0;

    // Check all orthogonally adjacent positions
    for adjacent_pos in [
        (current_pos.0 + 1, current_pos.1), // Right
        (current_pos.0 - 1, current_pos.1), // Left
        (current_pos.0, current_pos.1 + 1), // Up
        (current_pos.0, current_pos.1 - 1), // Down
    ] {
        if plant_group.contains(&adjacent_pos) {
            if !visited.contains(&adjacent_pos) {
                visited.insert(adjacent_pos);
                let (sub_perimeter, _) =
                    get_connected_plant(visited, corners, adjacent_pos, plant_group);
                perimeter_count += sub_perimeter;
            }
        } else {
            perimeter_count += 1
        }
        add_corner_points(current_pos, plant_group, corners);
    }

    (perimeter_count, 0)
}

/// Calculates total area and perimeter products for all connected plant groups
///
/// # Arguments
/// * `plant_group` - Set of coordinates for a specific plant type
///
/// # Returns
/// Tuple containing:
/// - Sum of (perimeter * area) for all connected groups
/// - Sum of (corner_count * area) for all connected groups
fn sum_scores(plant_group: &mut HashSet<Coord>) -> (usize, usize) {
    let mut total_perimeter_score = 0;
    let mut total_corner_score = 0;

    // Process each connected group until no unprocessed plants remain
    while !plant_group.is_empty() {
        let mut visited_positions = HashSet::new();
        let mut corner_sets = (
            HashSet::new(), // Top-right corners
            HashSet::new(), // Top-left corners
            HashSet::new(), // Bottom-right corners
            HashSet::new(), // Bottom-left corners
        );

        // Start with first unprocessed plant position
        let start_position = plant_group.iter().next().unwrap();
        visited_positions.insert(*start_position);

        // Calculate metrics for this connected group
        let (perimeter, _) = get_connected_plant(
            &mut visited_positions,
            &mut corner_sets,
            *start_position,
            plant_group,
        );
        let corner_count =
            corner_sets.0.len() + corner_sets.1.len() + corner_sets.2.len() + corner_sets.3.len();
        let group_area = visited_positions.len();

        // Update total scores
        total_perimeter_score += perimeter * group_area;
        total_corner_score += corner_count * group_area;

        // Remove processed positions from remaining plants
        *plant_group = plant_group
            .difference(&visited_positions)
            .copied()
            .collect();
    }
    (total_perimeter_score, total_corner_score)
}

/// Processes all plant types and calculates final scores for both parts
fn stars(Data { mut plants }: Data) {
    let final_scores = plants.values_mut().map(sum_scores).fold(
        (0, 0),
        |(sum_perimeter, sum_corners), (perimeter, corners)| {
            (sum_perimeter + perimeter, sum_corners + corners)
        },
    );

    println!("Star1 {:?}", final_scores.0);
    println!("Star2 {:?}", final_scores.1);
}

/// Main function: Read input file and solve both puzzle parts
fn main() -> Result<(), std::io::Error> {
    let input_raw = std::fs::read_to_string(input_file(file!()))?;
    let parsed_data: Data = input_raw.parse()?;
    stars(parsed_data);
    Ok(())
}
