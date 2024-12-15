use std::collections::HashSet;

use aoc::input_file;

// Represents a 2D coordinate with x,y positions
type Coord = (i64, i64);

#[derive(Debug)]
struct Data {
    wall: HashSet<Coord>,
    boxes: HashSet<Coord>,
    pos: Coord,
    dirs: Vec<Coord>,
}

impl std::str::FromStr for Data {
    type Err = std::io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut wall = HashSet::new();
        let mut boxes = HashSet::new();
        let mut pos = None;
        let mut dirs = Vec::new();
        let mut it = input.split("\n\n");
        let map = it.next().unwrap();
        for (y, line) in map.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let coord = (y as i64, x as i64);
                match c {
                    '#' => {
                        wall.insert(coord);
                    }
                    'O' => {
                        boxes.insert(coord);
                    }
                    '@' => {
                        pos = Some(coord);
                    }
                    _ => {}
                }
            }
        }
        for dir in it.next().unwrap().chars() {
            match dir {
                '<' => dirs.push((0, -1)),
                '>' => dirs.push((0, 1)),
                '^' => dirs.push((-1, 0)),
                'v' => dirs.push((1, 0)),
                _ => {}
            }
        }
        Ok(Data {
            wall,
            boxes,
            pos: pos.unwrap(),
            dirs,
        })
    }
}

fn print_grid(wall: &HashSet<Coord>, boxes: &HashSet<Coord>, pos: Coord) {
    let max = wall.iter().max().unwrap();
    let mut map = vec![vec!['.'; 1 + max.0 as usize]; 1 + max.1 as usize];
    for (x, y) in wall.iter() {
        map[*x as usize][*y as usize] = '#';
    }
    for (x, y) in boxes.iter() {
        map[*x as usize][*y as usize] = 'O';
    }
    map[pos.0 as usize][pos.1 as usize] = '@';
    for line in map {
        println!("{}", line.iter().collect::<String>());
    }
}

// Returns if it's empty
fn move_robot(
    wall: &HashSet<Coord>,
    boxes: &mut HashSet<Coord>,
    pos: Coord,
    dirs: &[Coord],
) -> bool {
    // print_grid(&wall, &boxes, pos);
    // println!("moving robot");
    let mut free_space = false;
    if let [head, tail @ ..] = dirs {
        let mut new_pos = (pos.0 + head.0, pos.1 + head.1);
        // println!("{new_pos:?} {head:?}");
        if wall.contains(&new_pos) {
            free_space = false;
            move_robot(wall, boxes, pos, tail);
        } else if !boxes.contains(&new_pos) {
            free_space = true;
            move_robot(wall, boxes, new_pos, tail);
        } else {
            // println!("Moving box");
            // Move the box (do the)
            free_space = move_robot(wall, boxes, new_pos, &[*head]);
            if free_space {
                boxes.remove(&new_pos);
                let new_box_pos = (new_pos.0 + head.0, new_pos.1 + head.1);
                boxes.insert(new_box_pos);
            } else {
                new_pos = pos;
            }
            move_robot(wall, boxes, new_pos, tail);
        }
    }
    free_space
}

fn stars(
    Data {
        wall,
        mut boxes,
        pos,
        dirs,
    }: Data,
) {
    move_robot(&wall, &mut boxes, pos, &dirs);
    println!(
        "Star1: {:?}",
        boxes.iter().map(|(x, y)| x * 100 + y).sum::<i64>()
    );
}

fn main() -> Result<(), std::io::Error> {
    let input_raw = std::fs::read_to_string(input_file(file!()))?;
    let parsed_data: Data = input_raw.parse()?;
    stars(parsed_data);
    Ok(())
}
