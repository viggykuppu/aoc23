use std::collections::HashMap;

use aocd::*;
use regex::Regex;

#[aocd(2023,15)]
pub fn one() {
    let binding = input!();
    let output: u32 = binding.split(',').fold(0, |acc, step| {
        acc + hash(step) as u32
    });
    submit!(1, output)
}

#[aocd(2023,15)]
pub fn two() {
    let binding = input!();
    let mut buckets: HashMap<u8, Vec<(&str, u8)>> = HashMap::new();
    let step_regex = Regex::new(r"(.*)([=-])(\d?)").unwrap();
    binding.split(',').for_each(|step| {
        let caps: Vec<_> = step_regex.captures_iter(step).map(|c| c.extract::<3>()).collect();
        let token = caps[0].1[0];
        let op = caps[0].1[1];
        let hash = hash(caps[0].1[0]);
        let val = caps[0].1.get(2);
        if let Some(e) =  buckets.get_mut(&hash) {
            if op == "-" {
                let mut element_idx = -1;
                for (i, element) in e.iter().enumerate() {
                    if element.0 == token {
                        element_idx = i as i32;
                    }
                }
                if element_idx != -1 {
                    e.remove(element_idx as usize);
                }
            } else if op == "=" {
                let value = val.unwrap().parse::<u8>().unwrap();
                let mut found_same_token = false;
                for element in e.iter_mut() {
                    if element.0 == token {
                        element.1 = value;
                        found_same_token = true;
                        break;
                    }
                }
                if !found_same_token {
                    e.push((token, value));
                }
            }
        } else if let Some(v) = val {
            if op == "=" {
                buckets.insert(hash, vec![(token, v.parse::<u8>().unwrap())]);
            }
        }
    });
    // dbg!(&buckets);
    let output: u64 = buckets.keys().fold(0, |acc, key| {
        let contents = &buckets[key];
        acc + contents.iter().enumerate().fold(0, |acc2, (i, element)| {
            acc2 + ((*key as u64 + 1) * (i + 1) as u64 * element.1 as u64)
        })
    });
    submit!(2, output)
}

fn hash(s: &str) -> u8 {
    s.chars().fold(0, |acc2, val| {
        (acc2 + val as u8) * 17
    })
}