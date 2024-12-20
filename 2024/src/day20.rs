use std::{
    collections::{HashMap, HashSet},
    i64::MAX,
};

use aoc::input_file;

type Coord = (isize, isize);

#[derive(Debug)]
struct Data {
    walls: HashSet<Coord>,
    start: Coord,
    end: Coord,
}

impl std::str::FromStr for Data {
    type Err = std::io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut walls = HashSet::new();
        let mut start = None;
        let mut end = None;
        for (row_n, line) in input.lines().enumerate() {
            for (column_n, c) in line.chars().enumerate() {
                let coord = (column_n as isize, row_n as isize);
                match c {
                    '#' => {
                        walls.insert(coord);
                    }
                    'E' => end = Some(coord),
                    'S' => start = Some(coord),
                    _ => {}
                }
            }
        }
        Ok(Data {
            walls,
            start: start.unwrap(),
            end: end.unwrap(),
        })
    }
}

fn get_diffs(steps: &HashMap<Coord, i64>) -> HashMap<i64, Vec<Coord>> {
    let mut r = HashMap::new();
    for ((x, y), from_steps) in steps {
        for (dx, dy) in [(0, 2), (0, -2), (2, 0), (-2, 0)] {
            if let Some(to_steps) = steps.get(&(x + dx, y + dy)) {
                let diff = to_steps - from_steps - 2;
                r.entry(diff).or_insert_with(Vec::new).push((*x, *y));
            }
        }
    }
    r
}

fn get_diffs2(steps: &HashMap<Coord, i64>) -> HashMap<i64, Vec<Coord>> {
    let mut r = HashMap::new();
    let mut potential_adj = HashSet::new();
    for it in 2..=20 {
        for x in 0..it {
            let y: i32 = it - x;
            potential_adj.insert((x as isize, y as isize));
            potential_adj.insert((-x as isize, y as isize));
            potential_adj.insert((x as isize, -y as isize));
            potential_adj.insert((-x as isize, -y as isize));
        }
    }
    for ((x, y), from_steps) in steps {
        for (dx, dy) in &potential_adj {
            if let Some(to_steps) = steps.get(&(x + dx, y + dy)) {
                let diff = to_steps - from_steps - (dx.abs() + dy.abs()) as i64;
                r.entry(diff).or_insert_with(Vec::new).push((*x, *y));
            }
        }
    }
    r
}

fn solve(walls: &HashSet<Coord>, start: Coord) -> HashMap<Coord, i64> {
    let mut r = HashMap::new();
    let mut queue = vec![(start, 0)];
    let mut visited = HashSet::new();
    while let Some((coord, steps)) = queue.pop() {
        r.insert(coord, steps);
        // if coord == end {
        //     break;
        //     return steps;
        // }
        if visited.contains(&coord) {
            continue;
        }
        visited.insert(coord);
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next = (coord.0 + dx, coord.1 + dy);
            if walls.contains(&next) || visited.contains(&next) {
                continue;
            }
            queue.push((next, steps + 1));
        }
    }
    return r;
}

const SAVES: i64 = 100;
const SAVES2: i64 = 100;

fn stars(Data { walls, start, end }: Data) {
    let steps = solve(&walls.clone(), start);
    let diffs: HashMap<i64, Vec<(isize, isize)>> = get_diffs(&steps);
    // println!("Star1: {:?}", start);
    // println!("Star1: {:?}", steps);
    // println!("Star1: {:?}", diffs);
    let mut r = 0;
    for k in diffs.keys() {
        if k >= &SAVES {
            // println!("Star1: {:?} {:?}", k, diffs[k].len());
            r += diffs[k].len();
        }
    }
    println!("Star1: {:?}", r);

    let diffs2: HashMap<i64, Vec<(isize, isize)>> = get_diffs2(&steps);
    let mut r2 = 0;
    for k in diffs2.keys() {
        if k >= &SAVES2 {
            // println!("Star1: {:?} {:?}", k, diffs[k].len());
            r2 += diffs2[k].len();
        }
    }
    println!("Star2: {:?}", r2);
}

fn main() -> Result<(), std::io::Error> {
    let input_raw = std::fs::read_to_string(input_file(file!()))?;
    let parsed_data: Data = input_raw.parse()?;
    stars(parsed_data);
    Ok(())
}
