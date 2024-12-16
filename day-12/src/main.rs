use std::collections::HashSet;

use aoc_tools::run_solution;

fn main() {
    let plots = read_input("input.txt");
    run_solution(|| get_fence_cost(&plots), 1);
}

#[derive(Debug, Clone)]
struct Region {
    name: char,
    plots: HashSet<(usize, usize)>,
    last_layer: HashSet<(usize, usize)>,
    current_layer: HashSet<(usize, usize)>,
}

impl Region {
    fn new(name: char, coords: (usize, usize)) -> Self {
        Region {
            name,
            plots: HashSet::from([(coords)]),
            last_layer: HashSet::new(),
            current_layer: HashSet::from([(coords)]),
        }
    }

    fn from_regions(name: char, regions: impl Iterator<Item = Self>) -> Self {
        let mut new_region = Region {
            name,
            plots: HashSet::new(),
            last_layer: HashSet::new(),
            current_layer: HashSet::new(),
        };
        for region in regions {
            new_region.plots.extend(region.plots);
            new_region.last_layer.extend(region.last_layer);
            new_region.current_layer.extend(region.current_layer);
        }
        new_region
    }

    fn add_plot(&mut self, plot: (usize, usize)) {
        self.plots.insert(plot);
        self.current_layer.insert(plot);
    }

    fn next_row(&mut self) {
        self.last_layer = self.current_layer.clone();
        self.current_layer.clear();
    }

    fn get_perimeter(&self) -> u64 {
        let mut perimeter = 0;
        for (i, j) in self.plots.iter() {
            let mut sides = 4;
            if *i > 0 && self.plots.contains(&(*i - 1, *j)) {
                sides -= 1;
            }
            if self.plots.contains(&(*i + 1, *j)) {
                sides -= 1;
            }
            if *j > 0 && self.plots.contains(&(*i, *j - 1)) {
                sides -= 1;
            }
            if self.plots.contains(&(*i, *j + 1)) {
                sides -= 1;
            }
            perimeter += sides;
        }
        perimeter
    }
}

fn get_fence_cost(plots: &[Vec<char>]) -> u64 {
    let mut regions: Vec<Region> = Vec::new();

    for (i, row) in plots.iter().enumerate() {
        for (j, plot) in row.iter().enumerate() {
            let mut added_to = Vec::new();
            // Check each region for a plot adjacent to current
            for (idx, region) in regions.iter_mut().enumerate() {
                // Skip regions with a different char or nothing on this or the previous row
                if region.name != *plot
                    || (region.current_layer.is_empty() && region.last_layer.is_empty())
                {
                    continue;
                }
                // Check current and last layer for an adjacent plot
                if (j > 0 && region.current_layer.contains(&(i, j - 1)))
                    || (i > 0 && region.last_layer.contains(&(i - 1, j)))
                {
                    region.add_plot((i, j));
                    added_to.push(idx);
                }
            }
            if added_to.is_empty() {
                regions.push(Region::new(*plot, (i, j)));
            }
            // Merge together regions if plot was added to more than 1 (so bridging the 2)
            if added_to.len() > 1 {
                regions.push(Region::from_regions(
                    *plot,
                    added_to.iter().map(|idx| regions[*idx].clone()),
                ));
                // Delete merged regions, starting from the latest so the indexes of earlier
                // entries don't change
                added_to.into_iter().rev().for_each(|idx| {
                    regions.remove(idx);
                });
            }
        }
        // Move current layer to last layer on each region
        regions.iter_mut().for_each(|r| r.next_row());
    }
    //regions
    //    .iter()
    //    .for_each(|r| println!("{:?}\nPerimeter: {}", r, r.get_perimeter()));

    regions
        .iter()
        .fold(0, |acc, r| acc + r.get_perimeter() * r.plots.len() as u64)
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    std::fs::read_to_string(format!("day-12/{}", filename))
        .expect("Failed to read file")
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}
