use aoc::input_file;

#[derive(Debug)]
struct Data(std::collections::HashMap<(i32, i32), char>);

impl std::str::FromStr for Data {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Data(
            s.lines()
                .enumerate()
                .map(|(line_n, line)| {
                    line.chars()
                        .enumerate()
                        .map(move |(column_n, c)| ((line_n as i32, column_n as i32), c))
                })
                .flatten()
                .collect::<std::collections::HashMap<(i32, i32), char>>(),
        ))
    }
}

fn star1(l: &Data) {
    let Data(d) = l;
    let mut sum = 0;
    for line_n in 0.. {
        if let None = d.get(&(line_n, 0)) {
            break;
        }
        for column_n in 0.. {
            if let None = d.get(&(line_n, column_n)) {
                break;
            }
            if let Some('X') = d.get(&(line_n, column_n)) {
                if let (Some('M'), Some('A'), Some('S')) = (
                    d.get(&(line_n + 1, column_n)),
                    d.get(&(line_n + 2, column_n)),
                    d.get(&(line_n + 3, column_n)),
                ) {
                    sum += 1;
                }
                if let (Some('M'), Some('A'), Some('S')) = (
                    d.get(&(line_n + 1, column_n + 1)),
                    d.get(&(line_n + 2, column_n + 2)),
                    d.get(&(line_n + 3, column_n + 3)),
                ) {
                    sum += 1;
                }
                if let (Some('M'), Some('A'), Some('S')) = (
                    d.get(&(line_n, column_n + 1)),
                    d.get(&(line_n, column_n + 2)),
                    d.get(&(line_n, column_n + 3)),
                ) {
                    sum += 1;
                }
                if let (Some('M'), Some('A'), Some('S')) = (
                    d.get(&(line_n - 1, column_n + 1)),
                    d.get(&(line_n - 2, column_n + 2)),
                    d.get(&(line_n - 3, column_n + 3)),
                ) {
                    sum += 1;
                }
                if let (Some('M'), Some('A'), Some('S')) = (
                    d.get(&(line_n + 1, column_n - 1)),
                    d.get(&(line_n + 2, column_n - 2)),
                    d.get(&(line_n + 3, column_n - 3)),
                ) {
                    sum += 1;
                }
                if let (Some('M'), Some('A'), Some('S')) = (
                    d.get(&(line_n - 1, column_n)),
                    d.get(&(line_n - 2, column_n)),
                    d.get(&(line_n - 3, column_n)),
                ) {
                    sum += 1;
                }
                if let (Some('M'), Some('A'), Some('S')) = (
                    d.get(&(line_n, column_n - 1)),
                    d.get(&(line_n, column_n - 2)),
                    d.get(&(line_n, column_n - 3)),
                ) {
                    sum += 1;
                }
                if let (Some('M'), Some('A'), Some('S')) = (
                    d.get(&(line_n - 1, column_n - 1)),
                    d.get(&(line_n - 2, column_n - 2)),
                    d.get(&(line_n - 3, column_n - 3)),
                ) {
                    sum += 1;
                }
            }
        }
    }
    println!("Star1: {sum:?}");
}

fn star2(l: &Data) {
    let Data(d) = l;
    let mut sum = 0;
    for line_n in 0.. {
        if let None = d.get(&(line_n, 0)) {
            break;
        }
        for column_n in 0.. {
            if let None = d.get(&(line_n, column_n)) {
                break;
            }
            if let Some('A') = d.get(&(line_n, column_n)) {
                if let (Some('M'), Some('S'), Some('M'), Some('S')) = (
                    d.get(&(line_n - 1, column_n - 1)),
                    d.get(&(line_n + 1, column_n + 1)),
                    d.get(&(line_n - 1, column_n + 1)),
                    d.get(&(line_n + 1, column_n - 1)),
                ) {
                    sum += 1;
                }
                if let (Some('S'), Some('M'), Some('S'), Some('M')) = (
                    d.get(&(line_n - 1, column_n - 1)),
                    d.get(&(line_n + 1, column_n + 1)),
                    d.get(&(line_n - 1, column_n + 1)),
                    d.get(&(line_n + 1, column_n - 1)),
                ) {
                    sum += 1;
                }
                if let (Some('M'), Some('S'), Some('S'), Some('M')) = (
                    d.get(&(line_n - 1, column_n - 1)),
                    d.get(&(line_n + 1, column_n + 1)),
                    d.get(&(line_n - 1, column_n + 1)),
                    d.get(&(line_n + 1, column_n - 1)),
                ) {
                    sum += 1;
                }
                if let (Some('S'), Some('M'), Some('M'), Some('S')) = (
                    d.get(&(line_n - 1, column_n - 1)),
                    d.get(&(line_n + 1, column_n + 1)),
                    d.get(&(line_n - 1, column_n + 1)),
                    d.get(&(line_n + 1, column_n - 1)),
                ) {
                    sum += 1;
                }
            }
        }
    }
    println!("Star1: {sum:?}");
}

// Main function: Read input file and solve both parts
fn main() -> Result<(), std::io::Error> {
    // Read input file and parse into vector of integer vectors
    let input_raw = std::fs::read_to_string(input_file(file!()))?;
    let l: Data = input_raw.parse()?;

    star1(&l);
    star2(&l);
    Ok(())
}
