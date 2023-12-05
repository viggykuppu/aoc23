use core::num;
use std::ops::{ControlFlow, Range};

use regex::Regex;

pub fn one() {
    let input = crate::lib::read_input("src/five/input.txt");

    let mut lines_iter = input.lines();
    let seeds_line = lines_iter.next().unwrap();
    let number_regex = Regex::new(r"\d+").unwrap();
    let header_regex = Regex::new("[a-zA-z]").unwrap();
    let mut seeds: Vec<_> = number_regex
        .find_iter(seeds_line)
        .map(|seed| seed.as_str().parse::<u64>().unwrap())
        .collect();
    let mut maps_list: Vec<Vec<(Range<u64>, i64)>> = Vec::new();
    let mut map_idx = 0;
    lines_iter.nth(1);
    lines_iter.filter(|line| !line.is_empty()).for_each(|line| {
        if maps_list.get(map_idx).is_none() {
            maps_list.push(Vec::new());
        }
        if header_regex.find(line).is_some() {
            map_idx += 1;
            return;
        }
        let current_map_vector = maps_list.get_mut(map_idx).unwrap();
        let vals: Vec<_> = number_regex
            .find_iter(line)
            .map(|m| m.as_str().parse::<u64>().unwrap())
            .collect();
        let (dest_base, source_base, range) = (
            *vals.first().unwrap(),
            *vals.get(1).unwrap(),
            *vals.get(2).unwrap(),
        );
        current_map_vector.push((
            source_base..(source_base + range),
            dest_base as i64 - source_base as i64,
        ));
    });
    dbg!(&maps_list.get(4).unwrap());

    seeds.iter_mut().enumerate().for_each(|(i, seed)| {
        maps_list.iter().for_each(|maps| {
            maps.iter().try_for_each(|map| {
                if map.0.contains(seed) {
                    *seed = ((*seed as i64) + map.1) as u64;
                    return ControlFlow::Break(())
                }
                ControlFlow::Continue(())
            });
        });
    });
    println!("minimum location is: {}", seeds.iter().min().unwrap());
}

pub fn two() {
    let input = crate::lib::read_input("src/five/input.txt");

    let mut lines_iter = input.lines();
    let seeds_line = lines_iter.next().unwrap();
    let number_regex = Regex::new(r"\d+").unwrap();
    let seed_regex = Regex::new(r"(\d+ \d+)").unwrap();
    let header_regex = Regex::new("[a-zA-z]").unwrap();
    let mut seeds: Vec<_> = seed_regex
        .find_iter(seeds_line)
        .map(|seed_pair| {
            let seeds_split: Vec<_> = seed_pair.as_str().split(' ').collect();
            let seed1 = seeds_split.first().unwrap().parse::<u64>().unwrap();
            let offset = seeds_split.get(1).unwrap().parse::<u64>().unwrap();
            seed1..(seed1 + offset)
        })
        .collect();
    let mut maps_list: Vec<Vec<(Range<u64>, i64)>> = Vec::new();
    let mut map_idx = 0;
    lines_iter.nth(1);
    lines_iter.filter(|line| !line.is_empty()).for_each(|line| {
        if maps_list.get(map_idx).is_none() {
            maps_list.push(Vec::new());
        }
        if header_regex.find(line).is_some() {
            map_idx += 1;
            return;
        }
        let current_map_vector = maps_list.get_mut(map_idx).unwrap();
        let vals: Vec<_> = number_regex
            .find_iter(line)
            .map(|m| m.as_str().parse::<u64>().unwrap())
            .collect();
        let (dest_base, source_base, range) = (
            *vals.first().unwrap(),
            *vals.get(1).unwrap(),
            *vals.get(2).unwrap(),
        );
        current_map_vector.push((
            source_base..(source_base + range),
            dest_base as i64 - source_base as i64,
        ));
    });

    let mut min = u64::MAX;

    seeds.iter_mut().enumerate().for_each(|(i, seed)| {
        for i in seed {
            let mut s = i;
            maps_list.iter().for_each(|maps| {
                maps.iter().try_for_each(|map| {
                    if map.0.contains(&s) {
                        s = ((s as i64) + map.1) as u64;
                        if s == 0 {
                            println!("Seed {i} became 0!!");
                        }
                        return ControlFlow::Break(())
                    }
                    ControlFlow::Continue(())
                });
            });
            if s < min {
                min = s;
            }
        }
    });
    println!("minimum location is: {}", min);
}

