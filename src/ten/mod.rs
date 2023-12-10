use std::collections::{HashMap, HashSet};

use aocd::*;
use regex::Regex;

#[aocd(2023,10)]
pub fn one() {
    let binding = input!();
    let grid: Vec<Vec<_>> = binding.lines().map(|line| line.chars().collect()).collect();
    let start_position = Regex::new(r"S").unwrap().find(&binding).unwrap().start();
    // need to offset by 1 since start_position also accounts for newlines
    let start_row = (start_position as f32 / (grid[0].len()+1) as f32) as usize;
    let start_column = start_position % (grid[0].len()+1);
    let pipes_map = build_pipes_map();
    let direction_map = build_direction_map();
    let directions = vec![Direction::North, Direction::East, Direction::South, Direction::West];
    let mut current_position = (start_column, start_row);
    let mut current_character = 'S';
    let mut new_direction: &Direction = &Direction::North;
    let mut distance_traveled: u32 = 1;
    for direction in directions.iter() {
        let (new_character, new_position) = get_val_at_direction(&grid, current_position, direction, &direction_map);
        if let Some(d) = pipes_map.get(direction).unwrap().get(new_character) {
            current_position = new_position;
            current_character = *new_character;
            new_direction = d;
            break;
        }
    }
    
    while current_character != 'S' {
        distance_traveled += 1;
        let (new_character, new_position) = get_val_at_direction(&grid, current_position, new_direction, &direction_map);
        current_position = new_position;
        current_character = *new_character;
        if current_character != 'S' {
            new_direction = pipes_map.get(new_direction).unwrap().get(new_character).unwrap();
        }
    }
    let furthest_distance = (distance_traveled as f32 / 2_f32) as u32;
    submit!(1, furthest_distance);
}

#[aocd(2023,10)]
pub fn two() {
    let binding = input!();
    let mut start_row = 0;
    let mut start_column = 0;
    let mut grid: Vec<Vec<_>> = binding.lines().enumerate().map(|(i, line)| {
        let mut chars: Vec<_> = line.chars().collect();
        chars.insert(0, 'O');
        if let Some(m) = Regex::new(r"S").unwrap().find(line) {
            start_row = i;
            start_column = m.start()+1;
        }
        chars
    }).collect();
    let pipes_map = build_pipes_map();
    let direction_map = build_direction_map();
    let directions = vec![Direction::North, Direction::East, Direction::South, Direction::West];
    let mut loop_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut current_position = (start_column, start_row);
    loop_positions.insert(current_position);
    let mut current_character = 'S';
    let mut new_direction: &Direction = &Direction::North;
    for direction in directions.iter() {
        let (new_character, new_position) = get_val_at_direction(&grid, current_position, direction, &direction_map);
        if let Some(d) = pipes_map.get(direction).unwrap().get(new_character) {
            current_position = new_position;
            current_character = *new_character;
            new_direction = d;
            break;
        }
    }
    while current_character != 'S' {
        loop_positions.insert(current_position);
        let (new_character, new_position) = get_val_at_direction(&grid, current_position, new_direction, &direction_map);
        current_position = new_position;
        current_character = *new_character;
        if current_character != 'S' {
            new_direction = pipes_map.get(new_direction).unwrap().get(new_character).unwrap();
        }
    }
    grid.iter_mut().enumerate().for_each(|(i, row)| {
        for j in 0..row.len() {
            if !loop_positions.contains(&(j, i)) && row[j] != 'O' {
                row[j] = '.';
            }
        }
    });
    let upper_chars = HashSet::from(['|','F','7', 'S']);
    let lower_chars = HashSet::from(['|','L','J', 'S']);
    let mut num_inside = 0;
    for i in 0..grid.len() {
        let mut inside = false;
        for j in 0..grid[i].len() {
            // if we're at bottom rows all .s are outside
            if i == grid.len() - 1 {
                if grid[i][j] == '.' {
                    grid[i][j] = 'O';
                }
                continue;
            }
            // Now search from start of row to . for vertical lines
            let upper_char = grid[i][j];
            let lower_char = grid[i+1][j];
            if upper_chars.contains(&upper_char) && lower_chars.contains(&lower_char) {
                inside = !inside;
            }
            if upper_char == '.' {
                grid[i][j] = if inside {
                    num_inside += 1;
                    'I'
                } else {
                    'O'
                }
            }
        }
    }
    print_map(&grid, true);

    submit!(2, num_inside);
}

fn print_map(grid: &Vec<Vec<char>>, pretty: bool) {
    grid.iter().enumerate().for_each(|(i, row)| {
        for j in 0..row.len() {
            if pretty {
                print!("{}",pretty_print(row[j]));
            } else {
                print!("{}",row[j]);
            }
        }
        print!("\n");
    });
    println!("");
}

fn pretty_print(c: char) -> char {
    if c == 'L' {
        '\u{02517}'
    } else if c == 'J' {
        '\u{0251B}'
    } else if c == '7' {
        '\u{02513}'
    } else if c == 'F' {
        '\u{0250F}'
    } else if c == '|' {
        '\u{02503}'
    } else if c == '-' {
        '\u{02501}'
    } else {
        c
    }
}

fn build_pipes_map() -> HashMap<Direction, HashMap<char,Direction>> {
    let mut pipes_map = HashMap::new();

    let mut north_map: HashMap<char, Direction> = HashMap::new();
    north_map.insert('|', Direction::North);
    north_map.insert('F', Direction::East);
    north_map.insert('7', Direction::West);
    pipes_map.insert(Direction::North, north_map);

    let mut east_map: HashMap<char, Direction> = HashMap::new();
    east_map.insert('-', Direction::East);
    east_map.insert('7', Direction::South);
    east_map.insert('J', Direction::North);
    pipes_map.insert(Direction::East, east_map);
    pipes_map.insert(Direction::West, HashMap::new());

    let mut west_map: HashMap<char, Direction> = HashMap::new();
    west_map.insert('-', Direction::West);
    west_map.insert('F', Direction::South);
    west_map.insert('L', Direction::North);
    pipes_map.insert(Direction::West, west_map);

    let mut south_map: HashMap<char, Direction> = HashMap::new();
    south_map.insert('|', Direction::South);
    south_map.insert('J', Direction::West);
    south_map.insert('L', Direction::East);
    pipes_map.insert(Direction::South, south_map);

    pipes_map
}   

fn build_direction_map() -> HashMap<Direction, (i32, i32)> {
    let mut direction_map = HashMap::new();
    direction_map.insert(Direction::North, (0,-1));
    direction_map.insert(Direction::East, (1,0));
    direction_map.insert(Direction::South, (0,1));
    direction_map.insert(Direction::West, (-1,0));
    direction_map
}

fn get_val_at_direction<'a>(grid: &'a Vec<Vec<char>>, current_position: (usize, usize), direction: &Direction, direction_map: &HashMap<Direction, (i32, i32)>) -> (&'a char, (usize, usize)) {
    let (dx, dy) = direction_map.get(direction).unwrap();
    let new_x = (current_position.0 as i32 + *dx as i32) as usize;
    let new_y = (current_position.1 as i32 + *dy as i32) as usize;
    (grid.get(new_y).unwrap().get(new_x).unwrap(), (new_x, new_y))
}

#[derive(Hash, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    West,
    South,
}