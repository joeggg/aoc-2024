use std::fs;

use aoc_tools::run_solution;

fn main() {
    let data = read_input("input.txt");
    run_solution(|| get_total_calibration(&data, false), 1);
    run_solution(|| get_total_calibration(&data, true), 2);
}

fn get_total_calibration(data: &[(i64, Vec<i64>)], use_third_operator: bool) -> i64 {
    let mut result = 0;

    for (expected_total, nums) in data {
        let current_total = nums[0];
        if check_equation(
            &nums[1..],
            current_total,
            *expected_total,
            use_third_operator,
        ) {
            result += expected_total;
        }
    }
    result
}

fn check_equation(
    nums: &[i64],
    current_total: i64,
    expected_total: i64,
    use_third_operator: bool,
) -> bool {
    if nums.is_empty() {
        return false;
    }

    let new_total = current_total * nums[0];
    if nums.len() == 1 && new_total == expected_total {
        return true;
    }

    if check_equation(&nums[1..], new_total, expected_total, use_third_operator) {
        return true;
    }

    let new_total = current_total + nums[0];
    if nums.len() == 1 && new_total == expected_total {
        return true;
    }

    if check_equation(&nums[1..], new_total, expected_total, use_third_operator) {
        return true;
    }

    if use_third_operator {
        let new_total = (current_total.to_string() + &nums[0].to_string())
            .parse()
            .unwrap();
        if nums.len() == 1 && new_total == expected_total {
            return true;
        }

        if check_equation(&nums[1..], new_total, expected_total, use_third_operator) {
            return true;
        }
    }

    false
}

fn read_input(filename: &str) -> Vec<(i64, Vec<i64>)> {
    fs::read_to_string(format!("day-7/{}", filename))
        .expect("Failed to read input")
        .lines()
        .map(|line| {
            let (total, rest) = line.split_once(":").unwrap();
            (
                total.parse().unwrap(),
                rest.trim()
                    .split(" ")
                    .map(|num| num.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}
