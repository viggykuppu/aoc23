use std::collections::{HashMap, HashSet};

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

#[aocd(2023,14, "src/fourteen/input.txt")]
pub fn two() {
    let binding = input!();
    let dish: Vec<Vec<char>> = binding.lines().map(|line| line.chars().collect()).collect();
    let mut load: usize = 0;
    let mut rounds: Vec<(usize, usize)> = Vec::new();
    let mut squares: HashSet<(i32, i32)> = HashSet::new();
    for i in -1..=dish.len() as i32 {
        squares.insert((i, -1));
        squares.insert((i, dish.len() as i32));
        squares.insert((-1, i));
        squares.insert((dish.len() as i32, i));
    }
    dish.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, c)| {
            if *c == '#' {
                squares.insert((i as i32,j as i32));
            } else if *c == 'O' {
                rounds.push((i,j));
            }
        });
    });
    let mut info: Vec<Option<RockInfo>> = Vec::new();
    dish.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, c)| {
            if *c != '#' {
                let mut offset = 1;
                // north
                let north;
                loop {
                    if let Some(r) = squares.get(&(i as i32 - offset, j as i32)) {
                        north = r;
                        break;
                    }
                    offset += 1;
                }
                let west;
                offset = 1;
                loop {
                    if let Some(r) = squares.get(&(i as i32, j as i32 - offset)) {
                        west = r;
                        break;
                    }
                    offset += 1;
                }
                let south;
                offset = 1;
                loop {
                    if let Some(r) = squares.get(&(i as i32 + offset, j as i32)) {
                        south = r;
                        break;
                    }
                    offset += 1;
                }
                let east;
                offset = 1;
                loop {
                    if let Some(r) = squares.get(&(i as i32, j as i32 + offset)) {
                        east = r;
                        break;
                    }
                    offset += 1;
                }
                info.push(Some(RockInfo {
                    north_rock: *north,
                    east_rock:*east,
                    west_rock: *west,
                    south_rock: *south,
                }));
            } else {
                info.push(None);
            }
        });
    });
    for _ in 0..1000000 {
        // tilt north
        let mut offsets: HashMap<i32, i32> = HashMap::new();
        for r in &mut rounds {
            let rock_info = &info[r.0*10 + r.1].as_ref().unwrap();
            let rock = rock_info.north_rock;
            if let Some(offset) = offsets.get_mut(&(rock.0*10 + rock.1)) {
                *offset += 1;
                r.0 = (rock.0 + *offset) as usize;
            } else {
                offsets.insert((rock.0*10 + rock.1), 1);
                r.0 = (rock.0 + 1) as usize;
            }
        }

        // tilt west
        offsets.clear();
        for r in &mut rounds {
            let rock_info = &info[r.0*10 + r.1].as_ref().unwrap();
            let rock = rock_info.west_rock;
            if let Some(offset) = offsets.get_mut(&(rock.0*10 + rock.1)) {
                *offset += 1;
                r.1 = (rock.1 + *offset) as usize;
            } else {
                offsets.insert((rock.0*10 + rock.1), 1);
                r.1 = (rock.1 + 1) as usize;
            }
        }

        // tilt south
        offsets.clear();
        for r in &mut rounds {
            let rock_info = &info[r.0*10 + r.1].as_ref().unwrap();
            let rock = rock_info.south_rock;
            if let Some(offset) = offsets.get_mut(&(rock.0*10 + rock.1)) {
                *offset += 1;
                r.0 = (rock.0 - *offset) as usize;
            } else {
                offsets.insert((rock.0*10 + rock.1), 1);
                r.0 = (rock.0 - 1) as usize;
            }
        }

        // tilt east
        offsets.clear();
        for r in &mut rounds {
            let rock_info = &info[r.0*10 + r.1].as_ref().unwrap();
            let rock = rock_info.east_rock;
            if let Some(offset) = offsets.get_mut(&(rock.0*10 + rock.1)) {
                *offset += 1;
                r.1 = (rock.1 - *offset) as usize;
            } else {
                offsets.insert((rock.0*10 + rock.1), 1);
                r.1 = (rock.1 - 1) as usize;
            }
        }
    }
    print_dish(&rounds, &squares);
    submit!(2, load);
}

fn print_dish(rounds: &Vec<(usize, usize)>, squares: &HashSet<(i32, i32)>) {
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
    north_rock: (i32, i32),
    east_rock: (i32, i32),
    west_rock:(i32, i32),
    south_rock: (i32, i32),
}

// fn foo() {
//     rounds.sort_by(|r1, r2| {
//         if r1.1 == r2.1 {
//             r1.0.cmp(&r2.0)
//         } else {
//             r1.1.cmp(&r2.1)
//         }                                  
//     });
//     squares.sort_by(|r1, r2| {
//         if r1.1 == r2.1 {
//             r1.0.cmp(&r2.0)
//         } else {
//             r1.1.cmp(&r2.1)
//         }
//     });
//     println!("{:?}", rounds);
//     println!("{:?}", squares);
//     let mut r_idx = 0;
//     let mut s_idx = 0;
//     let mut previous_column = 100;
//     let mut open_row = 0;
//     while r_idx < rounds.len() {
//         let square = if s_idx < squares.len() {
//             squares.get(s_idx).unwrap()
//         } else {
//            &(10_usize, 10_usize)
//         };
//         let round = rounds.get_mut(r_idx).unwrap();
//         if square.1 != previous_column {
//             open_row = 0;
//             previous_column = square.1;
//         }
//         // check if they're in the same column
//         if round.1 == square.1 {
//             // compare which row they are in
//             // if round is in a lesser row then round can go all the way up
//             if round.0 > square.0 {
//                 round.0 = open_row;
//                 open_row += 1;
//                 r_idx += 1;
//             } else {
//                 // if round is in a greater row then need to check if there is another square that blocks it
//                 s_idx += 1;
//                 if s_idx < squares.len() {
//                     open_row = squares[s_idx].0 + 1_usize;
//                 }
//             }
//         } else {
//             // else if round is in the lesser column, we can move square as far north as possible and then go to the next round
//             if round.1 < square.1 {
//                 round.0 = open_row;
//                 open_row += 1;
//                 r_idx += 1;
//             } else {
//                 // square is in the lesser column so check the next square to see if it will block
//                 s_idx += 1;
//                 if s_idx < squares.len() {
//                     open_row = squares[s_idx].0 + 1_usize;
//                 }
//             }
//         }
//     }
// }