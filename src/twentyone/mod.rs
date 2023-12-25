use std::collections::{HashSet, HashMap};
use itertools::*;

use aocd::*;

#[aocd(2023,21)]
pub fn one() {
    let binding = input!();
    let mut start: (isize, isize) = (0,0);
    let grid: Vec<Vec<char>> = binding.lines().map(|line| line.chars().collect()).collect();
    let mut valid_spaces: HashSet<(isize, isize)> = HashSet::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match grid[i][j] {
                '.' => { valid_spaces.insert((i as isize,j as isize)); },
                'S' => { start = (i as isize,j as isize); valid_spaces.insert((i as isize,j as isize)); },
                _ => {}
            }
        }
    }
    let mut visited_nodes: HashSet<(isize,isize, usize)> = HashSet::new();
    let num_steps = 131;
    depth_first_search(&grid, &mut visited_nodes, &valid_spaces, start, num_steps);
    // println!("visited {:?}", visited_nodes);
    let num_possible_positions = visited_nodes.iter().filter(|n| n.2 == num_steps).count();
    println!("unique odd corners {}", visited_nodes.iter().filter(|n| (n.0 - 65).abs() + (n.1 - 65).abs() > 65 && n.2 % 2 == 1).unique_by(|n| (n.0, n.1)).count());
    println!("unique even corners {}", visited_nodes.iter().filter(|n| (n.0 - 65).abs() + (n.1 - 65).abs() > 65 && n.2 % 2 == 0).unique_by(|n| (n.0, n.1)).count());
    submit!(1, num_possible_positions);
}

#[aocd(2023,21)]
pub fn two() {
    let binding = input!();
    let mut start: (isize, isize) = (0,0);
    let grid: Vec<Vec<char>> = binding.lines().map(|line| line.chars().collect()).collect();
    let mut valid_spaces: HashSet<(isize, isize)> = HashSet::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match grid[i][j] {
                '.' => { valid_spaces.insert((i as isize,j as isize)); },
                'S' => { start = (i as isize,j as isize); valid_spaces.insert((i as isize,j as isize)); },
                _ => {}
            }
        }
    }
    let num_steps = 65 + 131*4;
    // let n_o = depth_first_search_basic(&grid, &mut HashSet::new(), &valid_spaces, start, num_steps);
    // let n_l = depth_first_search_basic(&grid, &mut HashSet::new(), &valid_spaces, (65, 130), num_steps-66);
    // let n_r = depth_first_search_basic(&grid, &mut HashSet::new(), &valid_spaces, (65, 0), num_steps-66);
    // let n_u = depth_first_search_basic(&grid, &mut HashSet::new(), &valid_spaces, (0, 65), num_steps-66);
    // let n_d = depth_first_search_basic(&grid, &mut HashSet::new(), &valid_spaces, (130, 65), num_steps-66);
    // let n_ul = depth_first_search_basic(&grid, &mut HashSet::new(), &valid_spaces, (130, 130), num_steps-132);
    // let n_ur = depth_first_search_basic(&grid, &mut HashSet::new(), &valid_spaces, (130, 0), num_steps-132);
    // let n_dl = depth_first_search_basic(&grid, &mut HashSet::new(), &valid_spaces, (0, 130), num_steps-132);
    // let n_dr = depth_first_search_basic(&grid, &mut HashSet::new(), &valid_spaces, (0, 0), num_steps-132);
    let num_possible_positions_total = depth_first_search_two(&grid, &mut HashSet::new(), &valid_spaces, start, num_steps);
    // println!("info for {} steps", num_steps);
    // let cross_total = n_o + n_r + n_l + n_u + n_d;
    // let corner_total = n_ul + n_ur + n_dl + n_dr;
    // println!("cross total: {}", cross_total);
    // println!("corner total: {}", corner_total);
    // println!("no: {}; nl: {}, nr: {}, nu: {}, nd: {}, total: {}; difference: {}", n_o, n_l, n_r, n_u, n_d, num_possible_positions_total, num_possible_positions_total - (cross_total + corner_total));
    submit!(2, num_possible_positions_total);
}