pub fn two_special() {
    let input = crate::lib::read_input("src/five/input.txt");

    let mut lines_iter = input.lines();
    let seeds_line = lines_iter.next().unwrap();
    let number_regex = Regex::new(r"\d+").unwrap();
    let seed_regex = Regex::new(r"(\d+ \d+)").unwrap();
    let header_regex = Regex::new("[a-zA-z]").unwrap();
    let mut seeds: Vec<_> = seed_regex
        .find_iter(seeds_line)
        .map(|seed_pair| {
            let seeds_split: Vec<_> = seed_pair.as_str().split(' ').collect();
            let seed1 = seeds_split.first().unwrap().parse::<u64>().unwrap();
            let offset = seeds_split.get(1).unwrap().parse::<u64>().unwrap();
            seed1..(seed1 + offset)
        })
        .collect();
    let mut maps_list: Vec<Vec<(Range<u64>, i64)>> = Vec::new();
    let mut map_idx = 0;
    lines_iter.nth(1);
    lines_iter.filter(|line| !line.is_empty()).for_each(|line| {
        if maps_list.get(map_idx).is_none() {
            maps_list.push(Vec::new());
        }
        if header_regex.find(line).is_some() {
            map_idx += 1;
            return;
        }
        let current_map_vector = maps_list.get_mut(map_idx).unwrap();
        let vals: Vec<_> = number_regex
            .find_iter(line)
            .map(|m| m.as_str().parse::<u64>().unwrap())
            .collect();
        let (dest_base, source_base, range) = (
            *vals.first().unwrap(),
            *vals.get(1).unwrap(),
            *vals.get(2).unwrap(),
        );
        current_map_vector.push((
            source_base..(source_base + range),
            dest_base as i64 - source_base as i64,
        ));
        // println!("{line}");
        // dbg!(current_map_vector);
    });
    // dbg!(&seeds);
    let mut new_seeds: Vec<Range<u64>> = Vec::new();
    maps_list.iter().for_each(|maps| {
        let total_seeds = seeds
            .iter()
            .fold(0, |acc, seed| acc + seed.end - seed.start);
        println!(
            "Total seeds: {total_seeds}; total number of seed groups: {}",
            seeds.len()
        );
        // dbg!(&seeds);
        for i in 0..seeds.len() {
            maps.iter().try_for_each(|map| {
                // let (new_seed_one, new_seed_two) = compare_seed_to_map(seed, map);
                let seed = seeds.get_mut(i).unwrap();
                if compare_seed_to_map(seed, map, &mut new_seeds) {
                    while let Some(new_seed) = new_seeds.pop() {
                        if new_seed.start == 0 {
                            println!("0 new seed");
                            dbg!(&new_seed);
                        }
                        seeds.push(new_seed);
                    }
                    return ControlFlow::Break(())
                }
                ControlFlow::Continue(())
            });
        }
        // dbg!(&seeds);
    });

    println!(
        "minimum location is: {}",
        seeds.iter().map(|s| s.start).min().unwrap()
    );
}

fn compare_seed_to_map(
    seed: &mut Range<u64>,
    map: &(Range<u64>, i64),
    new_seeds: &mut Vec<Range<u64>>,
) -> bool {
    if map.0.contains(&seed.start) {
        if map.0.contains(&seed.end) || map.0.end == seed.end {
            // Case 1: Seed entirely in map range
            // m1 s1 s2 m2
            // println!("Case 1");
            seed.start = ((seed.start as i64) + map.1) as u64;
            seed.end = ((seed.end as i64) + map.1) as u64;
            if seed.start == 0 {
                println!("0 seed case 1");
                dbg!(seed);
            }
            return true;
        } else {
            // Case 2: Seed lower bound in map range, but not upper bound
            // m1 s1 m2 s2
            // println!("Case 2");
            // m2..s2
            let new_seed_start = map.0.end;
            let new_seed_end = seed.end;
            let new_seed = new_seed_start..new_seed_end;
            new_seeds.push(new_seed);

            // s1..m2
            let seed_start = ((seed.start as i64) + map.1) as u64;
            let seed_end = seed_start + (map.0.end - seed.start);
            seed.start = seed_start;
            seed.end = seed_end;
            if seed.start == 0 {
                println!("0 seed case 2");
                dbg!(seed);
            }
            return true;
        }
    } else if map.0.contains(&seed.end) {
        // Case 3: Seed upper bound in map range, but not lower bound
        // s1 m1 s2 m2
        // println!("Case 3");
        // s1..m1
        // let new_seed_start = ((map.0.start as i64) + map.1) as u64;
        // let new_seed_end = new_seed_start + (seed.end - map.0.start);
        let orig_start = seed.start;
        let orig_end = seed.end;
        let new_seed_start = seed.start;
        let new_seed_end = new_seed_start + (map.0.start - seed.start);
        let new_seed = new_seed_start..new_seed_end;
        new_seeds.push(new_seed);
        // m1..s2
        let seed_start = ((map.0.start as i64) + map.1) as u64;
        let seed_end = seed_start + (seed.end - map.0.start);
        seed.start = seed_start;
        seed.end = seed_end;
        if seed.start == 0 {
            println!("0 seed case 3; original:{}..{}", orig_start, orig_end);
            dbg!(seed);
        }
        return true;
    } else if seed.contains(&map.0.start) && (seed.contains(&map.0.end) || seed.end == map.0.end) {
        // Case 4: Map range entirely in seed
        // s1 m1 m2 s2
        // println!("Case 4");
        let new_seed_one = seed.start..map.0.start;
        let new_seed_two = map.0.end..seed.end;
        seed.start = ((map.0.start as i64) + map.1) as u64;
        seed.end = ((map.0.end as i64) + map.1) as u64;
        new_seeds.push(new_seed_one);
        new_seeds.push(new_seed_two);
        if seed.start == 0 {
            println!("0 seed case 4");
            dbg!(seed);
        }
        return true;
    }
    false
}
