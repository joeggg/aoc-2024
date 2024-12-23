use std::fs;

use aoc_tools::run_solution;

fn main() {
    let (map, moves) = read_input("input.txt");
    run_solution(|| get_total_gps_coords(&map, &moves, false), 1);
    run_solution(|| get_total_gps_coords(&map, &moves, true), 2);
}

fn get_total_gps_coords(map: &[Vec<char>], moves: &[char], use_double_map: bool) -> u64 {
    let mut current_map = if use_double_map {
        double_map(map)
    } else {
        map.to_vec()
    };
    // Find initial robot position
    let robot_pos = current_map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, &c)| if c == '@' { Some((x, y)) } else { None })
        })
        .unwrap();
    // Remove robot from map
    current_map[robot_pos.1][robot_pos.0] = '.';

    let (current_map, _) = if use_double_map {
        simulate_moves_v2(current_map, moves, robot_pos)
    } else {
        simulate_moves(current_map, moves, robot_pos)
    };

    // Calculate total gps coords
    current_map.iter().enumerate().fold(0, |acc, (y, row)| {
        acc + row.iter().enumerate().fold(0, |acc, (x, &c)| {
            if (use_double_map && c == '[') || c == 'O' {
                acc + (x as u64) + 100 * (y as u64)
            } else {
                acc
            }
        })
    })
}

type Coords = (usize, usize);

fn simulate_moves(
    mut map: Vec<Vec<char>>,
    moves: &[char],
    mut robot_pos: Coords,
) -> (Vec<Vec<char>>, Coords) {
    for current_move in moves.iter() {
        let translation = match current_move {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!("Invalid move"),
        };
        // New robot position (if we can move)
        let new_pos = apply_translation(&robot_pos, &translation, 1);
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
                    let (new_x, new_y) = apply_translation(&(x, y), &translation, 1);
                    map[new_y][new_x] = 'O';
                });
                robot_pos = new_pos;
                break;
            } else {
                panic!("Invalid space");
            }
            next_pos = apply_translation(&next_pos, &translation, 1);
        }
    }
    (map, robot_pos)
}

fn double_map(map: &[Vec<char>]) -> Vec<Vec<char>> {
    map.iter().fold(Vec::new(), |mut acc, row| {
        acc.push(row.iter().fold(Vec::new(), |mut acc, c| {
            if *c == '@' {
                acc.push('@');
                acc.push('.');
            } else if *c == 'O' {
                acc.push('[');
                acc.push(']');
            } else {
                acc.push(*c);
                acc.push(*c);
            }
            acc
        }));
        acc
    })
}

fn simulate_moves_v2(
    mut map: Vec<Vec<char>>,
    moves: &[char],
    mut robot_pos: Coords,
) -> (Vec<Vec<char>>, Coords) {
    for current_move in moves.iter() {
        let translation = match current_move {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!("Invalid move"),
        };
        // New robot position (if we can move)
        let new_pos = apply_translation(&robot_pos, &translation, 1);
        let (boxes, hit_wall) = find_boxes(&map, &translation, &new_pos);

        if !hit_wall {
            // Have to iterate twice so boxes dont overwrite each other since order not guaranteed
            boxes.iter().rev().for_each(|(left, right)| {
                map[left.1][left.0] = '.';
                map[right.1][right.0] = '.';
            });
            boxes.into_iter().rev().for_each(|(left, right)| {
                let new_left = apply_translation(&left, &translation, 1);
                let new_right = apply_translation(&right, &translation, 1);
                map[new_left.1][new_left.0] = '[';
                map[new_right.1][new_right.0] = ']';
            });
            robot_pos = new_pos;
        }
    }
    (map, robot_pos)
}

fn find_boxes(
    map: &[Vec<char>],
    translation: &(i64, i64),
    pos: &Coords,
) -> (Vec<(Coords, Coords)>, bool) {
    let mut boxes = Vec::new();
    let c = &map[pos.1][pos.0];

    match *c {
        '#' => (boxes, true),
        '.' => (boxes, false),
        '[' | ']' => {
            // Find the left and right sides of the box and add to boxes vec
            let (left, right) = if *c == '[' {
                (*pos, apply_translation(pos, &(1, 0), 1))
            } else {
                (apply_translation(pos, &(-1, 0), 1), *pos)
            };
            boxes.push((left, right));
            // If x axis translation, move 2 spaces in the given direction
            if translation.0 != 0 {
                let next_pos = if translation.0 == 1 { &left } else { &right };
                let (new_boxes, hit_wall) = find_boxes(
                    map,
                    translation,
                    &apply_translation(next_pos, translation, 2),
                );
                if hit_wall {
                    return (boxes, true);
                }
                boxes.extend(new_boxes);
            } else if translation.1 != 0 {
                // Expand down the left side
                let (new_boxes, hit_wall) =
                    find_boxes(map, translation, &apply_translation(&left, translation, 1));
                if hit_wall {
                    return (boxes, true);
                }
                boxes.extend(new_boxes);
                // Expand down the right side
                let (new_boxes, hit_wall) =
                    find_boxes(map, translation, &apply_translation(&right, translation, 1));
                if hit_wall {
                    return (boxes, true);
                }
                boxes.extend(new_boxes);
            } else {
                panic!("Invalid translation");
            }
            (boxes, false)
        }
        _ => panic!("Invalid space"),
    }
}

fn apply_translation(pos: &Coords, translation: &(i64, i64), times: i64) -> (usize, usize) {
    (
        (pos.0 as i64 + times * translation.0) as usize,
        (pos.1 as i64 + times * translation.1) as usize,
    )
}

fn print_map(map: &[Vec<char>], robot_pos: &Coords) {
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