fn depth_first_search_basic(grid: &Vec<Vec<char>>, visited: &mut HashSet<(isize,isize, usize)>, valid_spaces: &HashSet<(isize, isize)>, start_position: (isize, isize), move_limit: usize) -> usize  {
    let mut nodes_to_visit: Vec<(isize, isize, usize)>;
    let direction_map = HashMap::from([(Direction::UP, (-1, 0)), (Direction::DOWN, (1, 0)),(Direction::LEFT, (0, -1)),(Direction::RIGHT, (0, 1))]);
    let directions = [Direction::UP, Direction::LEFT, Direction::RIGHT, Direction::DOWN];
    let mut end_nodes: HashSet<(isize, isize, usize)> = HashSet::from([(start_position.0, start_position.1, 0)]);
    for i in 1..=move_limit {
        nodes_to_visit = end_nodes.iter().cloned().collect();
        end_nodes.clear();
        while let Some(current_node) = nodes_to_visit.pop() {
            for direction in &directions {
                let v = direction_map.get(direction).unwrap();
                let new_position = &(current_node.0 + v.0, current_node.1 + v.1);
                let new_node = (new_position.0, new_position.1, current_node.2 + 1);
                if is_valid_index(new_position, grid) && valid_spaces.contains(&new_position) && !visited.contains(&new_node) && new_node.2 <= move_limit {
                    visited.insert(new_node);
                    if new_node.2 == i {
                        end_nodes.insert(new_node);
                    } else {
                        nodes_to_visit.push(new_node);
                    }
                }
            }
        }
        // println!("iteration {i} had capacity {}", end_nodes.len());
    }
    end_nodes.len()
}

fn depth_first_search_two(grid: &Vec<Vec<char>>, visited: &mut HashSet<(isize,isize, usize)>, valid_spaces: &HashSet<(isize, isize)>, start_position: (isize, isize), move_limit: usize) -> usize  {
    let mut nodes_to_visit: Vec<(isize, isize, usize)>;
    let direction_map = HashMap::from([(Direction::UP, (-1, 0)), (Direction::DOWN, (1, 0)),(Direction::LEFT, (0, -1)),(Direction::RIGHT, (0, 1))]);
    let directions = [Direction::UP, Direction::LEFT, Direction::RIGHT, Direction::DOWN];
    let mut end_nodes: HashSet<(isize, isize, usize)> = HashSet::from([(start_position.0, start_position.1, 0)]);
    for i in 1..=move_limit {
        nodes_to_visit = end_nodes.iter().cloned().collect();
        end_nodes.clear();
        while let Some(current_node) = nodes_to_visit.pop() {
            for direction in &directions {
                let v = direction_map.get(direction).unwrap();
                let new_position = &(current_node.0 + v.0, current_node.1 + v.1);
                let new_node = (new_position.0, new_position.1, current_node.2 + 1);
                if valid_spaces.contains(&(new_position.0.rem_euclid(grid.len() as isize), new_position.1.rem_euclid(grid.len() as isize))) && !visited.contains(&new_node) && new_node.2 <= i {
                    visited.insert(new_node);
                    if new_node.2 == i {
                        end_nodes.insert(new_node);
                    } else {
                        nodes_to_visit.push(new_node);
                    }
                }
            }
        }
    }
    end_nodes.len()
}


fn depth_first_search(grid: &Vec<Vec<char>>, visited: &mut HashSet<(isize,isize, usize)>, valid_spaces: &HashSet<(isize, isize)>, start_position: (isize, isize), move_limit: usize) {
    let mut nodes_to_visit = vec![(start_position.0, start_position.1, 0)];
    let direction_map = HashMap::from([(Direction::UP, (-1, 0)), (Direction::DOWN, (1, 0)),(Direction::LEFT, (0, -1)),(Direction::RIGHT, (0, 1))]);
    let directions = [Direction::UP, Direction::LEFT, Direction::RIGHT, Direction::DOWN];
    while let Some(current_node) = nodes_to_visit.pop() {
        for direction in &directions {
            let v = direction_map.get(direction).unwrap();
            let new_position = &(current_node.0 + v.0, current_node.1 + v.1);
            let new_node = (new_position.0, new_position.1, current_node.2 + 1);
            if is_valid_index(new_position, grid) && valid_spaces.contains(&new_position) && !visited.contains(&new_node) && new_node.2 <= move_limit {
                visited.insert(new_node);
                nodes_to_visit.push(new_node);
            }
        }
    }
}

fn is_valid_index(test_idx: &(isize, isize), grid: &Vec<Vec<char>>) -> bool {
    return test_idx.0 >= 0 && test_idx.0 < grid.len() as isize
        && test_idx.1 >= 0 && test_idx.1 < grid[0].len() as isize;
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    UP,
    RIGHT,
    LEFT,
    DOWN,
}