use std::fs;

use aoc_tools::run_solution;

fn main() {
    let data = read_input();
    run_solution(
        || {
            let matches = find_total_words(&data, "XMAS");
            display_results(&data, &matches);
            matches.len()
        },
        1,
    );
}

fn read_input() -> Vec<Vec<char>> {
    fs::read_to_string("day-4/example.txt")
        .expect("Failed to read input file")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn display_results(data: &[Vec<char>], matches: &Vec<Match>) {
    for (i, row) in data.iter().enumerate() {
        for (j, letter) in row.iter().enumerate() {
            let mut found = false;
            for m in matches {
                for index in m.indexes.iter().flatten() {
                    if *index == (i, j) {
                        found = true;
                    }
                }
            }
            if found {
                print!("{}", letter);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[derive(Debug, Clone)]
struct Match {
    current: usize,
    success: bool,
    values: Vec<char>,
    indexes: Vec<Option<(usize, usize)>>,
}

impl Match {
    fn new(to_find: &str, letter_num: usize, letter: &char, i: usize, j: usize) -> Match {
        let mut values = vec!['*'; to_find.len()];
        let mut indexes = vec![None; to_find.len()];
        values[letter_num] = *letter;
        indexes[letter_num] = Some((i, j));
        Match {
            current: letter_num,
            success: false,
            values,
            indexes,
        }
    }

    fn new_from_current(&self, letter_num: usize, letter: &char, i: usize, j: usize) -> Match {
        let mut new_vals = self.values.clone();
        let mut new_indexes = self.indexes.clone();
        new_vals[letter_num] = *letter;
        new_indexes[letter_num] = Some((i, j));
        Match {
            current: letter_num,
            success: new_vals.iter().all(|v| *v != '*'),
            values: new_vals,
            indexes: new_indexes,
        }
    }
}

fn find_total_words(data: &[Vec<char>], to_find: &str) -> Vec<Match> {
    let mut results = Vec::new();
    let max_j = data[0].len() as i64;
    let first_letter = to_find.chars().next().unwrap();
    let last_letter = to_find.chars().last().unwrap();

    // Store a list of ongoing matches for each letter in previous row
    // that we can attempt to extend with letters from the current row
    let mut prev_row_matches: Vec<Match> = Vec::new();

    for (i, row) in data.iter().enumerate() {
        // Store matches of last letter in row for horizontal matches
        let mut prev_letter_matches = Vec::new();
        let mut current_row_matches = Vec::new();

        for (j, letter) in row.iter().enumerate() {
            let mut current_letter_matches = Vec::new();
            // Get index of the letter in the word to find
            let letter_num = if let Some(num) = to_find.find(*letter) {
                num
            } else {
                continue;
            };
            // Always start a new match for the first and last letter
            if *letter == first_letter || *letter == last_letter {
                current_letter_matches.push(Match::new(to_find, letter_num, letter, i, j));
            }
            // Try to extend matches from the previous letter in current row
            let mut matches_to_check = Vec::new();
            // Also from above and diagonal in the previous row, if after first row
            if i > 0 {
                let mut js_to_check = Vec::new();
                for val in [j as i64, (j as i64) - 1, (j as i64) + 1] {
                    if val >= 0 && val < max_j {
                        js_to_check.push(val);
                    }
                }
                for m in prev_row_matches.iter() {
                    for jj in js_to_check.iter() {
                        if m.indexes[m.current] == Some((i - 1, *jj as usize)) {
                            matches_to_check.push(m);
                        }
                    }
                }
            }
            matches_to_check.extend(prev_letter_matches.iter());

            for m in matches_to_check {
                if let Some(new_match) = extend_match(m, to_find, letter, letter_num, i, j) {
                    if new_match.success {
                        results.push(new_match);
                    } else {
                        current_letter_matches.push(new_match);
                    }
                }
            }
            prev_letter_matches = current_letter_matches.clone();
            current_row_matches.extend(current_letter_matches);
        }
        prev_row_matches = current_row_matches;
    }
    results
}

fn extend_match(
    m: &Match,
    to_find: &str,
    letter: &char,
    letter_num: usize,
    i: usize,
    j: usize,
) -> Option<Match> {
    // Check if we can extend either side of the last letter we were on.
    // Should prevent direction changes since we start with stars and replace them
    // as we find letters
    let mut letters_available = vec![];
    // Store indexes of letters to check direction later
    let mut past_indexes = vec![];

    // Check if next letter is found
    if m.current + 1 < to_find.len() {
        if m.values[m.current + 1] == '*' {
            letters_available.push(to_find.chars().nth(m.current + 1).unwrap());
        } else {
            past_indexes.push(m.indexes[m.current + 1].unwrap());
        }
    }
    // Check if previous letter is found
    if m.current as i64 > 0 {
        if m.values[m.current - 1] == '*' {
            letters_available.push(to_find.chars().nth(m.current - 1).unwrap());
        } else {
            past_indexes.push(m.indexes[m.current - 1].unwrap());
        }
    }

    if !letters_available.contains(letter) {
        return None;
    }

    // 1 adjacent letter found & 1 space free, check gradient stays the same with the new letter
    if past_indexes.len() == 1 && letters_available.len() == 1 {
        past_indexes.push(m.indexes[m.current].unwrap());
        let past_i_diff = (past_indexes[1].0 as i64) - (past_indexes[0].0 as i64);
        let past_j_diff = (past_indexes[1].1 as i64) - (past_indexes[0].1 as i64);

        let new_i_diff = (i as i64) - (past_indexes[1].0 as i64);
        let new_j_diff = (j as i64) - (past_indexes[1].1 as i64);

        if past_i_diff != new_i_diff || past_j_diff != new_j_diff {
            return None;
        }
    }
    Some(m.new_from_current(letter_num, letter, i, j))
}
