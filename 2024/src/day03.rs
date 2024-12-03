use aoc::input_file;

struct Data(Vec<(u64, u64, bool)>);

// Associated regex mul\((\d{1,3}),(\d{1,3})\)
impl std::str::FromStr for Data {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = Vec::new();
        let mut it = s.chars().peekable();
        let mut keep_scanning = true;
        'operation: while let Some(c) = it.next() {
            if c == 'd' {
                if let Some('o') = it.next_if(|x| *x == 'o') {
                    if let (Some('('), Some(')')) =
                        (it.next_if(|x| *x == '('), it.next_if(|x| *x == ')'))
                    {
                        keep_scanning = true;
                    } else if let (Some('n'), Some('\''), Some('t'), Some('('), Some(')')) = (
                        it.next_if(|x| *x == 'n'),
                        it.next_if(|x| *x == '\''),
                        it.next_if(|x: &char| *x == 't'),
                        it.next_if(|x| *x == '('),
                        it.next_if(|x| *x == ')'),
                    ) {
                        keep_scanning = false;
                    }
                }
            }
            if c == 'm' {
                if let (Some('u'), Some('l'), Some('(')) = (
                    it.next_if(|x| *x == 'u'),
                    it.next_if(|x| *x == 'l'),
                    it.next_if(|x| *x == '('),
                ) {
                    let mut x = String::new();
                    let mut counter = 0;
                    'num: while let Some(n) = it.next() {
                        counter += 1;
                        if n == ',' {
                            break 'num;
                        }
                        if counter > 3 || !n.is_numeric() {
                            continue 'operation;
                        }
                        x.push(n);
                    }
                    if x.len() < 1 || x.len() > 3 {
                        continue 'operation;
                    }
                    counter = 0;

                    let mut y = String::new();
                    'num: while let Some(n) = it.next() {
                        if n == ')' {
                            break 'num;
                        }
                        if counter > 3 || !n.is_numeric() {
                            continue 'operation;
                        }
                        y.push(n);
                    }

                    if y.len() < 1 || y.len() > 3 {
                        continue 'operation;
                    }

                    if let (Ok(x), Ok(y)) = (x.parse::<u64>(), y.parse::<u64>()) {
                        v.push((x, y, keep_scanning));
                    }
                }
            }
        }

        Ok(Data(v))
    }
}

fn star1(l: &Data) {
    let Data(ops) = l;
    println!(
        "Star1: {}",
        ops.iter().fold(0, |acc, (n1, n2, _)| acc + (n1 * n2)),
    );
}

fn star2(l: &Data) {
    let Data(ops) = l;
    println!(
        "Star1: {}",
        ops.iter()
            .fold(0, |acc, (n1, n2, b)| acc + if *b { n1 * n2 } else { 0 }),
    );
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
