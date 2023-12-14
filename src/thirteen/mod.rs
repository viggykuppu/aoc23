use std::collections::{HashMap, HashSet};

use aocd::*;

#[aocd(2023,13)]
pub fn one() {
    let mut total = 0;
    let binding = input!();
    binding.split("\n\n").for_each(|mirror| {
        total += get_mirror_value(&mirror.split('\n').collect(), 0);
    });

    submit!(1, total);
}

#[aocd(2023,13)]
pub fn two() {
    let mut total = 0;
    let binding = input!();
    binding.split("\n\n").for_each(|mirror| {
        total += get_mirror_value(&mirror.split('\n').collect(), 1);
    });

    submit!(2, total);
}

fn get_mirror_value(mirror: &Vec<&str>, number_smudges: usize) -> usize {
    let rows: Vec<Vec<u32>> = mirror.iter().map(|line| line.chars().map(|c| {
        if c == '.' {
            0
        } else {
            1
        }
    }).collect()).collect();
    let val1 = get_smudged_mirror_values(&rows, 100, number_smudges);

    let mut transpose: Vec<Vec<u32>> = Vec::new();
    for i in 0..rows[0].len() {
        transpose.push(Vec::new());
    }

    for i in 0..rows.len() {
        for j in 0..rows[0].len() {
            transpose[j].push(rows[i][j]);
        }
    }
    let val2 = get_smudged_mirror_values(&transpose, 1, number_smudges);
    val1 + val2
}

fn get_smudged_mirror_values(rows: &Vec<Vec<u32>>, scale_factor: usize, target_difference: usize) -> usize {
    for test_pivot in 0..rows.len()-1 {
        let mut offset = 0;
        let lower_index = test_pivot;
        let upper_index = test_pivot + 1;
        let mut total_mirror_difference: usize = 0;
        while offset + upper_index < rows.len() && lower_index as i32 - offset as i32 >= 0{
           total_mirror_difference += xor_rows(&rows[lower_index - offset], &rows[upper_index + offset]);
           offset += 1;
        }
        // println!("test pivot {test_pivot} has mirror difference {total_mirror_difference}");
        if total_mirror_difference == target_difference {
            // println!("test pivot is valid smudged mirror {test_pivot}, lower index is {}", lower_index);
            return (lower_index+1)*scale_factor;
        }
    }
    0
}

fn xor_rows(row_1: &Vec<u32>, row_2: &Vec<u32>) -> usize {
    row_1.iter().zip(row_2.iter()).map(|(c1, c2)| {
        (c1 ^ c2) as usize
    }).sum()
}