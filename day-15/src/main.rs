use std::fs;

use aoc_tools::run_solution;

fn main() {
    let (map, moves) = read_input("input.txt");
    run_solution(|| get_total_gps_coords(&map, &moves), 1);
}

fn get_total_gps_coords(map: &[Vec<char>], moves: &[char]) -> u64 {
    // Find initial robot position
    let robot_pos = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, &c)| if c == '@' { Some((x, y)) } else { None })
        })
        .unwrap();
    // Remove robot from map
    let mut current_map = map.to_vec();
    current_map[robot_pos.1][robot_pos.0] = '.';

    let (current_map, _) = simulate_moves(current_map, moves, robot_pos);

    // Calculate total gps coords
    current_map.iter().enumerate().fold(0, |acc, (y, row)| {
        acc + row.iter().enumerate().fold(0, |acc, (x, &c)| {
            if c == 'O' {
                acc + (x as u64) + 100 * (y as u64)
            } else {
                acc
            }
        })
    })
}

fn simulate_moves(
    mut map: Vec<Vec<char>>,
    moves: &[char],
    mut robot_pos: (usize, usize),
) -> (Vec<Vec<char>>, (usize, usize)) {
    for current_move in moves.iter() {
        let translation = match current_move {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!("Invalid move"),
        };
        // New robot position (if we can move)
        let new_pos = (
            (robot_pos.0 as i32 + translation.0) as usize,
            (robot_pos.1 as i32 + translation.1) as usize,
        );
        let mut next_pos = new_pos;
        let mut boxes = Vec::new();
        loop {
            // Move along the translation, if we hit a wall stop, if we hit a box, accumulate it
            // if we hit a space, move the accumulated boxes and the robot
            if map[next_pos.1][next_pos.0] == '#' {
                break;
            } else if map[next_pos.1][next_pos.0] == 'O' {
                boxes.push(next_pos);
            } else if map[next_pos.1][next_pos.0] == '.' {
                boxes.into_iter().rev().for_each(|(x, y)| {
                    map[y][x] = '.';
                    map[(y as i32 + translation.1) as usize][(x as i32 + translation.0) as usize] =
                        'O';
                });
                robot_pos = new_pos;
                break;
            } else {
                panic!("Invalid space");
            }
            next_pos = (
                (next_pos.0 as i32 + translation.0) as usize,
                (next_pos.1 as i32 + translation.1) as usize,
            );
        }
    }
    (map, robot_pos)
}

fn print_map(map: &[Vec<char>], robot_pos: &(usize, usize)) {
    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| {
            if x == robot_pos.0 && y == robot_pos.1 {
                print!("@")
            } else {
                print!("{}", c);
            }
        });
        println!();
    });
}

fn read_input(filename: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let raw = fs::read_to_string(format!("day-15/{}", filename)).expect("Failed to read input");
    let (map, moves) = raw.split_once("\n\n").unwrap();
    (
        map.lines().map(|l| l.chars().collect()).collect(),
        moves.chars().filter(|c| *c != '\n').collect(),
    )
}
