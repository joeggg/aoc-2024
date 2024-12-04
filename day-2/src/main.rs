use std::{cmp::Ordering, fs};

use aoc_tools::run_solution;

fn main() {
    let reports = read_input();

    run_solution(|| count_safe_reports(&reports, false), 1);
    run_solution(|| count_safe_reports(&reports, true), 2);
}

fn count_safe_reports(reports: &[Vec<i32>], use_dampener: bool) -> i32 {
    let mut count = 0;

    for report in reports {
        let diffs = get_diffs(report);

        if !violates_rules(report, &diffs) {
            count += 1;
            continue;
        }

        if !use_dampener {
            continue;
        }

        for i in 0..report.len() {
            let mut new_report = report.clone();
            new_report.remove(i);
            let new_diffs = get_diffs(&new_report);
            if !violates_rules(&new_report, &new_diffs) {
                count += 1;
                break;
            }
        }
    }
    count
}

fn get_diffs(report: &[i32]) -> Vec<i32> {
    report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(prev, num)| num - prev)
        .collect()
}

fn violates_rules(report: &[i32], diffs: &[i32]) -> bool {
    let (mut num_incr, mut num_decr) = (0, 0);
    for diff in diffs {
        if diff.abs() > 3 {
            return true;
        }
        match diff.cmp(&0) {
            Ordering::Less => num_decr += 1,
            Ordering::Equal => return true,
            Ordering::Greater => num_incr += 1,
        }
    }
    if num_incr == report.len() - 1 || num_decr == report.len() - 1 {
        return false;
    }
    true
}

fn read_input() -> Vec<Vec<i32>> {
    fs::read_to_string("day-2/input.txt")
        .expect("Error reading file")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().expect("Failed to parse to int"))
                .collect()
        })
        .collect()
}
