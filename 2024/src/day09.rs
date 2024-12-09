use std::{collections::HashMap, collections::HashSet};

use aoc::input_file;

// Represents a 2D coordinate with isize components
type Coord = (isize, isize);

/// Represents the state of a disk with files and empty spaces
#[derive(Debug, Clone)]
struct Data {
    // Vector representing disk blocks, Some(id) for files, None for empty space
    disk: Vec<Option<usize>>,
    // Maps file ID to (size, position) tuple
    sizes: HashMap<usize, (usize, usize)>,
    // Maps position to list of empty block indices (unused in current implementation)
    empty_spaces: HashMap<usize, Vec<usize>>,
}

impl std::str::FromStr for Data {
    type Err = std::io::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut disk_blocks = Vec::new();
        let mut file_sizes = HashMap::new();
        let mut is_file_block = true;
        for (block_index, character) in input.lines().next().unwrap().chars().enumerate() {
            let block_size = character.to_digit(10).expect("Not numeric char") as usize;
            disk_blocks.append(&mut vec![
                if is_file_block {
                    file_sizes.insert(block_index / 2, (block_size, disk_blocks.len()));
                    Some(block_index / 2)
                } else {
                    None
                };
                block_size
            ]);
            is_file_block = !is_file_block;
        }

        Ok(Data {
            disk: disk_blocks,
            sizes: file_sizes,
            empty_spaces: HashMap::new(),
        })
    }
}

/// Compresses disk by moving all files to the beginning, leaving empty spaces at the end
fn compress(disk_blocks: &mut Vec<Option<usize>>) {
    let mut left_index = 0;
    let mut right_index = disk_blocks.len() - 1;
    while left_index < right_index {
        if disk_blocks[left_index] == None {
            while disk_blocks[right_index] == None {
                right_index -= 1;
            }
            disk_blocks[left_index] = disk_blocks[right_index];
            disk_blocks[right_index] = None;
        }
        left_index += 1;
    }
}

/// Optimally compresses disk by moving larger files into empty spaces when possible
fn optimal_compress_without_fragmentation(
    disk_blocks: &mut Vec<Option<usize>>,
    file_metadata: &mut HashMap<usize, (usize, usize)>,
) {
    let mut current_pos = 0;
    while current_pos < disk_blocks.len() {
        if disk_blocks[current_pos] == None {
            let mut empty_block_size = 1;
            while (current_pos + empty_block_size) < disk_blocks.len()
                && disk_blocks[current_pos + empty_block_size] == None
            {
                empty_block_size += 1;
            }
            // Find largest file that fits in empty space
            let mut file_id = if let Some(id) = file_metadata.keys().max() {
                *id
            } else {
                return;
            };
            'search_file: while file_id > 0 {
                if let Some((file_size, file_position)) = file_metadata.get(&file_id) {
                    if *file_size > empty_block_size || *file_position < current_pos {
                        file_id -= 1;
                        continue;
                    }
                    // Move file to empty space
                    for block in current_pos..(current_pos + file_size) {
                        disk_blocks[block] = Some(file_id);
                    }
                    // Clear original file location
                    for block in *file_position..(file_position + file_size) {
                        disk_blocks[block] = None;
                    }
                    file_metadata.remove(&file_id);
                    break;
                }
                file_id -= 1;
            }
        }
        current_pos += 1;
    }
}

/// Calculates score for first star by compressing disk and summing position*file_id
fn star1(Data { mut disk, .. }: Data) {
    compress(&mut disk);
    println!(
        "Star1: {}",
        disk.iter()
            .enumerate()
            .fold(0, |sum, (pos, id)| sum + pos * id.unwrap_or(0))
    );
}

/// Calculates score for second star using optimal compression algorithm
fn star2(
    Data {
        mut disk,
        mut sizes,
        ..
    }: Data,
) {
    optimal_compress_without_fragmentation(&mut disk, &mut sizes);
    println!(
        "Star2: {}",
        disk.iter()
            .enumerate()
            .fold(0, |sum, (pos, id)| sum + pos * id.unwrap_or(0))
    );
}

fn main() -> Result<(), std::io::Error> {
    let input_contents = std::fs::read_to_string(input_file(file!()))?;
    let disk_data: Data = input_contents.parse()?;
    star1(disk_data.clone());
    star2(disk_data);
    Ok(())
}
