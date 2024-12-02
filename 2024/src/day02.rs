use std::cmp::Ordering;

use aoc::input_file;

// Type alias for a vector of vectors containing integers
type Data = Vec<Vec<i32>>;

// First star solution: Check if sequences follow a strictly increasing or decreasing pattern
// with differences between consecutive elements in range [1,3] or [-3,-1] respectively
fn star1(l: &Data) {
    let r: Vec<bool> =
        l.iter()
            .map(|v| {
                // For each sequence:
                // 1. Calculate differences between consecutive elements
                // 2. Fold over differences to check if pattern is consistently increasing/decreasing
                // 3. Return true if sequence follows either pattern
                v.iter().zip(v.iter().skip(1)).map(|(a, b)| a - b).fold(
                    v[0].cmp(&v[1]),
                    |dir, diff| match dir {
                        Ordering::Greater if diff >= 1 && diff <= 3 => Ordering::Greater,
                        Ordering::Less if diff >= -3 && diff <= -1 => Ordering::Less,
                        _ => Ordering::Equal,
                    },
                ) != Ordering::Equal
            })
            .collect();
    println!("Star1: {}", r.iter().filter(|a| **a).count());
}

// Second star solution: Similar to star1 but allows skipping one element to make pattern valid
fn star2(l: &Data) {
    // Helper function that checks if a sequence follows the pattern, with option to skip one element
    // Returns true if sequence is valid after potentially skipping one element
    let solve = |v: &Vec<i32>, mut skipped: bool| {
        let current_order = v[0].cmp(&v[1]);
        let mut last = &v[0];
        for e in v.iter().skip(1) {
            let diff = (last - e).abs();
            // Check if current element breaks pattern
            if last.cmp(e) != current_order || !(diff >= 1 && diff <= 3) {
                if skipped {
                    return false; // Already skipped one element, sequence invalid
                }
                skipped = true;
                continue; // Skip this element and continue checking
            }
            last = e;
        }
        true
    };

    let r: Vec<bool> = l
        .iter()
        .map(|v| {
            // Try three possibilities for each sequence:
            // 1. Use whole sequence without skipping any elements
            // 2. Skip first element
            // 3. Skip second element
            solve(v, false)
                || solve(&v.iter().skip(1).map(|x| *x).collect(), true)
                || solve(
                    &v.iter()
                        .take(1)
                        .chain(v.iter().skip(2))
                        .map(|x| *x)
                        .collect(),
                    true,
                )
        })
        .collect();
    println!("Star2: {}", r.iter().filter(|a| **a).count());
}

// Main function: Read input file and solve both parts
fn main() -> Result<(), std::io::Error> {
    // Read input file and parse into vector of integer vectors
    let input_raw = std::fs::read_to_string(input_file(file!()))?;
    let l: Data = input_raw
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|e: &str| e.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    star1(&l);
    star2(&l);
    Ok(())
}
