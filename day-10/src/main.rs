use std::{collections::HashSet, fs};

use aoc_tools::run_solution;

fn main() {
    let map = read_input("input.txt");
    run_solution(|| get_trailhead_scores(&map, false), 1);
    run_solution(|| get_trailhead_scores(&map, true), 2);
}

fn get_trailhead_scores(map: &[Vec<u8>], use_trail_count: bool) -> usize {
    let mut trailhead_locations: Vec<(usize, usize)> = Vec::new();

    map.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, entry)| {
            if *entry == 0 {
                trailhead_locations.push((i, j));
            }
        })
    });

    let mut total = 0;

    for (i, j) in trailhead_locations {
        let trails = find_trails(map, i, j, Vec::new());
        if use_trail_count {
            total += trails.len()
        } else {
            let unique_finals: HashSet<&(usize, usize)> =
                trails.iter().map(|trail| trail.last().unwrap()).collect();
            total += unique_finals.len();
        }
        //println!("Trailhead {:?}, {} trails", (i, j), trails.len());
        //print_trails(map, &trails);
    }

    total
}

fn find_trails(
    map: &[Vec<u8>],
    i: usize,
    j: usize,
    mut trail: Vec<(usize, usize)>,
) -> Vec<Vec<(usize, usize)>> {
    trail.push((i, j));
    let current_height = map[i][j];
    if current_height == 9 {
        return vec![trail];
    }

    let max_i = map.len();
    let max_j = map[0].len();
    let mut trails = Vec::new();
    if i + 1 < max_i && map[i + 1][j] == current_height + 1 {
        //println!("Moving down");
        trails.extend(find_trails(map, i + 1, j, trail.clone()));
    }

    if j + 1 < max_j && map[i][j + 1] == current_height + 1 {
        //println!("Moving right");
        trails.extend(find_trails(map, i, j + 1, trail.clone()));
    }

    if i > 0 && map[i - 1][j] == current_height + 1 {
        //println!("Moving up");
        trails.extend(find_trails(map, i - 1, j, trail.clone()));
    }

    if j > 0 && map[i][j - 1] == current_height + 1 {
        //println!("Moving left");
        trails.extend(find_trails(map, i, j - 1, trail.clone()));
    }

    trails
}

fn print_trails(map: &[Vec<u8>], trails: &[Vec<(usize, usize)>]) {
    let all_coords: HashSet<&(usize, usize)> = trails.iter().flatten().collect();

    for (i, row) in map.iter().enumerate() {
        for (j, entry) in row.iter().enumerate() {
            if all_coords.contains(&(i, j)) {
                print!("{}", entry);
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn read_input(filename: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(format!("day-10/{}", filename))
        .expect("Failed to read input")
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}
