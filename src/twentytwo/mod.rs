use std::{collections::{HashMap, HashSet}, ops::{RangeInclusive}};

use aocd::*;
use itertools::Itertools;
use regex::Regex;

#[aocd(2023, 22)]
pub fn one() {
    let binding = input!();
    let mut bricks: Vec<_> = build_bricks(binding);

    let_bricks_fall(&mut bricks);

    let mut brick_tops_at_z: HashMap<usize, Vec<&Brick>> = HashMap::new();
    let mut brick_bottoms_at_z: HashMap<usize, Vec<&Brick>> = HashMap::new();
    let mut single_brick_supports: HashSet<usize> = HashSet::new();
    for brick in &bricks {
        if let None = brick_bottoms_at_z.get(brick.position.2.start()) {
            brick_bottoms_at_z.insert(*brick.position.2.start(), Vec::new());
        }
        if let None = brick_tops_at_z.get(brick.position.2.end()) {
            brick_tops_at_z.insert(*brick.position.2.end(), Vec::new());
        }
        brick_bottoms_at_z.get_mut(brick.position.2.start()).unwrap().push(brick);
        brick_tops_at_z.get_mut(brick.position.2.end()).unwrap().push(brick);
    }
    // println!("{:?}", brick_bottoms_at_z);
    for z in brick_bottoms_at_z.keys().sorted().skip(1) {
        // println!("{}",z);
        let bricks_to_check = brick_bottoms_at_z.get(z).unwrap();
        let below_bricks = brick_tops_at_z.get(&(z-1)).unwrap();
        for brick_to_check in bricks_to_check {
            let mut supporting_bricks = Vec::new();
            for below_brick in below_bricks {
                if brick_to_check.overlaps_xy(below_brick) {
                    supporting_bricks.push(below_brick);
                }
            }
            if supporting_bricks.len() == 1 {
                single_brick_supports.insert(supporting_bricks.first().unwrap().label);
            }
            if supporting_bricks.len() == 0 {
                panic!();
            }
        }
    }

    submit!(1, bricks.len() - single_brick_supports.len());
}

#[aocd(2023,22)]
pub fn two() {
    let binding = input!();
    let mut bricks: Vec<_> = build_bricks(binding);

    let_bricks_fall(&mut bricks);

    let mut brick_tops_at_z: HashMap<usize, Vec<&Brick>> = HashMap::new();
    let mut brick_bottoms_at_z: HashMap<usize, Vec<&Brick>> = HashMap::new();
    let mut single_brick_supports: HashSet<(usize, usize)> = HashSet::new();
    for brick in &bricks {
        if let None = brick_bottoms_at_z.get(brick.position.2.start()) {
            brick_bottoms_at_z.insert(*brick.position.2.start(), Vec::new());
        }
        if let None = brick_tops_at_z.get(brick.position.2.end()) {
            brick_tops_at_z.insert(*brick.position.2.end(), Vec::new());
        }
        brick_bottoms_at_z.get_mut(brick.position.2.start()).unwrap().push(brick);
        brick_tops_at_z.get_mut(brick.position.2.end()).unwrap().push(brick);
    }
    let mut supported_by_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    for z in brick_bottoms_at_z.keys().sorted().skip(1) {
        let bricks_to_check = brick_bottoms_at_z.get(z).unwrap();
        let below_bricks = brick_tops_at_z.get(&(z-1)).unwrap();
        for brick_to_check in bricks_to_check {
            supported_by_map.insert(brick_to_check.label, HashSet::new());
            let supporters = supported_by_map.get_mut(&brick_to_check.label).unwrap();
            let mut supporting_bricks = Vec::new();
            for below_brick in below_bricks {
                if brick_to_check.overlaps_xy(below_brick) {
                    supporting_bricks.push(below_brick);
                    supporters.insert(below_brick.label);
                }
            }
            if supporting_bricks.len() == 1 {
                single_brick_supports.insert((supporting_bricks.first().unwrap().label, *brick_to_check.position.2.start()));
            }
            if supporting_bricks.len() == 0 {
                panic!();
            }
        }
    }
    let mut total_fallen_bricks = 0;
    for single_brick_support in single_brick_supports.iter() {
        let brick_to_remove = single_brick_support.0;
        let first_row_with_bricks_falling = single_brick_support.1;
        let mut removed_bricks = HashSet::from([brick_to_remove]);
        for z in brick_bottoms_at_z.keys().filter(|k| **k >= first_row_with_bricks_falling).sorted() {
            let potential_fallers = brick_bottoms_at_z.get(z).unwrap();
            for b in potential_fallers {
                if supported_by_map.get(&b.label).unwrap().is_subset(&removed_bricks) {
                    removed_bricks.insert(b.label);
                }
            }
        }
        total_fallen_bricks += removed_bricks.len() - 1;
    }
    submit!(2, total_fallen_bricks)
}

