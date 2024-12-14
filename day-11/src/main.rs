use std::fs;

use aoc_tools::run_solution;

fn main() {
    let stones = read_input("input.txt");
    run_solution(|| get_num_stones(&stones, 25), 1);
}

fn get_num_stones(stones: &[String], blinks: u32) -> usize {
    let mut current_stones = stones.to_vec();
    //println!("{:?}", current_stones);

    for _ in 0..blinks {
        let mut next_stones = Vec::new();
        for stone in current_stones {
            if stone == "0" {
                // change to 1
                next_stones.push("1".to_string());
            } else if stone.len() % 2 == 0 {
                // split in 2
                let (first, second) = stone.split_at(stone.len() / 2);
                next_stones.push((first.parse::<u64>().unwrap()).to_string());
                next_stones.push((second.parse::<u64>().unwrap()).to_string());
            } else {
                // multiply by 2024
                let new_val = stone.parse::<u64>().unwrap() * 2024;
                next_stones.push(new_val.to_string())
            }
        }
        current_stones = next_stones;
        //println!("{:?}", current_stones);
    }
    current_stones.len()
}

fn read_input(filename: &str) -> Vec<String> {
    fs::read_to_string(format!("day-11/{}", filename))
        .expect("Failed to read input")
        .trim()
        .split(" ")
        .map(|s| s.to_string())
        .collect()
}
