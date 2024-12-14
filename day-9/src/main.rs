use std::fs;

use aoc_tools::run_solution;

fn main() {
    let data = read_input("input.txt");
    run_solution(
        || {
            let drive = get_drive_map(&data);
            get_drive_checksum(&compact_drive_fragmented(drive))
        },
        1,
    );
}

fn compact_drive_fragmented(mut drive: Vec<Option<usize>>) -> Vec<Option<usize>> {
    // Get indexes of all free slots, reverse order so we can pop to remove the earliest
    let mut free_slots: Vec<usize> = drive
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, entry)| entry.is_none())
        .map(|(i, _)| i)
        .collect();

    let mut result = None;
    let mut done = false;
    let mut last_pos = drive.len();

    while !done {
        // Iterate inwards from just before last moved file's starting index
        for (i, entry) in drive.iter().enumerate().take(last_pos).rev() {
            last_pos -= 1;
            if let Some(file_id) = entry {
                if let Some(slot) = free_slots.pop() {
                    if slot < i {
                        result = Some((*file_id, i, slot));
                    } else {
                        // Lowest slot at higher index
                        done = true;
                    }
                } else {
                    // No slots left
                    done = true;
                }
                break;
            }
        }

        // Swap file block with found slot
        if let Some((file_id, source, dest)) = result {
            drive[dest] = Some(file_id);
            drive[source] = None;
        }
    }
    drive
}

fn get_drive_map(data: &[i8]) -> Vec<Option<usize>> {
    let mut drive: Vec<Option<usize>> = Vec::new();
    let mut file_id = 0;

    for (i, num) in data.iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..*num {
                drive.push(Some(file_id));
            }
            file_id += 1;
        } else {
            for _ in 0..*num {
                drive.push(None);
            }
        }
    }

    drive
}

fn get_drive_checksum(drive: &[Option<usize>]) -> usize {
    drive
        .iter()
        .enumerate()
        .fold(0, |acc, (i, entry)| acc + i * entry.unwrap_or(0))
}

fn print_drive(drive: &[Option<usize>]) {
    for i in drive {
        match i {
            Some(id) => print!("{}", id),
            None => print!("."),
        }
    }
    println!();
}

fn read_input(filename: &str) -> Vec<i8> {
    fs::read_to_string(format!("day-9/{}", filename))
        .expect("Failed to read input")
        .trim()
        .split("")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}