fn build_bricks(input: String) -> Vec<Brick> {
    let brick_regex = Regex::new(r"(\d+,\d+,\d+)~(\d+,\d+,\d+)").unwrap();
    input.lines().enumerate().map(|(i, line)| {
        let caps: Vec<_> = brick_regex.captures_iter(line).collect();
        let caps = caps.get(0).unwrap();
        let first: Vec<_> = caps.get(1).unwrap().as_str().split(",").map(|d| d.parse::<usize>().unwrap()).collect();
        let second: Vec<_> = caps.get(2).unwrap().as_str().split(",").map(|d| d.parse::<usize>().unwrap()).collect();
        Brick {
            label: i,
            position: (first[0]..=second[0],first[1]..=second[1],first[2]..=second[2])
        }
    }).collect()
}

fn let_bricks_fall(bricks: &mut Vec<Brick>) {
    let max_x = bricks.iter().map(|b| b.position.0.end()).max().unwrap().clone();
    let max_y = bricks.iter().map(|b| b.position.1.end()).max().unwrap().clone();
    bricks.sort_by(|b1, b2| {
        b1.position.2.start().cmp(&b2.position.2.start())
    });
    // println!("{:?}", bricks);
    let mut height_map: HashMap<(usize, usize), usize> = HashMap::new();
    for i in 0..=max_x {
        for j in 0..=max_y {
            height_map.insert((i,j), 0);
        }
    }

    for i in 0..bricks.len() {
        let brick = bricks.get_mut(i).unwrap();
        // println!("brick lens {}, {}, {}", brick.get_x_len(), brick.get_y_len(), brick.get_z_len());
        if brick.get_x_len() != 0 {
            let mut max_height_below = 0;
            for x in brick.position.0.clone() {
                let pos = (x, *brick.position.1.start());
                if *height_map.get(&pos).unwrap() > max_height_below {
                    max_height_below = *height_map.get(&pos).unwrap();
                }
            }
            for x in brick.position.0.clone() {
                let pos = (x, *brick.position.1.start());
                height_map.insert(pos, max_height_below+1);
            }
            // println!("x case, max height below: {max_height_below}");
            brick.position.2 = max_height_below+1..=max_height_below+1;
        } else if brick.get_y_len() != 0 {
            let mut max_height_below = 0;
            for y in brick.position.1.clone() {
                let pos = (*brick.position.0.start(), y);
                if *height_map.get(&pos).unwrap() > max_height_below {
                    max_height_below = *height_map.get(&pos).unwrap();
                }
            }
            for y in brick.position.1.clone() {
                let pos = (*brick.position.0.start(), y);
                height_map.insert(pos, max_height_below+1);
            }
            // println!("y case, max height below: {max_height_below}");
            brick.position.2 = max_height_below+1..=max_height_below+1;
        } else {
            let x_y = (*brick.position.0.start(), *brick.position.1.start());
            let new_z_start = *height_map.get(&x_y).unwrap() + 1;
            brick.position.2 = (new_z_start)..=(new_z_start + brick.get_z_len());
            height_map.insert(x_y, *brick.position.2.end());
        }
    }
}

#[derive(Debug)]
struct Brick {
    label: usize,
    position: (RangeInclusive<usize>,RangeInclusive<usize>,RangeInclusive<usize>)
}

impl Brick {
    pub fn get_x_len(&self) -> usize {
        return self.position.0.end() - self.position.0.start();
    }

    pub fn get_y_len(&self) -> usize {
        return self.position.1.end() - self.position.1.start();
    }

    pub fn get_z_len(&self) -> usize {
        return self.position.2.end() - self.position.2.start();
    }

    pub fn overlaps_xy(&self, other: &Brick) -> bool {
        let overlaps_x = self.position.0.contains(other.position.0.start()) 
            || self.position.0.contains(other.position.0.end())
            || other.position.0.contains(self.position.0.start());
        let overlaps_y = self.position.1.contains(other.position.1.start()) 
            || self.position.1.contains(other.position.1.end())
            || other.position.1.contains(self.position.1.start());
        return overlaps_x & overlaps_y;
    }
}