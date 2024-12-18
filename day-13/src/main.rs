use std::fs;

use aoc_tools::run_solution;
use regex::Regex;

fn main() {
    let data = read_input("input.txt");
    run_solution(|| get_min_tokens(&data), 1);
}

fn get_min_tokens(data: &Vec<[(u32, u32); 3]>) -> u32 {
    let mut total = 0;
    for [(xa, ya), (xb, yb), (xt, yt)] in data {
        // Solution to simultaneous equations of:
        // xa*a + xb*b = xt
        // ya*a + yb*b = yt
        let b: f64 = (*xt as f64 / (*xb as f64 - (*xa as f64 * *yb as f64 / *ya as f64)))
            - ((*xa as f64 * *yt as f64) / (*xb as f64 * *ya as f64 - *xa as f64 * *yb as f64));
        let a: f64 = (*xt as f64 - *xb as f64 * b) / *xa as f64;

        if a > 100.0
            || b > 100.0
            // please save me from those floating point errors
            || (*xa as f64 * a.round() + *xb as f64 * b.round() != *xt as f64)
            || (*ya as f64 * a.round() + *yb as f64 * b.round() != *yt as f64)
        {
            continue;
        }

        total += 3 * a.round() as u32 + b.round() as u32;
    }
    total
}

fn read_input(filename: &str) -> Vec<[(u32, u32); 3]> {
    let mut data = Vec::new();
    let mut current: Vec<(u32, u32)> = Vec::new();
    let button_pattern = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
    let prize_pattern = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();

    for line in fs::read_to_string(format!("day-13/{}", filename))
        .expect("Failed to read input")
        .lines()
    {
        if line.is_empty() {
            data.push([current[0], current[1], current[2]]);
            current.clear();
        } else {
            let captures = if line.starts_with("Button") {
                button_pattern.captures(line).unwrap()
            } else {
                prize_pattern.captures(line).unwrap()
            };
            current.push((
                captures.get(1).unwrap().as_str().parse().unwrap(),
                captures.get(2).unwrap().as_str().parse().unwrap(),
            ));
        }
    }
    data.push([current[0], current[1], current[2]]);

    data
}
