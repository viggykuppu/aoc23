use aocd::*;
use regex::Regex;

#[aocd(2023,9)]
pub fn one() {
    let number_regex = Regex::new(r"-?\d+").unwrap();
    let binding = input!();
    let sum: i32 = binding.lines().map(|line| {
        let mut seq: Vec<Vec<i32>> = Vec::new();
        seq.push(number_regex.find_iter(line).map(|num| {
            num.as_str().parse::<i32>().unwrap()
        }).collect());
        let mut all_zeroes = false;
        let mut current_row_index = 0_usize;
        while !all_zeroes {
            all_zeroes = true;
            let current_row = seq.get(current_row_index).unwrap();
            let mut diffs: Vec<i32> = Vec::new();
            for i in 0..(current_row.len() - 1_usize) {
                let diff = current_row.get(i+1_usize).unwrap() - current_row.get(i).unwrap();
                all_zeroes = all_zeroes && (diff == 0);
                diffs.push(diff);
            }
            seq.push(diffs);
            current_row_index += 1_usize;
        }
        seq.reverse();
        let mut previous_diff = 0;
        seq.iter().skip(1).for_each(|diffs| {
            let last_element = diffs.last().unwrap();
            previous_diff = previous_diff + last_element;
        });
        previous_diff
    }).sum();
    submit!(1, sum);
}

#[aocd(2023,9)]
pub fn two() {
    let number_regex = Regex::new(r"-?\d+").unwrap();
    let binding = input!();
    let sum: i32 = binding.lines().map(|line| {
        let mut seq: Vec<Vec<i32>> = Vec::new();
        seq.push(number_regex.find_iter(line).map(|num| {
            num.as_str().parse::<i32>().unwrap()
        }).collect());
        let mut all_zeroes = false;
        let mut current_row_index = 0_usize;
        while !all_zeroes {
            all_zeroes = true;
            let current_row = seq.get(current_row_index).unwrap();
            let mut diffs: Vec<i32> = Vec::new();
            for i in 0..(current_row.len() - 1_usize) {
                let diff = current_row.get(i+1_usize).unwrap() - current_row.get(i).unwrap();
                all_zeroes = all_zeroes && (diff == 0);
                diffs.push(diff);
            }
            seq.push(diffs);
            current_row_index += 1_usize;
        }
        seq.reverse();
        let mut previous_diff = 0;
        seq.iter().skip(1).for_each(|diffs| {
            let first_element = diffs.first().unwrap();
            previous_diff = first_element - previous_diff;
        });
        previous_diff
    }).sum();
    submit!(2, sum);
}