use std::fs;

use aoc_tools::run_solution;
use regex::Regex;

fn main() {
    let data = read_input("input.txt");
    run_solution(|| get_min_tokens(&data, false), 1);
    run_solution(|| get_min_tokens(&data, true), 2);
}

fn get_min_tokens(data: &Vec<[(u32, u32); 3]>, fix_conversion_error: bool) -> u64 {
    let mut total = 0;
    let error_val = 10000000000000.0;

    for [(xa, ya), (xb, yb), (xt, yt)] in data {
        let (true_xt, true_yt) = if fix_conversion_error {
            (error_val + *xt as f64, error_val + *yt as f64)
        } else {
            (*xt as f64, *yt as f64)
        };
        // Solution to simultaneous equations of:
        // xa*a + xb*b = xt
        // ya*a + yb*b = yt
        let b: f64 = (true_xt / (*xb as f64 - (*xa as f64 * *yb as f64 / *ya as f64)))
            - ((*xa as f64 * true_yt) / (*xb as f64 * *ya as f64 - *xa as f64 * *yb as f64));
        let a: f64 = (true_xt - *xb as f64 * b) / *xa as f64;

        if !fix_conversion_error && (a > 100.0 || b > 100.0) {
            continue;
        }

        // please save me from those floating point errors
        if (*xa as f64 * a.round() + *xb as f64 * b.round() != true_xt)
            || (*ya as f64 * a.round() + *yb as f64 * b.round() != true_yt)
        {
            continue;
        }

        total += 3 * a.round() as u64 + b.round() as u64;
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
