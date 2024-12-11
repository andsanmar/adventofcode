use std::collections::HashMap;

use aoc::input_file;

#[derive(Debug)]
struct Data {
    stones: HashMap<usize, usize>,
}

impl std::str::FromStr for Data {
    type Err = std::io::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut stones = HashMap::new();
        for stone_n in input.split_whitespace() {
            let stone = stone_n.parse().unwrap();
            *stones.entry(stone).or_default() += 1;
        }

        Ok(Data { stones })
    }
}

fn evolve_stone(stones: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_stones = HashMap::new();
    for (stone, count) in stones {
        if *stone == 0 {
            *new_stones.entry(1).or_default() += count;
        } else {
            let length_num = stone.ilog10() + 1 % 2;
            if length_num % 2 == 0 {
                *new_stones
                    .entry(*stone / 10_usize.pow(length_num / 2))
                    .or_default() += count;
                *new_stones
                    .entry(*stone % 10_usize.pow(length_num / 2))
                    .or_default() += count;
            } else {
                *new_stones.entry(*stone * 2024).or_default() += count;
            }
        }
    }
    new_stones
}

fn stars(Data { mut stones }: Data) {
    for n in 0..75 {
        if n == 25 {
            println!("Star1: {}", stones.values().sum::<usize>())
        };
        stones = evolve_stone(&stones);
    }
    println!("Star2: {}", stones.values().sum::<usize>());
}

fn main() -> Result<(), std::io::Error> {
    let input_raw = std::fs::read_to_string(input_file(file!()))?;
    let parsed_data: Data = input_raw.parse()?;
    stars(parsed_data);
    Ok(())
}
