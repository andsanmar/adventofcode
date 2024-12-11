use std::collections::HashMap;

use aoc::input_file;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Number {
    Pure(u64),
    Splitted(Vec<Number>),
    // Cached(Number, u64),
}

#[derive(Debug)]
struct Data {
    stones: Number,
}

impl std::str::FromStr for Data {
    type Err = std::io::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Data {
            stones: Number::Splitted(
                input
                    .split_whitespace()
                    .map(|line| Number::Pure(line.parse().unwrap()))
                    .collect(),
            ),
        })
    }
}

fn evolve_stone(stone: &Number, cache: &mut HashMap<Number, Number>) -> Number {
    if let Some(r) = cache.get(stone) {
        return r.clone();
    }
    let r: Number = {
        match stone {
            Number::Pure(0) => Number::Pure(1),
            Number::Pure(stone_n) => {
                let length_num = stone_n.ilog10() + 1 % 2;
                if length_num % 2 == 0 {
                    Number::Splitted(vec![
                        Number::Pure(stone_n / 10_u64.pow(length_num / 2)),
                        Number::Pure(stone_n % 10_u64.pow(length_num / 2)),
                    ])
                } else {
                    Number::Pure(stone_n * 2024)
                }
            }
            Number::Splitted(v) => {
                Number::Splitted(v.iter().map(|s| evolve_stone(s, cache)).collect())
            }
        }
    };
    cache.insert(stone.clone(), r.clone());
    r
}

fn count_stones(stones: &Number) -> usize {
    match stones {
        Number::Pure(_) => 1,
        Number::Splitted(v) => v.iter().map(count_stones).sum(),
    }
}

fn stars(Data { mut stones }: Data) {
    let mut cache: HashMap<Number, Number> = HashMap::new();

    for n in 0..75 {
        // println!("{cache:?}");
        if n == 25 {
            println!("Star1: {}", count_stones(&stones))
        };
        stones = evolve_stone(&stones, &mut cache);
    }
    println!("Star2: {}", count_stones(&stones));
}

fn main() -> Result<(), std::io::Error> {
    let input_raw = std::fs::read_to_string(input_file(file!()))?;
    let parsed_data: Data = input_raw.parse()?;
    stars(parsed_data);
    Ok(())
}
