use std::{collections::HashMap, ops::Range};

use aocd::*;
use regex::Regex;

#[aocd(2023, 3)]
pub fn one() {
    let input = input!();
    let char_matrix: Vec<_> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();
    let sum = input.lines().enumerate().fold(0, |acc, (i, line)| {
        let number_regex = Regex::new(r"(\d+)").unwrap();
        acc + number_regex.find_iter(line).fold(0, |part_total, n| {
            let number_range = n.start()..n.end();
            if is_symbol_adjacent(i, number_range, &char_matrix) {
                let engine_part_number = n.as_str().parse::<u32>().unwrap();
                part_total + engine_part_number
            } else {
                part_total
            }
        })
    });
    submit!(1, sum);
}

#[aocd(2023, 3)]
pub fn two() {
    let input = input!();
    let char_matrix: Vec<_> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();
    let mut symbol_adjacency_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let number_regex = Regex::new(r"(\d+)").unwrap();
        for n in number_regex.find_iter(line) {
            let number_range = n.start()..n.end();
            let engine_part_number = n.as_str().parse::<u32>().unwrap();
            calculate_symbol_adjacencies(
                i,
                number_range,
                &char_matrix,
                engine_part_number,
                &mut symbol_adjacency_map,
            );
        }
    }
    let sum: u64 = symbol_adjacency_map.keys().fold(0, |acc, key| {
        let adjacents: &Vec<u32> = symbol_adjacency_map.get(key).unwrap();
        if adjacents.len() == 2 {
            acc + (adjacents[0] * adjacents[1]) as u64
        } else {
            acc
        }
    });
    submit!(2, sum);
}

fn is_symbol_adjacent(row_num: usize, range: Range<usize>, char_matrix: &Vec<Vec<char>>) -> bool {
    let dims = (char_matrix.len() as isize, char_matrix[0].len() as isize);
    let symbol_regex = Regex::new(r"[^\d\.\n]").unwrap();
    for i in range {
        for x in -1..=1 {
            for y in -1..=1 {
                if is_valid_index((row_num as isize) + x, (i as isize) + y, dims) {
                    let x_idx = ((row_num as isize) + x) as usize;
                    let y_idx = ((i as isize) + y) as usize;
                    if symbol_regex.is_match(&char_matrix[x_idx][y_idx].to_string()) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn calculate_symbol_adjacencies(
    row_num: usize,
    range: Range<usize>,
    char_matrix: &Vec<Vec<char>>,
    part_number: u32,
    symbol_adjacency_map: &mut HashMap<(usize, usize), Vec<u32>>,
) {
    let dims = (char_matrix.len() as isize, char_matrix[0].len() as isize);
    let symbol_regex = Regex::new(r"[^\d\.\n]").unwrap();
    for i in range {
        for x in -1..=1 {
            for y in -1..=1 {
                if is_valid_index((row_num as isize) + x, (i as isize) + y, dims) {
                    let x_idx = ((row_num as isize) + x) as usize;
                    let y_idx = ((i as isize) + y) as usize;
                    if symbol_regex.is_match(&char_matrix[x_idx][y_idx].to_string()) {
                        match symbol_adjacency_map.get_mut(&(x_idx, y_idx)) {
                            None => {
                                symbol_adjacency_map.insert((x_idx, y_idx), vec![part_number]);
                            }
                            Some(adjacents) => {
                                adjacents.push(part_number);
                            }
                        }
                        return;
                    }
                }
            }
        }
    }
}

fn is_valid_index(row_num: isize, col_num: isize, dims: (isize, isize)) -> bool {
    row_num >= 0 && row_num < dims.0 && col_num >= 0 && col_num < dims.1
}
