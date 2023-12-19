use std::collections::{HashMap, BTreeSet};

use aocd::*;
use regex::Regex;


#[aocd(2023,18)]
pub fn one() {
    let binding = input!();
    let direction_map = HashMap::from([(Direction::UP, (-1, 0)), (Direction::DOWN, (1, 0)),(Direction::LEFT, (0, -1)),(Direction::RIGHT, (0, 1))]);
    let right_node = HashMap::from([(Direction::UP, (0, 1)), (Direction::DOWN, (0, -1)),(Direction::LEFT, (-1, 0)),(Direction::RIGHT, (1, 0))]);
    let instruction_regex = Regex::new(r"(\S) (\d+) \((\S*)\)").unwrap();
    let instructions: Vec<_> = binding.lines().map(|line| {
        let c: Vec<_> = instruction_regex.captures_iter(line).map(|c| c.extract::<3>()).collect();
        let direction = match c[0].1[0] {
            "U" => Direction::UP,
            "L" => Direction::LEFT,
            "R" => Direction::RIGHT,
            "D" => Direction::DOWN,
            _ => Direction::UP
        };
        let distance = c[0].1[1].parse::<u32>().unwrap();
        Instruction {
            direction: direction,
            distance: distance,
        }
    }).collect();
    let mut current = (0,0);
    let mut total_border_distance = 0;
    let vertices: Vec<_> = instructions.iter().map(|i| {
        let v = direction_map[&i.direction];
        current.0 += v.0*(i.distance as isize);
        current.1 += v.1*(i.distance as isize);
        total_border_distance += i.distance;
        current.clone()
    }).collect();
    let area = shoelace(&vertices);
    let volume = picks(total_border_distance, area);
    submit!(1, volume);
}

#[aocd(2023,18)]
pub fn two() {
    let binding = input!();
    let direction_map = HashMap::from([(Direction::UP, (-1, 0)), (Direction::DOWN, (1, 0)),(Direction::LEFT, (0, -1)),(Direction::RIGHT, (0, 1))]);
    let right_node = HashMap::from([(Direction::UP, (0, 1)), (Direction::DOWN, (0, -1)),(Direction::LEFT, (-1, 0)),(Direction::RIGHT, (1, 0))]);
    let instruction_regex = Regex::new(r"\(\#(.*)\)").unwrap();
    let instructions: Vec<_> = binding.lines().map(|line| {
        let c: Vec<_> = instruction_regex.captures_iter(line).map(|c| c.extract::<1>()).collect();
        let hex_code = c[0].1[0];
        let direction = match hex_code.chars().last().unwrap() {
            '3' => Direction::UP,
            '2' => Direction::LEFT,
            '0' => Direction::RIGHT,
            '1' => Direction::DOWN,
            _ => Direction::UP
        };
        let distance = u32::from_str_radix(&hex_code[0..(hex_code.len()-1)], 16).unwrap();
        Instruction {
            direction: direction,
            distance: distance,
        }
    }).collect();
    let mut current = (0,0);
    let mut total_border_distance = 0;
    let vertices: Vec<_> = instructions.iter().map(|i| {
        let v = direction_map[&i.direction];
        current.0 += v.0*(i.distance as isize);
        current.1 += v.1*(i.distance as isize);
        total_border_distance += i.distance;
        current.clone()
    }).collect();
    let area = shoelace(&vertices);
    let volume = picks(total_border_distance, area);
    submit!(2, volume);
}

fn shoelace(vertices: &Vec<(isize, isize)>) -> u64 {
    let mut total = 0;
    for i in 0..vertices.len() {
        let current = vertices[i];
        let next = vertices[(i+1) % vertices.len()];
        
        total += (next.0 + current.0)*(next.1 - current.1);
    }
    (total.abs() as u64)/2
}

// modified pick's theorem to return i + b = A + b/2 + 1
fn picks(total_border_distance: u32, area: u64) -> u64 {
    return area + (total_border_distance as u64)/2 + 1;
}

