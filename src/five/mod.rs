use aocd::*;
use std::ops::{ControlFlow, Range};

use regex::Regex;

#[aocd(2023,5)]
pub fn one() {
    let input = input!();

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
    submit!(1, seeds.iter().min().unwrap());
}

#[aocd(2023,5)]
pub fn two() {
    let input = input!();

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
    let mut new_seeds: Vec<Range<u64>> = Vec::new();
    maps_list.iter().for_each(|maps| {
        let mut i = 0;

        while i < seeds.len() {
            maps.iter().try_for_each(|map| {
                // let (new_seed_one, new_seed_two) = compare_seed_to_map(seed, map);
                let seed = seeds.get_mut(i).unwrap();
                if compare_seed_to_map(seed, map, &mut new_seeds) {
                    while let Some(new_seed) = new_seeds.pop() {
                        seeds.push(new_seed);
                    }
                    return ControlFlow::Break(())
                }
                ControlFlow::Continue(())
            });
            i += 1;
        }
    });
    submit!(2, seeds.iter().map(|s| s.start).min().unwrap());
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
            seed.start = ((seed.start as i64) + map.1) as u64;
            seed.end = ((seed.end as i64) + map.1) as u64;
            return true;
        } else {
            // Case 2: Seed lower bound in map range, but not upper bound
            // m1 s1 m2 s2
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
            return true;
        }
    } else if map.0.contains(&(seed.end-1)) {
        // Case 3: Seed upper bound in map range, but not lower bound
        // s1 m1 s2 m2
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
        return true;
    } else if seed.contains(&map.0.start) && (seed.contains(&(map.0.end-1)) || seed.end == map.0.end) {
        // Case 4: Map range entirely in seed
        // s1 m1 m2 s2
        let new_seed_one = seed.start..map.0.start;
        let new_seed_two = map.0.end..seed.end;
        seed.start = ((map.0.start as i64) + map.1) as u64;
        seed.end = ((map.0.end as i64) + map.1) as u64;
        new_seeds.push(new_seed_one);
        new_seeds.push(new_seed_two);
        return true;
    }
    false
}

#[aocd(2023,5)]
pub fn two_brute_force() {
    let input = input!();

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
    submit!(2, min);
}