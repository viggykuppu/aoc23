use std::{cmp::Ordering, collections::HashMap, ops::Range};

use aocd::*;
use regex::Regex;

#[aocd(2023, 19)]
pub fn one() {
    let binding = input!();
    let chunks: Vec<_> = binding.split("\n\n").collect();

    let workflows = chunks[0];
    let parts = chunks[1];
    let workflows: HashMap<_,_> = parse_workflows(workflows);
    let part_regex = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
    let mut parts: Vec<_> = parts.lines().map(|line| {
        let caps: Vec<_> = part_regex.captures_iter(line).map(|c| c.extract::<4>()).collect();
        ("in", [caps[0].1[0].parse::<u32>().unwrap(),caps[0].1[1].parse::<u32>().unwrap(),caps[0].1[2].parse::<u32>().unwrap(),caps[0].1[3].parse::<u32>().unwrap()])
    }).collect();

    let sum: u32 = parts.iter_mut().map(|p| {
        while p.0 != "A" && p.0 != "R" {
            let workflow_to_apply = workflows.get(&p.0).unwrap();
            let mut case_matched = false;
            for case in &workflow_to_apply.cases {
                let test_value = p.1[case.part_idx];
                if test_value.cmp(&case.value) == case.comparison {
                    case_matched = true;
                    p.0 = case.next_workflow;
                    break;
                }
            }
            if !case_matched {
                p.0 = workflow_to_apply.fallback;
            }
        }
        if p.0 == "A" {
            p.1.iter().sum()
        } else {
            0
        }
    }).sum();
    submit!(1, sum);
}

#[aocd(2023, 19)]
pub fn two() {
    let binding = input!();
    let chunks: Vec<_> = binding.split("\n\n").collect();

    let workflows = chunks[0];
    let parts = chunks[1];
    let workflow_regex = Regex::new(r"(.*)\{(.*)}").unwrap();
    let instruction_regex = Regex::new(r"(.+)([<>])(\d+):(.*)").unwrap();
    let workflows: HashMap<_,_> = parse_workflows(workflows);
    let part_regex = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
    let mut parts: Vec<(&str, [Range<_>; 4])> = Vec::new();
    parts.push(("in", [1..4001, 1..4001, 1..4001, 1..4001]));
    let mut accepted_part_sum: u64 = 0;
    let mut idx = 1;
    while let Some(mut p) = parts.pop() {
        while p.0 != "A" && p.0 != "R" {
            let workflow_to_apply = workflows.get(&p.0).unwrap();
            for case in &workflow_to_apply.cases {
                let test_range = &mut p.1[case.part_idx];
                if test_range.contains(&case.value) {
                    // Split range and add new range matching case to array of parts
                    let (new_part_range, current_part_updated_range) = if case.comparison == Ordering::Greater {
                        ((case.value + 1..test_range.end), (test_range.start..case.value+1)) 
                    } else {
                        ((test_range.start..case.value), (case.value..test_range.end))
                    };
                    test_range.start = current_part_updated_range.start;
                    test_range.end = current_part_updated_range.end;

                    let mut new_part = p.1.clone();
                    new_part[case.part_idx] = new_part_range;
                    parts.push((case.next_workflow, new_part));
                }
            }
            p.0 = workflow_to_apply.fallback;
        }
        if p.0 == "A" {
            accepted_part_sum += p.1.iter().fold(1, |acc, pv| {
                // println!("range {:?}, {}", pv, pv.len());
                acc * pv.len() as u64
            });
        }
        idx += 1;
    }
    submit!(2, accepted_part_sum);
}   

fn parse_workflows(workflows: &str) -> HashMap<&str, Workflow> {
    let workflow_regex = Regex::new(r"(.*)\{(.*)}").unwrap();
    let instruction_regex = Regex::new(r"(.+)([<>])(\d+):(.*)").unwrap();
    HashMap::from_iter(workflows.lines().map(|line| {
        let caps: Vec<_> = workflow_regex.captures_iter(line).map(|c| c.extract::<2>()).collect();
        let label = caps[0].1[0];
        let instructions: Vec<_> = caps[0].1[1].split(',').collect();
        let cases: Vec<_> = instructions.iter().take(instructions.len() - 1).map(|i| {
            let caps: Vec<_> = instruction_regex.captures_iter(i).map(|c| c.extract::<4>()).collect();
            let part_idx = match caps[0].1[0] {
                "x" => 0,
                "m" => 1,
                "a" => 2,
                "s" => 3,
                _ => 4,
            };
            let comparison = match caps[0].1[1] {
                ">" => Ordering::Greater,
                "<" => Ordering::Less,
                _ => Ordering::Less,
            };
            let comparison_value = caps[0].1[2].parse::<u32>().unwrap();
            let next_workflow = caps[0].1[3];
            Case {
                part_idx: part_idx,
                value: comparison_value,
                comparison: comparison,
                next_workflow: next_workflow,
            }
        }).collect();
        let fallback = instructions.last().unwrap();
        (label, Workflow {
            label: label,
            cases: cases,
            fallback: fallback,
        })
    }))
}

#[derive(Debug)]
struct Workflow<'a> {
    label: &'a str,
    cases: Vec<Case<'a>>,
    fallback: &'a str,
}

#[derive(Debug)]
struct Case<'a> {
    part_idx: usize,
    value: u32,
    comparison: Ordering,
    next_workflow: &'a str,
}