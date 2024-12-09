use std::collections::HashMap;

use aoc::input_file;

/// Represents the state of a disk with files and empty spaces
#[derive(Debug, Clone)]
struct Data {
    // Vector representing disk blocks, Some(id) for files, None for empty space
    disk: Vec<Option<usize>>,
    // Maps file ID to (size, position) tuple
    sizes: HashMap<usize, (usize, usize)>,
    // Maps position to size of empty block
    empty_spaces: Vec<(usize, usize)>,
}

impl std::str::FromStr for Data {
    type Err = std::io::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut disk_blocks = Vec::new();
        let mut file_sizes = HashMap::new();
        let mut empty_spaces = Vec::new();
        let mut is_file_block = true;
        for (block_index, character) in input.lines().next().unwrap().chars().enumerate() {
            let block_size = character.to_digit(10).expect("Not numeric char") as usize;
            disk_blocks.append(&mut vec![
                if is_file_block {
                    file_sizes.insert(block_index / 2, (block_size, disk_blocks.len()));
                    Some(block_index / 2)
                } else {
                    empty_spaces.push((disk_blocks.len(), block_size));
                    None
                };
                block_size
            ]);
            is_file_block = !is_file_block;
        }

        Ok(Data {
            disk: disk_blocks,
            sizes: file_sizes,
            empty_spaces: empty_spaces,
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
fn _optimal_compress_without_fragmentation(
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
                        continue 'search_file;
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
                    break 'search_file;
                }
                file_id -= 1;
            }
        }
        current_pos += 1;
    }
}

/// Optimally compresses disk by moving larger files into empty spaces when possible.
/// This function implements a more sophisticated compression algorithm that tries to
/// fit files into available empty spaces while maintaining optimal placement.
///
/// The algorithm works by:
/// 1. Processing files from largest ID to smallest
/// 2. For each file, finding an empty space that:
///    - Comes before the file's current position
///    - Is large enough to fit the file
/// 3. Moving the file to the found empty space if one exists
/// 4. Tracking and updating empty spaces as they are split
///
/// # Arguments
/// * `disk_blocks` - Mutable vector representing disk blocks (Some(id) for files, None for empty)
/// * `file_metadata` - HashMap mapping file ID to (size, position) tuple
/// * `empty_spaces` - Vector of (position, size) tuples representing empty spaces
fn compress2(
    disk_blocks: &mut Vec<Option<usize>>,
    file_metadata: &mut HashMap<usize, (usize, usize)>,
    empty_spaces: &mut Vec<(usize, usize)>,
) {
    // Get the largest file ID, return if no files exist
    let mut current_file_id = if let Some(largest_file_id) = file_metadata.keys().max() {
        *largest_file_id
    } else {
        return;
    };

    // Process files from largest to smallest ID
    while current_file_id > 0 {
        if let Some((file_size, file_current_position)) = file_metadata.get(&current_file_id) {
            let mut target_empty_position = None;

            // Search for suitable empty space that comes before current file
            'find_empty_space: for empty_space_index in 0..empty_spaces.len() {
                let (empty_space_position, empty_space_size) = empty_spaces[empty_space_index];

                // Only use spaces that come before current file position and are large enough
                if empty_space_position < *file_current_position && empty_space_size >= *file_size {
                    target_empty_position = Some(empty_space_position);

                    // If we split a larger empty space, leave the remainder in the same position
                    if empty_space_size != *file_size {
                        let remaining_space = (
                            empty_space_position + file_size,
                            empty_space_size - file_size,
                        );
                        empty_spaces[empty_space_index] = remaining_space;
                    } else {
                        empty_spaces.remove(empty_space_index);
                    }
                    break 'find_empty_space;
                }
            }

            // Move file to new position if suitable empty space was found
            if let Some(new_file_position) = target_empty_position {
                // Write file blocks to new position
                for block_index in new_file_position..(new_file_position + file_size) {
                    disk_blocks[block_index] = Some(current_file_id);
                }

                // Clear original file location blocks
                for block_index in *file_current_position..(file_current_position + file_size) {
                    disk_blocks[block_index] = None;
                }

                // Remove processed file from metadata tracking
                file_metadata.remove(&current_file_id);
            }
        }
        current_file_id -= 1;
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
        mut empty_spaces,
    }: Data,
) {
    compress2(&mut disk, &mut sizes, &mut empty_spaces);
    // We can calculate the checksum only with the sizes data structure, but this exercise is left to the reader
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
