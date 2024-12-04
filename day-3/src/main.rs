use std::fs;

use regex::{Match, Regex};

use aoc_tools::run_solution;

fn main() {
    let data = fs::read_to_string("day-3/input.txt").expect("Error reading input file");

    run_solution(|| get_mul_total(&data, false), 1);
    run_solution(|| get_mul_total(&data, true), 2);
}

fn get_mul_total(data: &str, parse_conditionals: bool) -> i64 {
    let mut total = 0;
    let mut do_idxs: Vec<usize> = Vec::new();
    let mut dont_idxs: Vec<usize> = Vec::new();

    if parse_conditionals {
        Regex::new(r"(do\(\))")
            .unwrap()
            .find_iter(data)
            .for_each(|m| do_idxs.push(m.start()));
        Regex::new(r"(don't\(\))")
            .unwrap()
            .find_iter(data)
            .for_each(|m| dont_idxs.push(m.start()));
    }

    let mul_pattern = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))").unwrap();

    for m in mul_pattern.find_iter(data) {
        if !parse_conditionals || mul_enabled(m.start(), &do_idxs, &dont_idxs) {
            total += parse_mul_command(m);
        }
    }

    total
}

fn parse_mul_command(m: Match) -> i64 {
    let (_, second_half) = m
        .as_str()
        .split_once("(")
        .expect("Error splitting mul(x,y) on (");

    let (num_1, num_2) = second_half[..second_half.len() - 1]
        .split_once(",")
        .expect("Error splitting mul(x,y) on ,");

    num_1.parse::<i64>().expect("Error parsing num_1")
        * num_2.parse::<i64>().expect("Error parsing num_2")
}

fn mul_enabled(idx: usize, do_idxs: &[usize], dont_idxs: &[usize]) -> bool {
    let closest_do_idx = get_closest_idx(idx, do_idxs);
    let closest_dont_idx = get_closest_idx(idx, dont_idxs);

    if let Some(closest_do_idx) = closest_do_idx {
        if let Some(closest_dont_idx) = closest_dont_idx {
            if closest_do_idx < closest_dont_idx {
                return false;
            }
        }
    } else if closest_dont_idx.is_some() {
        return false;
    }

    true
}
fn get_closest_idx(match_idx: usize, idxs: &[usize]) -> Option<usize> {
    let mut last_idx = None;
    for idx in idxs {
        if match_idx > *idx {
            last_idx = Some(*idx);
        } else {
            break;
        }
    }

    last_idx
}
