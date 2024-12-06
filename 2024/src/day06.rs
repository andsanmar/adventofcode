use std::{cell::RefCell, collections::HashMap, collections::HashSet, rc::Rc};

use aoc::input_file;

#[derive(Debug)]
struct Data(
    std::collections::HashMap<(isize, isize), char>,
    (isize, isize),
);

impl std::str::FromStr for Data {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let st = Rc::new(RefCell::new(None));
        let r = s
            .lines()
            .enumerate()
            .map(|(line_n, line)| {
                let st: Rc<RefCell<Option<(isize, isize)>>> = Rc::clone(&st);
                line.chars().enumerate().map(move |(column_n, c)| {
                    if c == '^' {
                        st.replace(Some((line_n as isize, column_n as isize)));
                    }
                    ((line_n as isize, column_n as isize), c)
                })
            })
            .flatten()
            .collect::<std::collections::HashMap<(isize, isize), char>>();

        Ok(Data(r, Rc::try_unwrap(st).unwrap().into_inner().unwrap()))
    }
}

fn solve(
    map: &HashMap<(isize, isize), char>,
    (mut x, mut y): (isize, isize),
) -> Option<HashSet<((isize, isize), (isize, isize))>> {
    let mut next = (-1, 0);
    let mut met: HashSet<((isize, isize), (isize, isize))> = HashSet::from([((x, y), next)]);
    let obstacle_found = |cur| match cur {
        (0, -1) => (-1, 0),
        (1, 0) => (0, -1),
        (0, 1) => (1, 0),
        (-1, 0) => (0, 1),
        _ => panic!("Unknown cur {cur:?}"),
    };

    // Navigate the map, if an obstacle is found do a clockwise turn
    while let Some(mut c) = map.get(&(x + next.0, y + next.1)) {
        while *c == '#' {
            next = obstacle_found(next);
            c = map.get(&(x + next.0, y + next.1)).unwrap();
        }
        (x, y) = (x + next.0, y + next.1);

        // Detect loops and continue otherwise
        if met.contains(&((x, y), next)) {
            // Loops are invalid paths
            return None;
        } else {
            met.insert(((x, y), next));
        }
    }
    Some(met)
}

fn stars(Data(mut map, start): Data) {
    let r = solve(&map, start).unwrap();
    let passed: HashSet<(isize, isize)> = r.iter().map(|(a, _)| *a).collect();
    println!("Star1: {}", passed.len());

    let mut c = 0;

    for k in passed {
        if k == start {
            continue;
        }
        map.insert(k, '#');
        if solve(&map, start) == None {
            c += 1;
        }
        map.insert(k, '.');
    }

    println!("Star2: {c}");
}

// Main function: Read input file and solve both parts
fn main() -> Result<(), std::io::Error> {
    // Read input file and parse into vector of integer vectors
    let input_raw = std::fs::read_to_string(input_file(file!()))?;
    let l: Data = input_raw.parse()?;
    stars(l);
    Ok(())
}
