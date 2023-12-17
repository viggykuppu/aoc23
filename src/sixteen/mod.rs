use std::collections::{HashMap, HashSet};

use aocd::*;

#[aocd(2023,16)]
pub fn one() {
    let direction_map = HashMap::from([(Direction::UP, (-1, 0)), (Direction::DOWN, (1, 0)),(Direction::LEFT, (0, -1)),(Direction::RIGHT, (0, 1))]);
    let binding = input!();
    let grid = binding.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let dims = (grid.len(), grid[0].len());
    let total_energy = calculate_energy(&grid, &direction_map, &dims, (0,0), Direction::RIGHT);
    submit!(1, total_energy);
}

#[aocd(2023,16)]
pub fn two() {
    // constants
    let direction_map = HashMap::from([(Direction::UP, (-1, 0)), (Direction::DOWN, (1, 0)),(Direction::LEFT, (0, -1)),(Direction::RIGHT, (0, 1))]);

    let binding = input!();
    let grid = binding.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let dims = (grid.len(), grid[0].len());
    let mut max_energy = 0;
    for i in 0..grid.len() {
        let total_energy = calculate_energy(&grid, &direction_map, &dims, (0,i as isize), Direction::DOWN);
        if total_energy > max_energy {
            max_energy = total_energy;
        }
        let total_energy = calculate_energy(&grid, &direction_map, &dims, (dims.0 as isize - 1,i as isize), Direction::UP);
        if total_energy > max_energy {
            max_energy = total_energy;
        }
        let total_energy = calculate_energy(&grid, &direction_map, &dims, (i as isize,0), Direction::RIGHT);
        if total_energy > max_energy {
            max_energy = total_energy;
        }
        let total_energy = calculate_energy(&grid, &direction_map, &dims, (dims.1 as isize - 1,0), Direction::LEFT);
        if total_energy > max_energy {
            max_energy = total_energy;
        }
    }
    submit!(2, max_energy);
}

#[aocd(2023,16)]
pub fn two_fast() {
    // constants
    let direction_map = HashMap::from([(Direction::UP, (-1, 0)), (Direction::DOWN, (1, 0)),(Direction::LEFT, (0, -1)),(Direction::RIGHT, (0, 1))]);

    let binding = input!();
    let grid = binding.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let dims = (grid.len(), grid[0].len());
    let mut max_energy = 0;
    let mut memo = HashMap::new();
    for i in 0..grid.len() {
        let total_energy = r2(&grid, &direction_map, &dims, (0,i as isize), Direction::DOWN, &mut memo, &mut HashSet::new(), &mut HashSet::new()).len();
        if total_energy > max_energy {
            max_energy = total_energy;
        }
        let total_energy = r2(&grid, &direction_map, &dims, (dims.0 as isize - 1,i as isize), Direction::UP, &mut memo, &mut HashSet::new(), &mut HashSet::new()).len();
        if total_energy > max_energy {
            max_energy = total_energy;
        }
        let total_energy = r2(&grid, &direction_map, &dims, (i as isize,0), Direction::RIGHT, &mut HashMap::new(), &mut HashSet::new(), &mut HashSet::new()).len();
        if total_energy > max_energy {
            max_energy = total_energy;
        }
        let total_energy = r2(&grid, &direction_map, &dims, (dims.1 as isize - 1,0), Direction::LEFT, &mut memo, &mut HashSet::new(), &mut HashSet::new()).len();
        if total_energy > max_energy {
            max_energy = total_energy;
        }
    }
    submit!(2, max_energy);
}

fn calculate_energy_recursive(grid: &Vec<Vec<char>>, direction_map: &HashMap<Direction, (isize, isize)>, dims: &(usize, usize), start_position: (isize, isize), start_direction: Direction, visited_positions: &mut HashSet<(isize, isize, Direction)>) -> HashSet<(isize, isize)> {
    let current_space = grid[start_position.0 as usize][start_position.1 as usize];
    let new_directions = get_new_direction(&start_direction, &current_space);
    let mut visited = HashSet::from([start_position]);
    visited_positions.insert((start_position.0, start_position.1, start_direction));
    for direction in new_directions {
        let v: (isize, isize) = direction_map[&direction];
        let new_position = (start_position.0 + v.0, start_position.1 + v.1);
        if is_valid_index(&new_position, &dims) {
            if !visited_positions.contains(&(new_position.0, new_position.1, direction)) {
                visited.extend(&calculate_energy_recursive(grid, direction_map, dims, new_position, direction, visited_positions));
            }
        }
    }
    return visited;
}