#[aocd(2023,18)]
pub fn one_slow() {
    let binding = input!();
    let direction_map = HashMap::from([(Direction::UP, (-1, 0)), (Direction::DOWN, (1, 0)),(Direction::LEFT, (0, -1)),(Direction::RIGHT, (0, 1))]);
    let right_node = HashMap::from([(Direction::UP, (0, 1)), (Direction::DOWN, (0, -1)),(Direction::LEFT, (-1, 0)),(Direction::RIGHT, (1, 0))]);
    let instruction_regex = Regex::new(r"(\S) (\d+) \((\S*)\)").unwrap();
    let instructions: Vec<_> = binding.lines().map(|line| {
        let c: Vec<_> = instruction_regex.captures_iter(line).map(|c| c.extract::<3>()).collect();
        let direction = match c[0].1[0] {
            "U" => Direction::UP,
            "L" => Direction::LEFT,
            "R" => Direction::RIGHT,
            "D" => Direction::DOWN,
            _ => Direction::UP
        };
        let distance = c[0].1[1].parse::<u32>().unwrap();
        Instruction {
            direction: direction,
            distance: distance,
        }
    }).collect();
    
    // find grid size
    let mut maxes: (isize, isize) = (isize::MIN, isize::MIN);
    let mut mins: (isize, isize) = (isize::MAX, isize::MAX);
    let mut counts: (isize, isize) = (0,0);
    instructions.iter().enumerate().for_each(|(j, i)|  {
        let d = direction_map[&i.direction];
        counts.0 += i.distance as isize*d.0;
        counts.1 += i.distance as isize*d.1;
        if counts.0 > maxes.0 {
            maxes.0 = counts.0;
        }
        if counts.0 < mins.0 {
            mins.0 =  counts.0;
        }
        if counts.1 > maxes.1 {
            maxes.1 = counts.1;
        }
        if counts.1 < mins.1 {
            mins.1 = counts.1;
        }
    });
    let size = (maxes.0 - mins.0 + 1, maxes.1 - mins.1 + 1);
    let mut grid: Vec<Vec<char>> = Vec::new();
    for i in 0..size.0 as usize {
        grid.push(Vec::new());
        for _ in 0..size.1{
            grid[i].push('.');
        }
    }
    let mut current_position = (0 - mins.0,0 - mins.1);

    instructions.iter().for_each(|i| {
        let v = direction_map[&i.direction];
        let v_right = right_node[&i.direction];
        for _ in 0..i.distance {
            grid[current_position.0 as usize][current_position.1 as usize] = '#';
            current_position.0 = current_position.0 + v.0;
            current_position.1 = current_position.1 + v.1;

            //set the right node to an inside node if it's not already a #
            let right_position = (current_position.0 + v_right.0, current_position.1 + v_right.1);
            if grid[right_position.0 as usize][right_position.1 as usize] != '#' {
                grid[right_position.0 as usize][right_position.1 as usize] = 'I';
            }
        }
    });
    
    // Flood Fill the I's into  #s
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'I' {
                fill_inside(&mut grid, &direction_map, (i as isize,j as isize));
            }
        }
    }

    let volume = grid.iter().fold(0, |acc, line| {
        let row_total= line.iter().fold(0, |acc2, c| {
            if *c == '#' || *c == 'I' {
                acc2 + 1
            } else {
                acc2
            }
        });
        acc + row_total
    });

    submit!(1, volume);
}

