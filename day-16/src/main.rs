use std::{
    collections::{BTreeMap, HashSet},
    fs,
};

use aoc_tools::run_solution;

fn main() {
    let map = read_input("input.txt");
    run_solution(|| get_best_path_cost(&map), 1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn apply_move(&self, coord: &Coord) -> Coord {
        match self {
            Self::North => Coord {
                x: coord.x,
                y: coord.y - 1,
            },
            Self::South => Coord {
                x: coord.x,
                y: coord.y + 1,
            },
            Self::West => Coord {
                x: coord.x - 1,
                y: coord.y,
            },
            Self::East => Coord {
                x: coord.x + 1,
                y: coord.y,
            },
        }
    }

    fn possible_turns(dir: Self) -> [Self; 2] {
        match dir {
            Self::North | Self::South => [Self::West, Self::East],
            Self::West | Self::East => [Self::North, Self::South],
        }
    }
}

fn get_best_path_cost(map: &[Vec<char>]) -> u64 {
    let (start, end) = get_start_and_end(map);
    let mut visited = HashSet::new();
    let mut to_check: BTreeMap<u64, Vec<(Coord, Direction)>> =
        BTreeMap::from([(0, vec![(start, Direction::East)])]);

    while !to_check.is_empty() {
        let mut entry = to_check.first_entry().unwrap();
        let (score, locs) = (*entry.key(), entry.get_mut());
        let (coord, dir) = locs.pop().unwrap();
        if locs.is_empty() {
            entry.remove();
        }
        if visited.contains(&(coord, dir)) {
            continue;
        }
        visited.insert((coord, dir));

        let forward = dir.apply_move(&coord);

        if map[forward.y][forward.x] == '.' {
            let new_score = score + 1;
            to_check.entry(new_score).or_default().push((forward, dir));
        } else if forward == end {
            return score + 1;
        }
        let new_score = score + 1000;
        Direction::possible_turns(dir)
            .into_iter()
            .for_each(|d| to_check.entry(new_score).or_default().push((coord, d)));
    }

    panic!("No paths found to end!");
}

fn get_start_and_end(map: &[Vec<char>]) -> (Coord, Coord) {
    let mut start = None;
    let mut end = None;
    for (i, row) in map.iter().enumerate() {
        for (j, entry) in row.iter().enumerate() {
            if *entry == 'S' {
                start = Some(Coord { x: j, y: i });
            } else if *entry == 'E' {
                end = Some(Coord { x: j, y: i });
            }
        }
    }
    (start.unwrap(), end.unwrap())
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(format!("day-16/{}", filename))
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}
