use std::collections::{HashMap, HashSet};

use aoc_tools::run_solution;

fn main() {
    let map = read_input("input.txt");
    run_solution(|| get_num_antinodes(&map, false), 1);
    run_solution(|| get_num_antinodes(&map, true), 2);
}

fn get_num_antinodes(map: &[Vec<char>], use_resonant_harmonics: bool) -> usize {
    let mut freq_to_locs: HashMap<char, Vec<(i64, i64)>> = HashMap::new();

    for (i, row) in map.iter().enumerate() {
        for (j, entry) in row.iter().enumerate() {
            if *entry == '.' {
                continue;
            }
            freq_to_locs
                .entry(*entry)
                .or_default()
                .push((i as i64, j as i64));
        }
    }

    let max_x = map[0].len() as i64;
    let max_y = map.len() as i64;
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for (_freq, locs) in freq_to_locs {
        for (y1, x1) in locs.iter() {
            for (y2, x2) in locs.iter() {
                if y1 == y2 && x1 == x2 {
                    continue;
                }

                let delta_y = y2 - y1;
                let delta_x = x2 - x1;

                if use_resonant_harmonics {
                    // Iterate gradient steps until hitting the edge
                    // First step back from p1 then forward from p2
                    let (mut x, mut y) = (*x1, *y1);
                    loop {
                        if x >= 0 && x < max_x && y >= 0 && y < max_y {
                            antinodes.insert((y as usize, x as usize));
                            x -= delta_x;
                            y -= delta_y;
                        } else {
                            break;
                        }
                    }
                    (x, y) = (*x2, *y2);
                    loop {
                        if x >= 0 && x < max_x && y >= 0 && y < max_y {
                            antinodes.insert((y as usize, x as usize));
                            x += delta_x;
                            y += delta_y;
                        } else {
                            break;
                        }
                    }
                } else {
                    // Move 1 step away from each antenna using the gradient
                    for (y, x) in [(y2 + delta_y, x2 + delta_x), (y1 - delta_y, x1 - delta_x)] {
                        if x >= 0 && x < max_x && y >= 0 && y < max_y {
                            antinodes.insert((y as usize, x as usize));
                        }
                    }
                }
            }
        }
    }
    print_map(map, &antinodes);

    antinodes.len()
}

fn print_map(map: &[Vec<char>], antinodes: &HashSet<(usize, usize)>) {
    for (i, row) in map.iter().enumerate() {
        for (j, entry) in row.iter().enumerate() {
            if !antinodes.contains(&(i, j)) || *entry != '.' {
                print!("{}", entry);
            } else {
                print!("#");
            }
        }
        println!();
    }
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    let data =
        std::fs::read_to_string(format!("day-8/{}", filename)).expect("Failed to read input");
    data.lines().map(|l| l.chars().collect()).collect()
}
