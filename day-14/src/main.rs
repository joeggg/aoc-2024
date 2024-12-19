use std::fs;

use aoc_tools::run_solution;
use regex::Regex;

fn main() {
    let data = read_input("example.txt");
    println!("{:?}", data);
    run_solution(|| println!("hi"), 1);
}

fn read_input(filename: &str) -> Vec<((u32, u32), (i32, i32))> {
    let pattern = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    fs::read_to_string(format!("day-14/{}", filename))
        .expect("Failed to read input")
        .lines()
        .map(|line| {
            pattern
                .captures(line)
                .map(|cap| {
                    (
                        (
                            cap.get(1).unwrap().as_str().parse().unwrap(),
                            cap.get(2).unwrap().as_str().parse().unwrap(),
                        ),
                        (
                            cap.get(3).unwrap().as_str().parse().unwrap(),
                            cap.get(4).unwrap().as_str().parse().unwrap(),
                        ),
                    )
                })
                .unwrap()
        })
        .collect()
}
