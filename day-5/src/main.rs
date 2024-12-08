use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs,
};

use aoc_tools::run_solution;

fn main() {
    let (rules, pages) = read_input("input.txt");
    run_solution(|| get_ordered_pages_mid_total(&rules, &pages), 1);
    run_solution(|| get_unordered_pages_mid_total(&rules, &pages), 2);
}

fn get_ordered_pages_mid_total(rules: &[(u8, u8)], pages: &[Vec<u8>]) -> u64 {
    let mut total = 0;
    let mut num_to_later: HashMap<&u8, HashSet<&u8>> = HashMap::new();

    rules.iter().for_each(|(smaller, bigger)| {
        num_to_later.entry(smaller).or_default().insert(bigger);
    });

    for page in pages {
        if page.is_sorted_by(|a, b| is_in_set_mapping(a, b, &num_to_later)) {
            let mid_idx = (page.len() as i64 - 1) / 2;
            total += page[mid_idx as usize] as u64;
        }
    }
    total
}

fn get_unordered_pages_mid_total(rules: &[(u8, u8)], pages: &[Vec<u8>]) -> u64 {
    let mut total = 0;
    let mut num_to_later: HashMap<&u8, HashSet<&u8>> = HashMap::new();

    rules.iter().for_each(|(smaller, bigger)| {
        num_to_later.entry(smaller).or_default().insert(bigger);
    });

    for page in pages {
        if !page.is_sorted_by(|a, b| is_in_set_mapping(a, b, &num_to_later)) {
            let page_cpy = sort_page(page, &num_to_later);
            //let mut page_cpy = page.clone();
            //page_cpy.sort_by(|a, b| match num_to_later.get(a).unwrap().get(b) {
            //    Some(_) => Ordering::Less,
            //    None => Ordering::Greater,
            //});
            if !page_cpy.is_sorted_by(|a, b| is_in_set_mapping(a, b, &num_to_later)) {
                println!("{:?}", page);
                println!("{:?}", page_cpy);
                println!();
            }

            let mid_idx = (page.len() as i64 - 1) / 2;
            total += page_cpy[mid_idx as usize] as u64;
        }
    }
    total
}

fn is_in_set_mapping(a: &u8, b: &u8, set_mapping: &HashMap<&u8, HashSet<&u8>>) -> bool {
    set_mapping.get(a).and_then(|set| set.get(b)).is_some()
}

fn sort_page(page: &[u8], num_to_later: &HashMap<&u8, HashSet<&u8>>) -> Vec<u8> {
    let mut page_cpy = page.to_vec();

    loop {
        let mut to_move = None;
        for (i, num) in page_cpy.iter().enumerate() {
            for (j, prev_num) in page_cpy.iter().take(i).enumerate() {
                if num_to_later.get(num).unwrap().get(prev_num).is_some() {
                    to_move = Some((i, j));
                    break;
                }
            }
            if to_move.is_some() {
                break;
            }
        }
        if let Some((i, j)) = to_move {
            let num = page_cpy.remove(i);
            page_cpy.insert(j, num);
            continue;
        }
        break;
    }
    page_cpy
}

fn is_page_ordered<'a>(page: &'a [u8], num_to_later: &mut HashMap<&'a u8, HashSet<&u8>>) -> bool {
    for (i, num) in page.iter().enumerate() {
        let later_nums = num_to_later.entry(num).or_default();

        for other_num in page.iter().take(i) {
            if later_nums.contains(other_num) {
                return false;
            }
        }
    }
    true
}

fn read_input(filename: &str) -> (Vec<(u8, u8)>, Vec<Vec<u8>>) {
    let raw = fs::read_to_string(format!("day-5/{}", filename)).expect("Failed to read input");
    let (raw_rules, raw_pages) = raw.split_once("\n\n").unwrap();

    let rules: Vec<(u8, u8)> = raw_rules
        .lines()
        .map(|line| {
            line.split_once("|")
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap()
        })
        .collect();

    let pages: Vec<Vec<u8>> = raw_pages
        .lines()
        .map(|line| line.split(",").map(|x| x.parse().unwrap()).collect())
        .collect();

    (rules, pages)
}
