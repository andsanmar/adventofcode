use std::collections::HashSet;

use aoc::input_file;

// Represents a 2D coordinate with x,y positions
type Coord = (isize, isize);

#[derive(Debug)]
struct Data {
    fallen: Vec<Coord>,
}

impl std::str::FromStr for Data {
    type Err = std::io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut fallen = input
            .lines()
            .filter_map(|line| {
                let mut it = line.split(",").filter_map(|n| n.parse().ok());
                Some((it.next()?, it.next()?))
            })
            .collect();
        Ok(Data { fallen })
    }
}

const SIZE_GRID: isize = 6;
const FALLEN: usize = 12;

fn stars(Data { fallen }: Data) {
    let mut to_check = HashSet::new();
    to_check.insert((0, 0));

    let mut iteration = 0;
    'outer: while !to_check.is_empty() {
        let mut new_to_check = HashSet::new();
        for (x, y) in to_check.iter() {
            if (*x, *y) == (SIZE_GRID, SIZE_GRID) {
                break 'outer;
            }
            // Check all 4 adjacent positions
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let next = (x + dx, y + dy);
                // Only consider positions within bounds and not fallen
                if next.0 >= 0
                    && next.0 <= SIZE_GRID
                    && next.1 >= 0
                    && next.1 <= SIZE_GRID
                    && !fallen[..FALLEN].contains(&next)
                {
                    new_to_check.insert(next);
                }
            }
        }
        to_check = new_to_check;
        iteration += 1;
    }

    println!("Star1: {:?}", iteration);
}

fn main() -> Result<(), std::io::Error> {
    let input_raw = std::fs::read_to_string(input_file(file!()))?;
    let parsed_data: Data = input_raw.parse()?;
    stars(parsed_data);
    Ok(())
}
