use std::{collections::HashMap, fs};

use aoc_tools::run_solution;

fn main() {
    let (list_1, list_2) = read_input();

    run_solution(|| find_distance(&list_1, &list_2), 1);
    run_solution(|| get_similarity_score(&list_1, &list_2), 2);
}

fn find_distance(list_1: &[i64], list_2: &[i64]) -> i64 {
    if list_1.len() != list_2.len() {
        panic!("Lists are not the same length");
    }
    let mut vec_1 = list_1.to_vec();
    vec_1.sort();
    let mut vec_2 = list_2.to_vec();
    vec_2.sort();

    vec_1
        .into_iter()
        .zip(vec_2)
        .fold(0, |acc, (num_1, num_2)| acc + (num_1 - num_2).abs())
}

fn get_similarity_score(list_1: &[i64], list_2: &[i64]) -> i64 {
    let mut counts = HashMap::new();
    list_2.iter().for_each(|num| {
        *counts.entry(num).or_insert(0) += 1;
    });

    list_1
        .iter()
        .fold(0, |acc, num| acc + num * counts.get(&num).unwrap_or(&0))
}

fn read_input() -> (Vec<i64>, Vec<i64>) {
    fs::read_to_string("day-1/input.txt")
        .expect("Failed to read input file")
        .lines()
        .map(|line| line.split_once("   ").expect("Failed to split line"))
        .map(|(num_1, num_2)| {
            (
                num_1.parse::<i64>().expect("Failed to parse number"),
                num_2.parse::<i64>().expect("Failed to parse number"),
            )
        })
        .collect()
}
