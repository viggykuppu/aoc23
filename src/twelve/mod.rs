use std::collections::HashMap;

use aocd::*;
use regex::Regex;

#[aocd(2023,12)]
pub fn one() {
    let binding = input!();
    let total_valid_configs: usize = binding.lines().map(|line| {
        // let damaged_groups = get_damaged_groups(line);
        let damaged_groups: Vec<usize> = line.split(' ').collect::<Vec<&str>>()[1].split(',').map(|n| {
            n.parse::<usize>().unwrap()
        }).collect();
        let damaged_regex = Regex::new(r"#+").unwrap();
        let working_regex = Regex::new(r"\.+").unwrap();
        efficient_search(&line.split(' ').collect::<Vec<&str>>()[0], &damaged_groups.as_slice(), 0, 0, &damaged_regex, &working_regex)
    }).sum();
    println!("total valid configs: {}", total_valid_configs);
    submit!(1, total_valid_configs);
}

#[aocd(2023,12)]
pub fn two() {
    let binding = input!();
    let mut i = 1;
    let total_valid_configs: usize = binding.lines().map(|line| {
        let repeat_count = 5_usize;
        let mut base_line = line.split(' ').collect::<Vec<&str>>()[0].to_string();
        base_line.push('?');
        base_line = base_line.repeat(repeat_count);
        base_line.pop();
        let mut truth = line.split(' ').collect::<Vec<&str>>()[1].to_string();
        truth.push(',');
        truth = truth.repeat(repeat_count);
        truth.pop();
        // println!("{base_line} {truth}");
        let mut damaged_groups: Vec<usize> = line.split(' ').collect::<Vec<&str>>()[1].split(',').map(|n| {
            n.parse::<usize>().unwrap()
        }).collect();
        damaged_groups = damaged_groups.repeat(repeat_count);
        let damaged_regex = Regex::new(r"#+").unwrap();
        let working_regex = Regex::new(r"\.+").unwrap();
        let total_broken: usize = damaged_groups.iter().sum();
        let mut map = HashMap::new();
        let test_total = r(&base_line, &damaged_groups, base_line.len() as isize - 1, damaged_groups.len() as isize - 1, &damaged_regex, &working_regex, &mut map);
        i += 1;
        test_total
    }).sum();
    println!("total valid configs: {}", total_valid_configs);
    submit!(2, total_valid_configs);
}

fn r(s: &str, b: &Vec<usize>, s_idx: isize, b_idx: isize, damaged_regex: &Regex, working_regex: &Regex, map: &mut HashMap<(isize, isize), usize>) -> usize {
    let u_s_idx = s_idx as usize;
    if let Some(v) = map.get(&(s_idx, b_idx)) {
        return *v;
    }
    if b_idx < 0 {
        if s_idx < 0 {
            return 1_usize;
        }
        for i in 0..=u_s_idx {
            if &s[i..=i] == "#" {
                return 0_usize;
            }
        }
        return 1_usize;
    }
    if s_idx < 0 {
        return 0_usize;
    }
    if s_idx < 4 {
        let val = efficient_search(&s[0..=u_s_idx], &b[0..=(b_idx as usize)], 0, 0, damaged_regex, working_regex);
        map.insert((s_idx, b_idx), val);
        return val;
    } else if &s[u_s_idx..(u_s_idx+1)] == "." {
        let val = r(s, b, s_idx.saturating_sub(1), b_idx, damaged_regex, working_regex, map);
        map.insert((s_idx, b_idx), val);
        return val;
    } else if &s[u_s_idx..(u_s_idx+1)] == "#" {
        // pop last broken group val
        let pop = b[b_idx as usize];
        let mut valid_run = true;
        let mut run_length = pop + 1;
        for i in 0..pop {
            if (s_idx as isize - i as isize) < 0 {
                valid_run = false;
                break;
            }
            let test_char = &s[(u_s_idx.saturating_sub(i))..=(u_s_idx.saturating_sub(i))];
            if test_char == "." {
                valid_run = false;
                break;
            }
        }
        if (s_idx as isize - pop as isize) < 0 {
            run_length = pop;
        } else if &s[(u_s_idx.saturating_sub(pop))..=(u_s_idx.saturating_sub(pop))] != "#" {
            run_length = pop + 1;
        } else {
            valid_run = false;
        }
        if valid_run {
            let val = r(s, b, s_idx.saturating_sub(run_length as isize), b_idx.saturating_sub(1), damaged_regex, working_regex, map);
            map.insert((s_idx, b_idx), val);
            return val;
        } else {
            return 0;
        }
    } else {
        let pop = b[b_idx as usize];
        let mut valid_run = true;
        let mut run_length = pop + 1;
        for i in 0..pop {
            if (s_idx as isize - i as isize) < 0 {
                valid_run = false;
                break;
            }
            let test_char = &s[(u_s_idx.saturating_sub(i))..=(u_s_idx.saturating_sub(i))];
            if test_char == "." {
                valid_run = false;
                break;
            }
        }
        if (s_idx as isize - pop as isize) < 0 {
            run_length = pop;
        } else if &s[(u_s_idx.saturating_sub(pop))..=(u_s_idx.saturating_sub(pop))] != "#" {
            run_length = pop + 1;
        } else {
            valid_run = false;
        }
        let val2 = if valid_run {
            r(s, b, s_idx.saturating_sub(run_length as isize), b_idx.saturating_sub(1), damaged_regex, working_regex, map)
        } else {
            0
        };
        let val1 = r(s, b, s_idx.saturating_sub(1), b_idx, damaged_regex, working_regex, map);
        map.insert((s_idx, b_idx), val1+val2);
        return val1 + val2;
    }
}

