use std::collections::HashSet;

use aoc::input_file;

// Represents a 2D coordinate with x,y positions
type Coord = (i64, i64);

#[derive(Debug)]
struct Data {
    wall1: HashSet<Coord>,
    boxes1: HashSet<Coord>,
    wall2: HashSet<Coord>,
    boxes2: HashSet<Coord>,
    pos1: Coord,
    pos2: Coord,
    dirs: Vec<Coord>,
}

impl std::str::FromStr for Data {
    type Err = std::io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut wall1 = HashSet::new();
        let mut wall2 = HashSet::new();
        let mut boxes1 = HashSet::new();
        let mut boxes2: HashSet<(i64, i64)> = HashSet::new();
        let mut pos1 = None;
        let mut pos2 = None;
        let mut dirs = Vec::new();
        let mut it = input.split("\n\n");
        let map = it.next().unwrap();
        for (y, line) in map.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let coord = (y as i64, x as i64);
                let coord_l = (y as i64, 2 * x as i64);
                let coord_r: (i64, i64) = (y as i64, 1 + 2 * x as i64);

                match c {
                    '#' => {
                        wall1.insert(coord);
                        wall2.insert(coord_l);
                        wall2.insert(coord_r);
                    }
                    'O' => {
                        boxes1.insert(coord);
                        boxes2.insert(coord_l);
                    }
                    '@' => {
                        pos1 = Some(coord);
                        pos2 = Some(coord_l);
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
            wall1,
            wall2,
            boxes1,
            boxes2,
            pos1: pos1.unwrap(),
            pos2: pos2.unwrap(),
            dirs,
        })
    }
}

