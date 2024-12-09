use std::{collections::HashSet, fs};

use aoc_tools::run_solution;

fn main() {
    let map = read_input("input.txt");
    run_solution(|| get_num_visited_positions(&map), 1);
    run_solution(|| get_number_loops(&map), 2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    fn reverse(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn apply_move(
        &self,
        coords: &(usize, usize),
        max_i: &i64,
        max_j: &i64,
    ) -> Option<(usize, usize)> {
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
        if new_coords.0 < 0 || new_coords.0 >= *max_i || new_coords.1 < 0 || new_coords.1 >= *max_j
        {
            None
        } else {
            Some((new_coords.0 as usize, new_coords.1 as usize))
        }
    }
}

fn get_num_visited_positions(map: &[Vec<char>]) -> usize {
    let (obstacle_coords, guard_coords) = get_obstacle_and_guard_coords(map);
    let path_taken = get_path_taken(map, &obstacle_coords, guard_coords);

    let mut unique_positions = HashSet::new();
    path_taken.iter().for_each(|(coords, _)| {
        unique_positions.insert(coords);
    });

    unique_positions.len()
}

fn get_obstacle_and_guard_coords(map: &[Vec<char>]) -> (HashSet<(usize, usize)>, (usize, usize)) {
    let mut obstacle_coords = HashSet::new();
    let mut guard_coords = (0, 0);

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
    (obstacle_coords, guard_coords)
}

fn get_path_taken(
    map: &[Vec<char>],
    obstacle_coords: &HashSet<(usize, usize)>,
    mut guard_coords: (usize, usize),
) -> Vec<((usize, usize), Direction)> {
    let max_i = map.len() as i64;
    let max_j = map[0].len() as i64;
    let mut guard_dir = Direction::Up;
    let mut visited_locations = vec![(guard_coords, guard_dir)];

    while let Some(coords) = guard_dir.apply_move(&guard_coords, &max_i, &max_j) {
        if obstacle_coords.contains(&coords) {
            guard_dir = guard_dir.turn_right();
        } else {
            guard_coords = coords;
            visited_locations.push((guard_coords, guard_dir));
        }
    }
    //print_map(map, &visited_locations);
    visited_locations
}

fn get_number_loops(map: &[Vec<char>]) -> usize {
    let mut loop_obstacles = HashSet::new();
    let max_i = map.len() as i64;
    let max_j = map[0].len() as i64;
    let (obstacle_coords, guard_coords) = get_obstacle_and_guard_coords(map);
    let positions = get_path_taken(map, &obstacle_coords, guard_coords);
    let mut path_so_far = HashSet::new();

    for (coords, dir) in positions {
        if coords == guard_coords || loop_obstacles.contains(&coords) {
            path_so_far.insert((coords, dir));
            continue;
        }
        // Imagine an obstacle is at coords, step back one and turn
        let mut curr_coords = dir.reverse().apply_move(&coords, &max_i, &max_j).unwrap();
        let mut curr_dir = dir.turn_right();
        let mut curr_path = path_so_far.clone();

        // Project path to check for loop
        while let Some(new_coords) = curr_dir.apply_move(&curr_coords, &max_i, &max_j) {
            if obstacle_coords.contains(&new_coords) || new_coords == coords {
                curr_dir = curr_dir.turn_right();
            } else if curr_path.contains(&(new_coords, curr_dir)) {
                // Rejoining path already travelled
                // TODO: fix weird check needed here to reduce the overestimate
                if test_loop(map, &obstacle_coords, guard_coords, coords) {
                    loop_obstacles.insert(coords);
                }
                break;
            } else {
                curr_coords = new_coords;
                curr_path.insert((curr_coords, curr_dir));
            };
        }
        path_so_far.insert((coords, dir));
    }
    loop_obstacles.len()
}

fn test_loop(
    map: &[Vec<char>],
    obstacle_coords: &HashSet<(usize, usize)>,
    mut guard_coords: (usize, usize),
    new_obstacle_coords: (usize, usize),
) -> bool {
    let max_i = map.len() as i64;
    let max_j = map[0].len() as i64;
    let mut curr_dir = Direction::Up;
    let mut curr_path = HashSet::new();

    // Project path to check for loop
    while let Some(new_coords) = curr_dir.apply_move(&guard_coords, &max_i, &max_j) {
        if obstacle_coords.contains(&new_coords) || new_coords == new_obstacle_coords {
            curr_dir = curr_dir.turn_right();
        } else if curr_path.contains(&(new_coords, curr_dir)) {
            // Rejoining path already travelled
            return true;
        } else {
            guard_coords = new_coords;
            curr_path.insert((guard_coords, curr_dir));
        };
    }
    false
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
