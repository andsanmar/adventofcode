use std::cmp::Ordering;

use aoc::input_file;

type Data = Vec<Vec<i32>>;

fn star1(l: &Data) {
    let r: Vec<bool> =
        l.iter()
            .map(|v| {
                v.iter().zip(v.iter().skip(1)).map(|(a, b)| a - b).fold(
                    v[0].cmp(&v[1]),
                    |dir, diff| match dir {
                        Ordering::Equal => Ordering::Equal,
                        Ordering::Greater => {
                            if diff >= 1 && diff <= 3 {
                                Ordering::Greater
                            } else {
                                Ordering::Equal
                            }
                        }
                        Ordering::Less => {
                            if diff >= -3 && diff <= -1 {
                                Ordering::Less
                            } else {
                                Ordering::Equal
                            }
                        }
                    },
                ) != Ordering::Equal
            })
            .collect();
    println!("Star1: {}", r.iter().filter(|a| **a).count());
}

fn star2(l: &Data) {}

fn main() -> Result<(), std::io::Error> {
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