#[aocd(2023,18)]
pub fn two_slow() {
    let binding = input!();
    let direction_map = HashMap::from([(Direction::UP, (-1, 0)), (Direction::DOWN, (1, 0)),(Direction::LEFT, (0, -1)),(Direction::RIGHT, (0, 1))]);
    let right_node = HashMap::from([(Direction::UP, (0, 1)), (Direction::DOWN, (0, -1)),(Direction::LEFT, (-1, 0)),(Direction::RIGHT, (1, 0))]);
    let instruction_regex = Regex::new(r"\(\#(.*)\)").unwrap();
    let instructions: Vec<_> = binding.lines().map(|line| {
        let c: Vec<_> = instruction_regex.captures_iter(line).map(|c| c.extract::<1>()).collect();
        let hex_code = c[0].1[0];
        let direction = match hex_code.chars().last().unwrap() {
            '3' => Direction::UP,
            '2' => Direction::LEFT,
            '0' => Direction::RIGHT,
            '1' => Direction::DOWN,
            _ => Direction::UP
        };
        let distance = u32::from_str_radix(&hex_code[0..(hex_code.len()-1)], 16).unwrap();
        Instruction {
            direction: direction,
            distance: distance,
        }
    }).collect();
    
    // find grid size
    let mut maxes: (isize, isize) = (isize::MIN, isize::MIN);
    let mut mins: (isize, isize) = (isize::MAX, isize::MAX);
    let mut counts: (isize, isize) = (0,0);
    instructions.iter().enumerate().for_each(|(j, i)|  {
        let d = direction_map[&i.direction];
        counts.0 += i.distance as isize*d.0;
        counts.1 += i.distance as isize*d.1;
        if counts.0 > maxes.0 {
            maxes.0 = counts.0;
        }
        if counts.0 < mins.0 {
            mins.0 =  counts.0;
        }
        if counts.1 > maxes.1 {
            maxes.1 = counts.1;
        }
        if counts.1 < mins.1 {
            mins.1 = counts.1;
        }
    });
    let size = (maxes.0 - mins.0 + 1, maxes.1 - mins.1 + 1);
    // println!("size: {:?}", size);
    let mut current_position = (0 - mins.0,0 - mins.1);
    // println!("current position: {:?}", current_position);
    let mut z: HashMap<usize, BTreeSet<(usize, usize, bool)>> = HashMap::new();

    instructions.iter().enumerate().for_each(|(idx, i)| {
        let v = direction_map[&i.direction];
        if v.0 == 0 {
            // this is a horizontal line so just add end value
            let previous = if idx == 0 {
                Direction::UP
            } else {
                instructions[idx - 1].direction
            };
            let next = if idx == instructions.len() - 1 {
                Direction::UP
            } else {
                instructions[idx + 1].direction
            };
            let parity_change = previous == next;
            if i.direction == Direction::RIGHT {
                insert(current_position.0 as usize, (current_position.1 as usize + 1, (current_position.1 + v.1*(i.distance as isize)) as usize, parity_change), &mut z);
            } else {
                insert(current_position.0 as usize, ((current_position.1 + v.1*(i.distance as isize)) as usize, (current_position.1 - 1) as usize, parity_change), &mut z);
            }
            current_position.1 +=  v.1*(i.distance as isize);
        } else if v.1 == 0 {
            // this is a vertical line so add each column value
            for _ in 0..(i.distance-1) {
                current_position.0 += v.0;
                insert(current_position.0 as usize, (current_position.1 as usize, current_position.1 as usize, true), &mut z);
            }
            current_position.0 += v.0;
            insert(current_position.0 as usize, (current_position.1 as usize, current_position.1 as usize, false), &mut z);
        }
    });
    let volume = z.keys().fold(0, |acc, k| {
        let mut row_total = 0;
        let mut row_columns = z.get(k).unwrap().iter();
        let mut inside = false;
        let idx = 0;
        let mut prev_c = row_columns.next().unwrap();
        row_total += prev_c.1 - prev_c.0 + 1;
        while let Some(next_c) = row_columns.next() {
            inside = if prev_c.2 {
                !inside
            } else {
                inside
            };
            if inside {
                row_total += (next_c.0 - prev_c.1) - 1;
            }
            row_total += next_c.1 - next_c.0 + 1;
            prev_c = next_c;
        }
        row_total + acc
    });

    submit!(2, volume);
}


fn insert(row: usize, column: (usize, usize, bool), z: &mut HashMap<usize, BTreeSet<(usize, usize, bool)>>) {
    if !z.contains_key(&row) {
        z.insert(row, BTreeSet::new());
    }
    z.get_mut(&row).unwrap().insert(column);
}

fn fill_inside(grid: &mut Vec<Vec<char>>, direction_map: &HashMap<Direction, (isize, isize)>, current_position: (isize, isize)) {
    for direction in [Direction::UP, Direction::LEFT, Direction::RIGHT, Direction::DOWN] {
        let v = direction_map[&direction];
        let new_position = (current_position.0 + v.0, current_position.1 + v.1);
        if is_valid_index(&new_position, grid) {
            let value = grid[new_position.0 as usize][new_position.1 as usize];
            if value == '.' {
                grid[new_position.0 as usize][new_position.1 as usize] = 'I';
                fill_inside(grid, direction_map, current_position);
            }
        }
    }
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in  0..grid[i].len() {
            print!("{}", grid[i][j]);
        }
        println!("");
    }

}

fn is_valid_index(test_idx: &(isize, isize), grid: &Vec<Vec<char>>) -> bool {
    return test_idx.0 >= 0 && test_idx.0 < grid.len() as isize
        && test_idx.1 >= 0 && test_idx.1 < grid[0].len() as isize;
}

struct Instruction {
    direction: Direction,
    distance: u32,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    UP,
    RIGHT,
    LEFT,
    DOWN,
}