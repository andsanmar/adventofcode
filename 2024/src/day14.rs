use aoc::input_file;

// Represents a 2D coordinate with x,y positions
type Coord = (i64, i64);

// Maximum grid dimensions for the robot movement space
const MAX: Coord = (103, 101);
// Test grid dimensions
// const MAX: Coord = (7, 11);

// Holds the robot positions and velocities
#[derive(Debug)]
struct Data {
    // Vec of (position, velocity) tuples for each robot
    robots: Vec<(Coord, Coord)>,
}

impl std::str::FromStr for Data {
    type Err = std::io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Parse input lines into robot position and velocity data
        let robots = input
            .lines()
            .map(|block| {
                let numbers: Vec<i64> = block
                    .split(|c| c == ' ' || c == ',' || c == '=')
                    .filter_map(|number_str| number_str.parse::<i64>().ok())
                    .collect();
                // Format: ((y_pos, x_pos), (y_vel, x_vel))
                ((numbers[1], numbers[0]), (numbers[3], numbers[2]))
            })
            .collect();
        Ok(Data { robots })
    }
}

// Displays robot positions and checks for message formation
fn display_robots(robot_positions: Vec<Coord>) -> bool {
    // Create grid with padding around MAX dimensions
    let mut display_grid = vec![vec!['.'; 10 + MAX.0 as usize]; 10 + MAX.1 as usize];

    // Mark robot positions on grid
    for (row, col) in robot_positions {
        display_grid[row as usize][col as usize] = '#';
    }

    // Check for message formation - continuous '#' in specific range
    for grid_line in &display_grid {
        if (43..67).all(|column| grid_line[column] == '#') {
            // Print grid when message is found
            for line in display_grid {
                println!("{}", line.iter().collect::<String>());
            }
            return true;
        }
    }
    false
}

// Calculate robot positions after given number of iterations
fn after_iteration(robots: &Vec<(Coord, Coord)>, iteration_count: i64) -> Vec<Coord> {
    robots
        .iter()
        .map(|((pos_y, pos_x), (vel_y, vel_x))| {
            (
                (pos_y + vel_y * iteration_count).rem_euclid(MAX.0),
                (pos_x + vel_x * iteration_count).rem_euclid(MAX.1),
            )
        })
        .collect()
}

fn stars(Data { robots }: Data) {
    // Calculate positions after 100 iterations
    let positions_at_100: Vec<Coord> = after_iteration(&robots, 100);

    // Star 1: Calculate product of robots in each quadrant
    println!(
        "Star1: {:?}",
        positions_at_100
            .iter()
            .fold([0; 4], |mut quadrant_counts, (y, x)| {
                if *y < MAX.0 / 2 && *x < MAX.1 / 2 {
                    quadrant_counts[0] += 1; // Top-left quadrant
                } else if *y > MAX.0 / 2 && *x < MAX.1 / 2 {
                    quadrant_counts[1] += 1; // Bottom-left quadrant
                } else if *y < MAX.0 / 2 && *x > MAX.1 / 2 {
                    quadrant_counts[2] += 1; // Top-right quadrant
                } else if *y > MAX.0 / 2 && *x > MAX.1 / 2 {
                    quadrant_counts[3] += 1; // Bottom-right quadrant
                }
                quadrant_counts
            })
            .iter()
            .product::<usize>()
    );

    // Star 2: Find iteration when message appears
    for current_iteration in 0.. {
        let message_found = display_robots(after_iteration(&robots, current_iteration));
        if message_found {
            println!("Star2: {:?}", current_iteration);
            break;
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let input_raw = std::fs::read_to_string(input_file(file!()))?;
    let parsed_data: Data = input_raw.parse()?;
    stars(parsed_data);
    Ok(())
}
