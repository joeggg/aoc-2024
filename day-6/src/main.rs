use std::{collections::HashSet, fs};

use aoc_tools::run_solution;

fn main() {
    let map = read_input("input.txt");
    run_solution(|| get_distinct_positions(&map), 1);
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }

    fn apply_move(&self, coords: &(usize, usize)) -> (i64, i64) {
        let mut new_coords = (coords.0 as i64, coords.1 as i64);
        match self {
            Direction::Up => {
                new_coords.0 -= 1;
            }
            Direction::Down => {
                new_coords.0 += 1;
            }
            Direction::Left => {
                new_coords.1 -= 1;
            }
            Direction::Right => {
                new_coords.1 += 1;
            }
        }
        new_coords
    }
}

fn get_distinct_positions(map: &[Vec<char>]) -> i64 {
    let mut obstacle_coords = HashSet::new();
    let mut guard_coords = (0, 0);
    let mut guard_dir = Direction::Up;

    // Get obstacle and guard coords
    for (i, row) in map.iter().enumerate() {
        for (j, point) in row.iter().enumerate() {
            match *point {
                '#' => {
                    obstacle_coords.insert((i, j));
                }
                '^' => guard_coords = (i, j),
                _ => (),
            }
        }
    }

    let max_i = map.len() as i64;
    let max_j = map[0].len() as i64;
    let mut visited_locations = HashSet::from([guard_coords]);

    loop {
        let coords = guard_dir.apply_move(&guard_coords);

        if coords.0 < 0 || coords.0 >= max_i || coords.1 < 0 || coords.1 >= max_j {
            break;
        }

        if obstacle_coords.contains(&(coords.0 as usize, coords.1 as usize)) {
            guard_dir = guard_dir.turn_right();
        } else {
            guard_coords = (coords.0 as usize, coords.1 as usize);
            visited_locations.insert(guard_coords);
        }
    }

    //print_map(map, &visited_locations);

    visited_locations.len() as i64
}

fn print_map(map: &[Vec<char>], visited_positions: &HashSet<(usize, usize)>) {
    for (i, row) in map.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            if visited_positions.contains(&(i, j)) {
                print!("X");
            } else {
                print!("{}", item);
            }
        }
        println!();
    }
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(format!("day-6/{}", filename))
        .expect("Failed to read input")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