fn r2(grid: &Vec<Vec<char>>, direction_map: &HashMap<Direction, (isize, isize)>, dims: &(usize, usize), start_position: (isize, isize), start_direction: Direction, memo: &mut HashMap<(isize, isize, Direction), HashSet<(isize, isize)>>, visited_positions: &mut HashSet<(isize, isize, Direction)>, z: &mut HashSet<(isize, isize)>) -> HashSet<(isize,isize)> {
    if let Some(z) = memo.get(&(start_position.0, start_position.1, start_direction)) {
        return z.clone();
    }
    let current_space = grid[start_position.0 as usize][start_position.1 as usize];
    let new_directions = get_new_direction(&start_direction, &current_space);
    let mut visited = HashSet::from([start_position]);
    visited_positions.insert((start_position.0, start_position.1, start_direction));
    for direction in new_directions {
        let v: (isize, isize) = direction_map[&direction];
        let new_position = (start_position.0 + v.0, start_position.1 + v.1);
        if is_valid_index(&new_position, &dims) {
            if !visited_positions.contains(&(new_position.0, new_position.1, direction)) {
                visited.extend(&r2(grid, direction_map, dims, new_position, direction, memo, visited_positions, z));
            }
        }
    }
    memo.insert((start_position.0, start_position.1, start_direction), visited.clone());
    return visited;
}

fn calculate_energy(grid: &Vec<Vec<char>>, direction_map: &HashMap<Direction, (isize, isize)>, dims: &(usize, usize), start_position: (isize, isize), start_direction: Direction) -> usize {
    let mut beams: Vec<((isize, isize), Direction)> = vec![(start_position, start_direction)];
    let mut path: Vec::<(isize, isize)> = Vec::new();
    let mut visited_positions: HashSet<(isize, isize, Direction)> = HashSet::new();
    visited_positions.insert((start_position.0, start_position.1, start_direction));
    while beams.len() != 0 {
        let mut new_beams: Vec<((isize, isize), Direction)> = Vec::new();
        let mut beams_to_remove: Vec<usize> = Vec::new();
        for (j, beam) in beams.iter_mut().enumerate() {
            path.push(beam.0);
            let current_space = grid[beam.0.0 as usize][beam.0.1 as usize];
            let new_directions = get_new_direction(&beam.1, &current_space);
            let v = direction_map[&new_directions[0]];
            let new_position = (beam.0.0 + v.0, beam.0.1 + v.1);
            // println!("CURRENT SPACE: {:?}; beam at position {:?} is going to position {:?}, old direction was {:?} and new direction is {:?}", current_space, beam.0, new_position, beam.1, new_directions);
            if is_valid_index(&new_position, &dims) {
                beam.0 = new_position;
                beam.1 = new_directions[0];
                if visited_positions.contains(&(beam.0.0, beam.0.1, new_directions[0])) {
                    beams_to_remove.push(j);
                } else {
                    visited_positions.insert((beam.0.0, beam.0.1, new_directions[0]));
                }
            } else {
                beams_to_remove.push(j);
            }
            if new_directions.len() == 2 {
                let v = direction_map[&new_directions[1]];
                let new_position = (beam.0.0 + v.0, beam.0.1 + v.1);
                if is_valid_index(&new_position, &dims) {
                    new_beams.push((new_position, new_directions[1]));
                    visited_positions.insert((new_position.0, new_position.1, new_directions[1]));
                }
            }
        }
        for (j, k) in beams_to_remove.iter().enumerate() {
            beams.remove(k - j);
        }
        while let Some(x) = new_beams.pop() {
            beams.push(x);
        }
    }
    let mut unique_positions: HashSet<(isize, isize)> = HashSet::new();
    visited_positions.iter().for_each(|(row, column, direction)| {
        unique_positions.insert((*row, *column));
    });
    unique_positions.len()
}

fn get_new_direction(current_direction: &Direction, current_space: &char) -> Vec<Direction> {
    match current_space {
        '-' => {
            if current_direction == &Direction::UP || current_direction == &Direction::DOWN {
                return vec![Direction::RIGHT, Direction::LEFT];
            } else {
                return vec![*current_direction];
            }
        }
        '|' => {
            if current_direction == &Direction::RIGHT || current_direction == &Direction::LEFT {
                return vec![Direction::DOWN, Direction::UP];
            } else {
                return vec![*current_direction];
            }
        },
        '\\' => {
            if  current_direction == &Direction::UP {
                return vec![Direction::LEFT];
            } else if current_direction == &Direction::RIGHT {
                return vec![Direction::DOWN];
            } else if current_direction == &Direction::LEFT {
                return vec![Direction::UP];
            } else {
                return vec![Direction::RIGHT];
            }
        },
        '/' => {
            if  current_direction == &Direction::UP {
                return vec![Direction::RIGHT];
            } else if current_direction == &Direction::RIGHT {
                return vec![Direction::UP];
            } else if current_direction == &Direction::LEFT {
                return vec![Direction::DOWN];
            } else {
                return vec![Direction::LEFT];
            }
        }
        _ => return vec![*current_direction]
    }
}

fn is_valid_index(test_idx: &(isize, isize), dims: &(usize, usize)) -> bool {
    return test_idx.0 >= 0 && test_idx.0 < dims.0 as isize
        && test_idx.1 >= 0 && test_idx.1 < dims.1 as isize;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    UP,
    RIGHT,
    LEFT,
    DOWN,
}