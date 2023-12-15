use std::collections::{HashMap, HashSet, BTreeSet};

use aocd::*;
use num::Float;

#[aocd(2023,14)]
pub fn one() {
    let binding = input!();
    let dish: Vec<Vec<char>> = binding.lines().map(|line| line.chars().collect()).collect();
    let mut load: usize = 0;
    for j in 0..dish[0].len() {
        let mut last_barrier_location = 0;
        let mut num_rocks_since_barrier = 0;
        for i in 0..dish.len() {
            let current_cell = dish[i][j];
            if current_cell == '#' {
                last_barrier_location = i+1;
                num_rocks_since_barrier = 0;
            } else if current_cell == 'O' {
                load += dish.len() - (last_barrier_location + num_rocks_since_barrier);
                num_rocks_since_barrier += 1;
            }
        }
    }
    submit!(1, load);
}

#[aocd(2023,14)]
pub fn two() {
    let binding = input!();
    let dish: Vec<Vec<char>> = binding.lines().map(|line| line.chars().collect()).collect();
    let mut rounds: Vec<(usize, usize)> = Vec::new();
    let mut squares: HashSet<(isize, isize)> = HashSet::new();
    for i in -1..=dish.len() as isize {
        squares.insert((i, -1));
        squares.insert((i, dish.len() as isize));
        squares.insert((-1, i));
        squares.insert((dish.len() as isize, i));
    }
    dish.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, c)| {
            if *c == '#' {
                squares.insert((i as isize,j as isize));
            } else if *c == 'O' {
                rounds.push((i,j));
            }
        });
    });
    let mut info: Vec<Vec<Option<RockInfo>>> = Vec::new();
    dish.iter().enumerate().for_each(|(i, line)| {
        info.push(Vec::<Option<RockInfo>>::new());
        line.iter().enumerate().for_each(|(j, c)| {
            if *c != '#' {
                let mut offset = 1;
                // north
                let north;
                loop {
                    if let Some(r) = squares.get(&(i as isize - offset, j as isize)) {
                        north = r;
                        break;
                    }
                    offset += 1;
                }
                let west;
                offset = 1;
                loop {
                    if let Some(r) = squares.get(&(i as isize, j as isize - offset)) {
                        west = r;
                        break;
                    }
                    offset += 1;
                }
                let south;
                offset = 1;
                loop {
                    if let Some(r) = squares.get(&(i as isize + offset, j as isize)) {
                        south = r;
                        break;
                    }
                    offset += 1;
                }
                let east;
                offset = 1;
                loop {
                    if let Some(r) = squares.get(&(i as isize, j as isize + offset)) {
                        east = r;
                        break;
                    }
                    offset += 1;
                }
                info[i].push(Some(RockInfo {
                    north_rock: *north,
                    east_rock:*east,
                    west_rock: *west,
                    south_rock: *south,
                }));
            } else {
                info[i].push(None);
            }
        });
    });
    let mut z: HashMap<String, usize> = HashMap::new();
    let mut first_encountered = 0;
    let mut cycle_length = 0;
    let goal_repetitions = 1000000000;
    let mut idx = 1;
    for _ in 0..10000 {
        // tilt north
        let mut offsets: HashMap<(isize, isize), isize> = HashMap::new();
        for r in &mut rounds {
            let rock_info = &info[r.0][r.1].as_ref().unwrap();
            let rock = rock_info.north_rock;
            if let Some(offset) = offsets.get_mut(&(rock.0, rock.1)) {
                *offset += 1;
                r.0 = (rock.0 + *offset) as usize;
            } else {
                offsets.insert((rock.0, rock.1), 1);
                r.0 = (rock.0 + 1) as usize;
            }
        }

        // tilt west
        offsets.clear();
        for r in &mut rounds {
            let rock_info = &info[r.0][r.1].as_ref().unwrap();
            let rock = rock_info.west_rock;
            if let Some(offset) = offsets.get_mut(&(rock.0, rock.1)) {
                *offset += 1;
                r.1 = (rock.1 + *offset) as usize;
            } else {
                offsets.insert((rock.0, rock.1), 1);
                r.1 = (rock.1 + 1) as usize;
            }
        }

        // tilt south
        offsets.clear();
        for r in &mut rounds {
            let rock_info = &info[r.0][r.1].as_ref().unwrap();
            let rock = rock_info.south_rock;
            if let Some(offset) = offsets.get_mut(&(rock.0, rock.1)) {
                *offset += 1;
                r.0 = (rock.0 - *offset) as usize;
            } else {
                offsets.insert((rock.0, rock.1), 1);
                r.0 = (rock.0 - 1) as usize;
            }
        }

        // tilt east
        offsets.clear();
        for r in &mut rounds {
            let rock_info = &info[r.0][r.1].as_ref().unwrap();
            let rock = rock_info.east_rock;
            if let Some(offset) = offsets.get_mut(&(rock.0, rock.1)) {
                *offset += 1;
                r.1 = (rock.1 - *offset) as usize;
            } else {
                offsets.insert((rock.0, rock.1), 1);
                r.1 = (rock.1 - 1) as usize;
            }
        }
        rounds.sort_by(|r1, r2| {
            if r1.1 == r2.1 {
                r1.0.cmp(&r2.0)
            } else {
                r1.1.cmp(&r2.1)
            }  
        });
        let s = rounds.iter().map(|r| format!("{},{};",r.0,r.1)).collect::<String>();
        if let Some(x) = z.get(&s) {
            if first_encountered == 0 {
                first_encountered = *x;
            }
            cycle_length = idx - x;
        }
        if cycle_length != 0 &&  (idx-first_encountered)%cycle_length == (goal_repetitions-first_encountered)%cycle_length {
            break;
        }
        z.insert(s, idx);
        idx += 1;
    }
    let load = get_load(&rounds, dish.len());
    submit!(2, load);
}

fn get_load(rounds: &Vec<(usize, usize)>, num_rows: usize) -> usize {
    rounds.iter().fold(0, |acc, r| {
        acc + (num_rows - r.0)
    })
}

fn print_dish(rounds: &Vec<(usize, usize)>, squares: &HashSet<(isize, isize)>) {
    for i in 0..10 {
        for j in 0..10 {
            if squares.contains(&(i,j)) {
                print!("#");
            } else if rounds.contains(&&(i as usize,j as usize)) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

#[derive(Debug)]
struct RockInfo  {
    north_rock: (isize, isize),
    east_rock: (isize, isize),
    west_rock:(isize, isize),
    south_rock: (isize, isize),
}