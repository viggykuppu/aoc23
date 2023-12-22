use std::{collections::{HashMap, HashSet}, ops::{RangeInclusive, Range}};

use aocd::*;
use regex::Regex;

#[aocd(2023, 22, "src/twentytwo/input.txt")]
pub fn one() {
    let binding = input!();
    let brick_regex = Regex::new(r"(\d+,\d+,\d+)~(\d+,\d+,\d+)").unwrap();
    let char_offset = 65_u8;
    let mut bricks: Vec<_> = binding.lines().enumerate().map(|(i, line)| {
        let caps: Vec<_> = brick_regex.captures_iter(line).collect();
        let caps = caps.get(0).unwrap();
        let first: Vec<_> = caps.get(1).unwrap().as_str().split(",").map(|d| d.parse::<usize>().unwrap()).collect();
        let second: Vec<_> = caps.get(2).unwrap().as_str().split(",").map(|d| d.parse::<usize>().unwrap()).collect();
        Brick {
            label: (char_offset + i as u8) as char,
            position: (first[0]..=second[0],first[1]..=second[1],first[2]..=second[2])
        }
    }).collect();

    let_bricks_fall(&mut bricks);
    println!("{:?}", bricks);

    let mut bricks_by_z: HashMap<usize, Vec<&Brick>> = HashMap::new();
    for brick in &bricks {
        if let None = bricks_by_z.get(brick.position.2.start()) {
            bricks_by_z.insert(*brick.position.2.start(), Vec::new());
        }
        bricks_by_z.get_mut(brick.position.2.start()).unwrap().push(brick);
    }

    let mut z_idx = 2;
    let mut destructible_bricks: HashSet<char> = HashSet::new();
    loop {
        if let Some(bricks_on_z) = bricks_by_z.get(&z_idx) {
            let bricks_below = bricks_by_z.get(&(z_idx - 1)).unwrap();
            for i in 0..bricks_on_z.len() {
                let brick = bricks_on_z.get(i).unwrap();
                let mut supporting_bricks: HashSet<char> = HashSet::new();
                for j in 0..bricks_below.len() {
                    let below_brick = bricks_below.get(j).unwrap();
                    for k in brick.position.0.clone() {
                        if below_brick.position.0.contains(&k) {
                            supporting_bricks.insert(below_brick.label);
                            break;
                        }
                    }
                    for k in brick.position.1.clone() {
                        if below_brick.position.1.contains(&k) {
                            supporting_bricks.insert(below_brick.label);
                            break;
                        }
                    }
                }
                println!("brick {} is supported by {:?}", brick.label, supporting_bricks);
                if supporting_bricks.len() > 1 {
                    supporting_bricks.iter().for_each(|brick| {
                        destructible_bricks.insert(*brick);
                    });
                }
            }
        } else {
            break;
        }
        z_idx += 1;
    }
    println!("destructible bricks: {:?}", destructible_bricks);
}

fn let_bricks_fall(bricks: &mut Vec<Brick>) {
    let max_x = bricks.iter().map(|b| b.position.0.end()).max().unwrap().clone();
    let max_y = bricks.iter().map(|b| b.position.1.end()).max().unwrap().clone();
    bricks.sort_by(|b1, b2| {
        b1.position.2.start().cmp(&b2.position.2.start())
    });
    let mut height_map: HashMap<(usize, usize), usize> = HashMap::new();
    for i in 0..=max_x {
        for j in 0..=max_y {
            height_map.insert((i,j), 0);
        }
    }

    let mut fallen_bricks: Vec<&Brick> = Vec::new();
    println!("{:?}", bricks);
    for i in 0..bricks.len() {
        let brick = bricks.get_mut(i).unwrap();
        println!("brick lens {}, {}, {}", brick.get_x_len(), brick.get_y_len(), brick.get_z_len());
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
            println!("x case, max height below: {max_height_below}");
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
            println!("y case, max height below: {max_height_below}");
            brick.position.2 = max_height_below+1..=max_height_below+1;
        } else if brick.get_z_len() != 0 {
            let x_y = (*brick.position.0.start(), *brick.position.1.start());
            let new_z_start = *height_map.get(&x_y).unwrap() + 1;
            brick.position.2 = (new_z_start)..=(new_z_start + brick.get_z_len());
            height_map.insert(x_y, *brick.position.2.end());
        }
    }
}

#[derive(Debug)]
struct Brick {
    label: char,
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
}