fn efficient_search(test_line: &str, damaged_groups: &[usize], idx: usize, num_broken: usize, damaged_regex: &Regex, working_regex: &Regex) -> usize {
    let mut total = 0_usize;
    let mut test_line = test_line;
    // end cases
    if damaged_groups.len() == 0 {
        if idx < test_line.len() && damaged_regex.is_match(&test_line[idx..test_line.len()]) {
            return 0_usize;
        }
        // valid config
        return 1_usize;
    } else if idx >= test_line.len() {
        // invalid config because we're at end of line
        // and we haven't made all the groups
        // println!("returning 0");
        return 0_usize;
    }
    let current_group = damaged_groups[0];
    let mut current_character = &test_line[idx..idx+1];

    let mut is_unknown = false;
    // if this is a ? then pick it as a . or a # and continue
    if current_character == "?" {
        // TODO
        is_unknown = true;
    }
    let binding1: String;
    if is_unknown {
        current_character = &".";
        binding1 = test_line.chars().enumerate().map(|(i, c)| {
            if i == idx {
                '.'
            } else {
                c
            }
        }).collect::<String>();
        test_line = &binding1;
    }
    if current_character == "." {
        // println!(". case test line: {}; idx: {idx}, damaged_groups: {:?}", test_line, damaged_groups);
        // This is a dot, so we terminate our broken part string
        let m = working_regex.find(&test_line[idx..test_line.len()]).unwrap();
        if num_broken == 0 {
            // This is a string of .s with no damaged in front so skip to end of this run of working parts
            total += efficient_search(test_line, damaged_groups, idx + m.end(), 0, damaged_regex, working_regex);
        } else {
            // This is a string of .s with damaged in front so compare the damaged run to what the next damaged run should be
            // Also skip to the end of this run of working parts
            if num_broken == current_group {
                // run is valid config so far
                // continue with our search
                total += efficient_search(test_line, &damaged_groups[1..damaged_groups.len()], idx + m.end(), 0, damaged_regex, working_regex);
            } else {
                // run is invalid config
                total +=  0_usize;
            }
        }
    }
    let binding2: String;
    if is_unknown {
        current_character = &"#";
        binding2 = test_line.chars().enumerate().map(|(i, c)| {
            if i == idx {
                '#'
            } else {
                c
            }
        }).collect::<String>();
        test_line = &binding2;
    }
    if current_character == "#" {
        // println!("# case test line: {}; idx: {idx}, damaged_groups: {:?}; num_broken {}", test_line, damaged_groups, num_broken);
        // Find where this damaged part run ends
        let m = damaged_regex.find(&test_line[idx..test_line.len()]).unwrap();
        let damaged_run_length = m.len() + num_broken;

        // If this run is too long then it's not a valid config
        if damaged_run_length > current_group {
            total +=  0_usize;
        }
        if idx+m.end() == test_line.len() || &test_line[(idx+m.end())..(idx+m.end()+1)] == "." {
            // This is a full damaged part run and must match the first damaged group length else this config is invalid
            if damaged_run_length != current_group {
                total +=  0_usize;
            } else {
                // We found a valid run and want to continue checking the rest of the string
                // This also forces the next character after the group to be a . so can also skip ahead another character
                total +=  efficient_search(test_line, &damaged_groups[1..damaged_groups.len()], idx + m.end()+1, 0, damaged_regex, working_regex);
            }
        } else if &test_line[(idx+m.end())..(idx+m.end()+1)] == "?" {
            // This run is followed by a ?
            // continue our search noting we've found m.len() damaged in a current run
            // println!("idx: {idx}");
            total +=  efficient_search(test_line, damaged_groups, idx + m.len(), m.len()+num_broken, damaged_regex, working_regex);
        }
    }
    
    return total;
}

fn is_valid_config(damaged_groups: &Vec<usize>, test_line: &str) -> bool {
    let possible_damaged_groups = get_damaged_groups(test_line);
    return damaged_groups == &possible_damaged_groups;
}

fn get_damaged_groups(line: &str) -> Vec<usize> {
    Regex::new(r"#+").unwrap().find_iter(line).map(|m| m.len()).collect()
}