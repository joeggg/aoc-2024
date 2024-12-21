use std::fs;

use nalgebra::Vector2;
use regex::Regex;

use aoc_tools::run_solution;

fn main() {
    let data = read_input("input.txt");
    run_solution(|| get_safety_factor(&data), 1);
}

fn get_safety_factor(data: &[(Vector2<i32>, Vector2<i32>)]) -> u32 {
    let max_x = 101;
    let max_y = 103;
    let mut robots: Vec<Vector2<i32>> = data.iter().map(|(p, _)| *p).collect();
    for _t in 0..100 {
        for (i, (_, v)) in data.iter().enumerate() {
            robots[i] += v;

            if robots[i].x < 0 {
                robots[i].x += max_x;
            }
            robots[i].x %= max_x;

            if robots[i].y < 0 {
                robots[i].y += max_y;
            }
            robots[i].y %= max_y;
        }
    }

    let mut num_per_quad = [0; 4];
    let mid_x = (max_x - 1) / 2;
    let mid_y = (max_y - 1) / 2;
    for p in robots {
        if p.x == mid_x || p.y == mid_y {
            continue;
        } else if p.x < mid_x && p.y < mid_y {
            num_per_quad[0] += 1;
        } else if p.x > mid_x && p.y < mid_y {
            num_per_quad[1] += 1;
        } else if p.x < mid_x && p.y > mid_y {
            num_per_quad[2] += 1;
        } else if p.x > mid_x && p.y > mid_y {
            num_per_quad[3] += 1;
        }
    }
    num_per_quad.into_iter().product()
}

fn print_robots(robots: &[Vector2<i32>]) {
    let max_x = 101;
    let max_y = 103;
    for y in 0..max_y {
        for x in 0..max_x {
            if robots.iter().any(|p| p.x == x && p.y == y) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn read_input(filename: &str) -> Vec<(Vector2<i32>, Vector2<i32>)> {
    let pattern = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    fs::read_to_string(format!("day-14/{}", filename))
        .expect("Failed to read input")
        .lines()
        .map(|line| {
            pattern
                .captures(line)
                .map(|cap| {
                    (
                        Vector2::from([
                            cap.get(1).unwrap().as_str().parse().unwrap(),
                            cap.get(2).unwrap().as_str().parse().unwrap(),
                        ]),
                        Vector2::from([
                            cap.get(3).unwrap().as_str().parse().unwrap(),
                            cap.get(4).unwrap().as_str().parse().unwrap(),
                        ]),
                    )
                })
                .unwrap()
        })
        .collect()
}
