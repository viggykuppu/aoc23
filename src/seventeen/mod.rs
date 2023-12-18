use std::{collections::{HashMap, HashSet, BinaryHeap}, thread::current, cmp::Reverse};

use aocd::*;

#[aocd(2023,17)]
pub fn one() {
    let binding = input!();
    let direction_map = HashMap::from([(Direction::UP, (-1, 0)), (Direction::DOWN, (1, 0)),(Direction::LEFT, (0, -1)),(Direction::RIGHT, (0, 1))]);
    let grid: Vec<Vec<u32>> = binding.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
    let mimum_heat_loss = calculate_minimum_heat_loss(&grid, &direction_map, &mut HashSet::new(), 1, 3);
    submit!(1, mimum_heat_loss);
}

#[aocd(2023,17)]
pub fn two() {
    let binding = input!();
    let direction_map = HashMap::from([(Direction::UP, (-1, 0)), (Direction::DOWN, (1, 0)),(Direction::LEFT, (0, -1)),(Direction::RIGHT, (0, 1))]);
    let grid: Vec<Vec<u32>> = binding.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
    let mimum_heat_loss = calculate_minimum_heat_loss(&grid, &direction_map, &mut HashSet::new(), 4, 10);
    submit!(2, mimum_heat_loss);
}

fn calculate_minimum_heat_loss(grid: &Vec<Vec<u32>>, direction_map: &HashMap<Direction, (isize, isize)>, memo: &mut HashSet<(isize, isize, Direction, u8)>, min_straight: u8, max_straight: u8) -> u32 {
    let mut heap = BinaryHeap::new();
    heap.push(Reverse(Node {
        dir: Direction::RIGHT,
        straight_count: 0,
        position: (0, 0),
        distance: 0,
    }));
    let mut current = heap.pop().unwrap();
    loop {
        let current_direction = current.0.dir;
        let current_position = current.0.position;
        if current.0.straight_count >= (min_straight - 1) {
            for (i, direction) in get_turn_directions(&current_direction).iter().enumerate() {
                let v = direction_map[&direction];
                let new_position = (current_position.0 + v.0, current_position.1 + v.1);
                if is_valid_index(&new_position, grid) {
                    let d = current.0.distance + grid[new_position.0 as usize][new_position.1 as usize];
                    if !memo.contains(&(new_position.0, new_position.1, *direction, 0)) {
                        heap.push(Reverse(Node {
                            dir: *direction,
                            straight_count: 0,
                            position: new_position,
                            distance: d
                        }));
                        memo.insert((new_position.0, new_position.1, *direction, 0));
                    }
                }
            }
        }
        if current.0.straight_count < (max_straight - 1) {
            let v = direction_map[&current_direction];
            let straight_position = (current_position.0 + v.0, current_position.1 + v.1);
            if is_valid_index(&straight_position, grid) {
                let d = current.0.distance + grid[straight_position.0 as usize][straight_position.1 as usize];
                if !memo.contains(&(straight_position.0, straight_position.1, current_direction, current.0.straight_count + 1)) {
                    heap.push(Reverse(Node {
                        dir: current_direction,
                        straight_count: current.0.straight_count + 1,
                        position: straight_position,
                        distance: current.0.distance + grid[straight_position.0 as usize][straight_position.1 as usize]
                    }));
                    memo.insert((straight_position.0, straight_position.1, current_direction, current.0.straight_count + 1));
                }
            }
        }
        current = heap.pop().unwrap();
        if current.0.position.0 == (grid.len() as isize - 1) && current.0.position.1 == (grid[0].len() as isize - 1) {
            break;
        }
    }
    return current.0.distance;
}

#[derive(PartialEq, Eq)]
struct Node {
    dir: Direction,
    straight_count: u8,
    position: (isize, isize),
    distance: u32,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}


fn is_valid_index(test_idx: &(isize, isize), grid: &Vec<Vec<u32>>) -> bool {
    return test_idx.0 >= 0 && test_idx.0 < grid.len() as isize
        && test_idx.1 >= 0 && test_idx.1 < grid[0].len() as isize;
}

fn get_turn_directions(current_direction: &Direction) -> [Direction; 2] {
    match current_direction {
        Direction::UP | Direction::DOWN => [Direction::LEFT, Direction::RIGHT],
        Direction::LEFT | Direction::RIGHT => [Direction::UP, Direction::DOWN],
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    UP,
    RIGHT,
    LEFT,
    DOWN,
}