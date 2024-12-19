use std::collections::HashMap;

use aoc::input_file;

#[derive(Debug)]
struct Data {
    towels: Vec<String>,
    combinations: Vec<String>,
}

impl std::str::FromStr for Data {
    type Err = std::io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut it = input.split("\n\n");
        Ok(Data {
            towels: it
                .next()
                .unwrap()
                .split(", ")
                .map(|s| s.to_string())
                .collect(),
            combinations: it.next().unwrap().lines().map(|s| s.to_string()).collect(),
        })
    }
}

fn check_valid<'a>(
    s: &'a str,
    available: &Vec<String>,
    cache: &mut HashMap<&'a str, bool>,
) -> bool {
    if let Some(r) = cache.get(s) {
        return *r;
    }
    if s.is_empty() {
        return true;
    }
    let mut r = false;
    for start in available {
        if s.starts_with(start) {
            let rest = &s[start.len()..];
            r |= check_valid(rest, available, cache);
        }
    }
    cache.insert(s, r);
    return r;
}

fn check_valid2<'a>(
    s: &'a str,
    available: &Vec<String>,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(r) = cache.get(s) {
        return *r;
    }
    if s.is_empty() {
        return 1;
    }
    let mut r = 0;
    for start in available {
        if s.starts_with(start) {
            let rest = &s[start.len()..];
            r += check_valid2(rest, available, cache);
        }
    }
    cache.insert(s, r);
    return r;
}

fn stars(
    Data {
        towels,
        combinations,
    }: Data,
) {
    let mut cache = HashMap::new();
    println!(
        "Star1: {:?}",
        combinations
            .iter()
            .filter(|s| check_valid(s, &towels, &mut cache))
            .count()
    );

    let mut cache2 = HashMap::new();
    println!(
        "Star2: {:?}",
        combinations
            .iter()
            .map(|s| check_valid2(s, &towels, &mut cache2))
            .sum::<usize>()
    );
}

fn main() -> Result<(), std::io::Error> {
    let input_raw = std::fs::read_to_string(input_file(file!()))?;
    let parsed_data: Data = input_raw.parse()?;
    stars(parsed_data);
    Ok(())
}
