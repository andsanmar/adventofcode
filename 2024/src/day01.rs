use aoc::input_file;

type Data = (Vec<u32>, Vec<u32>);

fn star1(l: &mut Data) {
    l.0.sort();
    l.1.sort();
    let r = std::iter::zip(&l.0, &l.1)
        .map(|(a, b)| (*a as i32 - *b as i32).abs() as u64)
        .sum::<u64>();
    println!("Star1: {}", r);
}

fn star2(l: &Data) {
    let mut similarity_score: u64 = 0;
    // Count occurrences in first vector
    let mut counts0 = std::collections::HashMap::new();
    for &num in &l.0 {
        *counts0.entry(num).or_insert(0) += 1;
    }

    // Count occurrences in second vector
    let mut counts1 = std::collections::HashMap::new();
    for &num in &l.1 {
        *counts1.entry(num).or_insert(0) += 1;
    }

    for key in counts0.keys() {
        if let Some(&count) = counts1.get(key) {
            similarity_score += count as u64 * *key as u64 * *counts0.get(key).unwrap();
        }
    }

    println!("Star2: {}", similarity_score);
}

fn main() {
    let input_raw = std::fs::read_to_string(input_file(file!())).unwrap();
    let mut l: Data = input_raw
        .lines()
        .map(|s| {
            let mut parts = s.split_whitespace();
            (
                parts.next().unwrap_or("0").parse::<u32>().unwrap(),
                parts.next().unwrap_or("0").parse::<u32>().unwrap(),
            )
        })
        .unzip();

    star1(&mut l);
    star2(&l);
}
