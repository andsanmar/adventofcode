use std::collections::HashMap;

use aoc::input_file;

#[derive(Debug, Hash, PartialEq, Eq)]
enum COLOR {
    RED,
    GREEN,
    BLUE,
}

#[derive(Debug)]
struct Data(Vec<(u32, Vec<HashMap<COLOR, usize>>)>);

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
                            .expect("Missing ID number")
                            .split_whitespace()
                            .last()
                            .expect("Game does not have a number")
                            .parse()
                            .unwrap(),
                        parts
                            .next()
                            .expect("Missing operands")
                            .split(";")
                            .map(|subsets| {
                                subsets
                                    .split(",")
                                    .map(|n_balls| {
                                        let mut it = n_balls.split_whitespace();
                                        let n = it
                                            .next()
                                            .expect("missing number of balls")
                                            .parse()
                                            .expect("cannot parse number of balls");
                                        let color = match it.next().expect("missing color") {
                                            "red" => COLOR::RED,
                                            "green" => COLOR::GREEN,
                                            "blue" => COLOR::BLUE,
                                            _ => panic!("unknown color"),
                                        };
                                        (color, n)
                                    })
                                    .collect()
                            })
                            .collect(),
                    )
                })
                .collect(),
        ))
    }
}

fn stars(Data(v): &Data) {
    let c = v
        .iter()
        .filter_map(|(id, game)| {
            game.iter()
                .all(|subset| {
                    subset.iter().all(|(color, n)| match color {
                        COLOR::RED => *n <= 12,
                        COLOR::GREEN => *n <= 13,
                        COLOR::BLUE => *n <= 14,
                    })
                })
                .then_some(id)
        })
        .collect::<Vec<&u32>>();
    println!("Part 1: {c:?}");

    let c2 = v.iter().map(|(_, game)| {
        let mut m = HashMap::new();
        game.iter().for_each(|subset| {
            subset.iter().for_each(|(color, n)| {
                m.entry(color)
                    .and_modify(|e| {
                        if *e < *n {
                            *e = *n
                        }
                    })
                    .or_insert(*n);
            })
        });
        m.values().product::<usize>()
    });
    println!("Part 2: {}", c2.sum::<usize>());
}

// Main function: Read input file and solve both parts
fn main() -> Result<(), std::io::Error> {
    // Read input file and parse into Data structure
    let input_raw = std::fs::read_to_string(input_file(file!()))?;
    let parsed_data: Data = input_raw.parse()?;

    stars(&parsed_data);
    Ok(())
}
