use std::cmp::Ordering::*;
use std::collections::{HashMap, HashSet};

use aoc::input_file;

type PreOrder = HashMap<u32, HashSet<u32>>;
type PostOrder = HashMap<u32, HashSet<u32>>;
#[derive(Debug)]
struct Data((PreOrder, PostOrder), Vec<Vec<u32>>);

impl std::str::FromStr for Data {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.split("\n\n");
        let ordering = i.next().expect("page ordering");
        let update = i.next().expect("page update");
        Ok(Data(
            ordering
                .lines()
                .map(|line| {
                    let mut l = line.split("|");
                    let x = l.next().expect("x").parse().expect("x");
                    let y = l.next().expect("y").parse().expect("y");
                    (x, y)
                })
                .fold(
                    (HashMap::new(), HashMap::new()),
                    |(mut preorder, mut postorder), (x, y)| {
                        preorder
                            .entry(x)
                            .and_modify(|e| {
                                e.insert(y);
                            })
                            .or_insert_with(|| {
                                let mut v = HashSet::new();
                                v.insert(y);
                                v
                            });
                        postorder
                            .entry(y)
                            .and_modify(|e| {
                                e.insert(x);
                            })
                            .or_insert_with(|| {
                                let mut v = HashSet::new();
                                v.insert(x);
                                v
                            });
                        (preorder, postorder)
                    },
                ),
            update
                .lines()
                .map(|line| {
                    line.split(",")
                        .map(|x| x.parse().expect("parse num"))
                        .collect()
                })
                .collect(),
        ))
    }
}

fn stars(l: &Data) {
    let Data((preorder, postorder), update) = l;
    let mut r = 0;
    let mut badly_ordered = Vec::new();

    'outer: for v in update {
        let mut checked = Vec::new();
        'inner: for e in v {
            for i in &checked {
                if let Some(x) = preorder.get(&i) {
                    if !x.contains(e) {
                        badly_ordered.push(v.clone());
                        continue 'outer;
                    }
                } else if let Some(x) = postorder.get(&e) {
                    if !x.contains(i) {
                        badly_ordered.push(v.clone());
                        continue 'outer;
                    }
                } else {
                    badly_ordered.push(v.clone());
                    continue 'outer;
                }
            }
            checked.push(*e);
        }
        r += v[v.len() / 2];
    }
    println!("Star1: {r:?}");

    let mut r2 = 0;
    for mut v in badly_ordered {
        v.sort_by(|x, y| match preorder.get(x) {
            None => match postorder.get(x) {
                None => Equal,
                Some(h) => {
                    if h.contains(y) {
                        Greater
                    } else {
                        Less
                    }
                }
            },
            Some(h) => {
                if h.contains(y) {
                    Less
                } else {
                    Greater
                }
            }
        });
        r2 += v[v.len() / 2];
    }
    println!("Star2: {r2:?}");
}

// Main function: Read input file and solve both parts
fn main() -> Result<(), std::io::Error> {
    // Read input file and parse into vector of integer vectors
    let input_raw = std::fs::read_to_string(input_file(file!()))?;
    let l: Data = input_raw.parse()?;

    stars(&l);
    Ok(())
}