fn print_grid1(wall: &HashSet<Coord>, boxes: &HashSet<Coord>, pos: Coord) {
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
fn move_robot1(
    wall: &HashSet<Coord>,
    boxes: &mut HashSet<Coord>,
    pos: Coord,
    dirs: &[Coord],
) -> bool {
    // print_grid1(&wall, &boxes, pos);
    // println!("moving robot");
    let mut free_space = false;
    if let [head, tail @ ..] = dirs {
        let mut new_pos = (pos.0 + head.0, pos.1 + head.1);
        // println!("{new_pos:?} {head:?}");
        if wall.contains(&new_pos) {
            free_space = false;
            move_robot1(wall, boxes, pos, tail);
        } else if !boxes.contains(&new_pos) {
            free_space = true;
            move_robot1(wall, boxes, new_pos, tail);
        } else {
            // println!("Moving box");
            // Move the box (do the)
            free_space = move_robot1(wall, boxes, new_pos, &[*head]);
            if free_space {
                boxes.remove(&new_pos);
                let new_box_pos = (new_pos.0 + head.0, new_pos.1 + head.1);
                boxes.insert(new_box_pos);
            } else {
                new_pos = pos;
            }
            move_robot1(wall, boxes, new_pos, tail);
        }
    }
    free_space
}

fn print_grid2(wall: &HashSet<Coord>, boxes: &HashSet<Coord>, pos: Coord) {
    let max = wall.iter().max().unwrap();
    let mut map = vec![vec!['.'; 1 + max.1 as usize]; 1 + max.0 as usize];
    for (x, y) in wall.iter() {
        map[*x as usize][*y as usize] = '#';
    }
    for (x, y) in boxes.iter() {
        map[*x as usize][*y as usize] = '[';
        map[*x as usize][1 + *y as usize] = ']';
    }
    map[pos.0 as usize][pos.1 as usize] = '@';
    for line in map {
        println!("{}", line.iter().collect::<String>());
    }
}

// Returns the boxes to move in the direction of the caller
fn move_robot2(
    wall: &HashSet<Coord>,
    boxes: &mut HashSet<Coord>,
    pos: Coord,
    dirs: &[Coord],
    is_box: bool,
) -> Option<Vec<Coord>> {
    // To debug, provides nice visualization :)
    // if !is_box {
    //     print_grid2(&wall, &boxes, pos);
    //     println!("{pos:?} {:?}", dirs.first());
    // }
    if let [head, tail @ ..] = dirs {
        // By default there would be a wall without letting to progress
        let mut to_move: Option<Vec<(i64, i64)>> = None;
        let pos_to_check = (pos.0 + head.0, pos.1 + head.1);
        let pos_to_check_l: (i64, i64) = (pos.0 + head.0, pos.1 + head.1 - 1);
        let pos_to_check_r: (i64, i64) = (pos.0 + head.0, pos.1 + head.1 + 1);
        // Check for the adjacent position that the current robot (or box) should move to
        if wall.contains(&pos_to_check) || (is_box && wall.contains(&pos_to_check_r)) {
            move_robot2(wall, boxes, pos, tail, is_box);
        } else if {
            match head {
                (0, 1) if !is_box => boxes.contains(&(pos_to_check)),
                (0, 1) if is_box => boxes.contains(&(&pos_to_check_r)),
                (0, -1) => boxes.contains(&(pos_to_check_l)),
                (1, 0) | (-1, 0) if is_box => {
                    boxes.contains(&(&pos_to_check))
                        || boxes.contains(&(&pos_to_check_l))
                        || boxes.contains(&(&pos_to_check_r))
                }
                _ => boxes.contains(&(pos_to_check)) || boxes.contains(&(pos_to_check_l)),
            }
        } {
            match head {
                // Try to move left or right
                (0, -1) | (0, 1) => {
                    to_move = move_robot2(
                        wall,
                        boxes,
                        if *head == (0, -1) {
                            pos_to_check_l
                        } else if is_box {
                            pos_to_check_r
                        } else {
                            pos_to_check
                        },
                        &[*head],
                        true,
                    );
                    if let Some(ref mut boxes_to_move) = to_move {
                        if boxes.contains(&pos) {
                            boxes_to_move.push(pos);
                        }
                    }
                }
                // Try to move up or down
                (-1, 0) | (1, 0) => {
                    // For vertical movements, propagate the effect for the corresponding box and the one inmediately to its right
                    let adjacent_box: (i64, i64) = if boxes.contains(&pos_to_check) {
                        pos_to_check
                    } else {
                        pos_to_check_l
                    };
                    let on_left = if boxes.contains(&adjacent_box) {
                        move_robot2(wall, boxes, adjacent_box, &[*head], true)
                    } else {
                        Some(Vec::new())
                    };
                    let on_right = if is_box
                        && adjacent_box == pos_to_check_l
                        && boxes.contains(&pos_to_check_r)
                    {
                        move_robot2(wall, boxes, pos_to_check_r, &[*head], true)
                    } else {
                        Some(Vec::new())
                    };
                    if on_left.is_some() && on_right.is_some() {
                        let mut stacked_boxes = on_left.unwrap();
                        stacked_boxes.append(&mut on_right.unwrap());
                        stacked_boxes.push(pos);
                        to_move = Some(stacked_boxes);
                    }
                }
                _ => {}
            };

            // Move boxes (if possible) and do the next iteration
            if let Some(ref boxes_to_move) = to_move {
                if !is_box {
                    let mut to_insert = HashSet::new();
                    for box_pos in boxes_to_move {
                        if boxes.remove(&box_pos) {
                            let new_box_pos = (box_pos.0 + head.0, box_pos.1 + head.1);
                            to_insert.insert(new_box_pos);
                        }
                    }
                    for new_box_pos in to_insert {
                        boxes.insert(new_box_pos);
                    }
                }
                move_robot2(wall, boxes, pos_to_check, tail, is_box);
            } else {
                move_robot2(wall, boxes, pos, tail, is_box);
            }
        } else {
            // If the robot (or box) is in an occupied position, it becomes free
            let mut r = Vec::new();
            r.push(pos);
            to_move = Some(r);
            move_robot2(wall, boxes, pos_to_check, tail, is_box);
        }
        //println!("Returning {pos:?} {to_move:?}");
        to_move
    } else {
        Some(Vec::new())
    }
}

fn stars(
    Data {
        wall1,
        wall2,
        mut boxes1,
        mut boxes2,
        pos1,
        pos2,
        dirs,
    }: Data,
) {
    move_robot1(&wall1, &mut boxes1, pos1, &dirs);
    println!(
        "Star1: {:?}",
        boxes1.iter().map(|(x, y)| x * 100 + y).sum::<i64>()
    );

    move_robot2(&wall2, &mut boxes2, pos2, &dirs, false);
    println!(
        "Star2: {:?}",
        boxes2.iter().map(|(x, y)| x * 100 + y).sum::<i64>()
    );
}

fn main() -> Result<(), std::io::Error> {
    let input_raw = std::fs::read_to_string(input_file(file!()))?;
    let parsed_data: Data = input_raw.parse()?;
    stars(parsed_data);
    Ok(())
}
