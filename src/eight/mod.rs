use std::{collections::HashMap, thread::current, ops::ControlFlow};

use aocd::*;
use regex::Regex;

#[aocd(2023,8)]
pub fn one() {
    let map_regex = Regex::new(r"([A-Z]{3})").unwrap();
    let binding = input!();
    let input: Vec<_> = binding.lines().collect();
    let mut path: HashMap<&str, [&str; 2]> = HashMap::new();
    let instructions: Vec<_> = input.first().unwrap().chars().map(|c| {
        if c == 'L' {
            0_usize
        } else {
            1_usize
        }
    }).collect();
    input.iter().skip(2).for_each(|line| {
        let caps: Vec<_> = map_regex.captures_iter(line).collect();
        let start = caps.first().unwrap().get(0).unwrap().as_str();
        let left = caps.get(1).unwrap().get(0).unwrap().as_str();
        let right = caps.get(2).unwrap().get(0).unwrap().as_str();
        path.insert(start, [left, right]);
    });
    let mut num_steps = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        let current_step = *instructions.get(num_steps % instructions.len()).unwrap();
        current = path.get(current).unwrap()[current_step];
        num_steps += 1;
    }
    submit!(1, num_steps);
}

#[aocd(2023,8)]
pub fn two() {
    let map_regex = Regex::new(r"([\w]{3})").unwrap();
    let binding = input!();
    let input: Vec<_> = binding.lines().collect();
    let mut path: HashMap<&str, [&str; 2]> = HashMap::new();
    let mut current_nodes:Vec<&str> = Vec::new();
    let instructions: Vec<_> = input.first().unwrap().chars().map(|c| {
        if c == 'L' {
            0_usize
        } else {
            1_usize
        }
    }).collect();
    input.iter().skip(2).for_each(|line| {
        let caps: Vec<_> = map_regex.captures_iter(line).collect();
        let start = caps.first().unwrap().get(0).unwrap().as_str();
        let left = caps.get(1).unwrap().get(0).unwrap().as_str();
        let right = caps.get(2).unwrap().get(0).unwrap().as_str();
        path.insert(start, [left, right]);
        if start.ends_with("A") {
            current_nodes.push(start);
        }
    });
    let cycle_counts = current_nodes.iter().map(|current| {
        let mut current_node = *current;
        let mut i = 1;
        instructions.iter().cycle().try_for_each(|instruction| {
            current_node = path.get(current_node).unwrap()[*instruction];
            if current_node.ends_with("Z") {
                ControlFlow::Break(())
            } else {
                i += 1;
                ControlFlow::Continue(())
            }
        });
        i
    });
    let lcm: u64 = cycle_counts.fold(1, |acc, cycle_count| {
        println!("cycles: {cycle_count}");
        num::Integer::lcm(&acc, &cycle_count)
    });
    println!("part 2 num steps: {lcm}");
    submit!(2, lcm);
}