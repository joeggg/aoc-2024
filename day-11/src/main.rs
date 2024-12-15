use std::{collections::HashMap, fs};

use aoc_tools::run_solution;

fn main() {
    let stones = read_input("input.txt");
    run_solution(|| get_num_stones(&stones, 25), 1);
    run_solution(|| get_num_stones(&stones, 75), 1);
}

fn get_num_stones(stones: &[String], blinks: u32) -> usize {
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|stone| get_num_stones_after_blinks(stone, blinks, &mut cache))
        .sum()
}

fn get_num_stones_after_blinks(
    start_stone: &String,
    blinks: u32,
    cache: &mut HashMap<(String, u32), usize>,
) -> usize {
    let key = (start_stone.clone(), blinks);
    if let Some(cached) = cache.get(&key) {
        return *cached;
    }

    if blinks == 0 {
        return 0;
    }

    let mut new_stones = Vec::new();
    if start_stone == "0" {
        // change to 1
        new_stones.push("1".to_string());
    } else if start_stone.len() % 2 == 0 {
        // split in 2
        let (first, second) = start_stone.split_at(start_stone.len() / 2);
        new_stones.push((first.parse::<u64>().unwrap()).to_string());
        new_stones.push((second.parse::<u64>().unwrap()).to_string());
    } else {
        // multiply by 2024
        let new_val = start_stone.parse::<u64>().unwrap() * 2024;
        new_stones.push(new_val.to_string())
    }

    let sum = if blinks == 1 {
        new_stones.len()
    } else {
        new_stones
            .into_iter()
            .map(|s| get_num_stones_after_blinks(&s, blinks - 1, cache))
            .sum()
    };

    cache.insert(key, sum);
    sum
}

fn read_input(filename: &str) -> Vec<String> {
    fs::read_to_string(format!("day-11/{}", filename))
        .expect("Failed to read input")
        .trim()
        .split(" ")
        .map(|s| s.to_string())
        .collect()
}
