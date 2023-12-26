use std::collections::{HashMap, HashSet};

use aocd::*;

#[aocd(2023, 23)]
pub fn one() {
    let binding = input!();
    let direction_map = HashMap::from([(Direction::UP, (-1, 0)), (Direction::DOWN, (1, 0)),(Direction::LEFT, (0, -1)),(Direction::RIGHT, (0, 1))]);
    let dims = (binding.lines().collect::<Vec<_>>().len() as isize , binding.lines().collect::<Vec<_>>()[0].len() as isize );
    let grid: HashMap<(isize, isize), char> = HashMap::from_iter(binding.lines().enumerate().flat_map(|(i, line)| {
        line.chars().enumerate().map(move |(j, c)| {
            ((i as isize,j as isize), c)
        })
    }));
    let mut groups = HashMap::new();
    let mut longest_path = HashMap::new();
    let z = build_groups(&grid, (0,1), (dims.0-1, dims.1-2), Direction::DOWN, &mut groups, &mut longest_path, &direction_map);
    // println!("{:?}", groups);
    // println!("{:?}", longest_path);
    // let mut current = &(0,1);
    // print!("{:?}({}) -> ", current, groups.get(current).unwrap().points.len());
    // while let Some(x) = longest_path.get(current) {
    //     print!("{:?}({}) -> ", x, groups.get(x).unwrap().points.len());
    //     current = x;
    // }
    // println!("");
    submit!(1, z);
}

#[aocd(2023, 23)]
pub fn two() {
    let binding = input!();
    let direction_map = HashMap::from([(Direction::UP, (-1, 0)), (Direction::DOWN, (1, 0)),(Direction::LEFT, (0, -1)),(Direction::RIGHT, (0, 1))]);
    let dims = (binding.lines().collect::<Vec<_>>().len() as isize , binding.lines().collect::<Vec<_>>()[0].len() as isize );
    let grid: HashMap<(isize, isize), char> = HashMap::from_iter(binding.lines().enumerate().flat_map(|(i, line)| {
        line.chars().enumerate().map(move |(j, c)| {
            ((i as isize,j as isize), c)
        })
    }));
    let mut start: &Node;
    let mut nodes: HashMap<(isize, isize), Node> = HashMap::new();
    for i in 0..dims.0 {
        for j in 0..dims.1 {
            let mut turn_count = 0;
            for direction in [Direction::UP, Direction::LEFT, Direction::RIGHT, Direction::DOWN] {
                let v = direction_map.get(&direction).unwrap();
                let new_position = (i + v.0, j + v.1);
                if let Some(c) = grid.get(&(new_position.0, new_position.1)) {
                    if *c == '^' || *c == 'v' || *c == '<' || *c == '>' {
                        turn_count += 1;
                    }
                }
            }
            if turn_count >= 3 {
                nodes.insert((i,j), Node {
                    position: (i, j),
                    neighbors: Vec::new(),
                    start_distance: None,
                    end_distance: None,
                });
            }
        }
    }
    let start = (0,1);
    let end = (dims.0-1, dims.1-2);
    let mut start_node = (0,0);
    let mut end_node = (0,0);
    let node_locations: HashSet<(isize, isize)> = HashSet::from_iter(nodes.keys().cloned());
    for position in &node_locations {
        let node = nodes.get_mut(position).unwrap();
        let mut visited = HashSet::new();
        let mut to_visit = vec![(node.position, 0)];
        while let Some(current) = to_visit.pop() {
            visited.insert(current.0);
            for direction in [Direction::UP, Direction::LEFT, Direction::RIGHT, Direction::DOWN] {
                let v = direction_map.get(&direction).unwrap();
                let new_position = (current.0.0 + v.0, current.0.1 + v.1);
                if !visited.contains(&new_position) {
                    // found another junction
                    if node_locations.contains(&new_position) {
                        node.neighbors.push((new_position.0, new_position.1, current.1 + 1));
                    } else if new_position == start {
                        node.start_distance = Some(current.1 + 1);
                        start_node = *position;
                    } else if new_position == end {
                        node.end_distance = Some(current.1 + 1);
                        end_node = *position;
                    } else if let Some(c) = grid.get(&new_position) {
                        if *c != '#' {
                            to_visit.insert(0, (new_position, current.1 + 1));
                        }
                    }
                }
            }
        }
    }
    let max_distance = get_max_distance(&nodes, start_node, &mut HashSet::new());
    submit!(2, max_distance);
}

fn get_max_distance(graph: &HashMap<(isize, isize), Node>, current: (isize, isize), visited: &mut HashSet<(isize, isize)>) -> u32 {
    let current_node = graph.get(&current).unwrap();
    visited.insert(current);
    if let Some(end_distance) = current_node.end_distance {
        return end_distance;
    }
    let mut neighbor_distances = Vec::new();
    for neighbor in &current_node.neighbors {
        if !visited.contains(&(neighbor.0, neighbor.1)) {
            neighbor_distances.push(neighbor.2 + get_max_distance(graph, (neighbor.0, neighbor.1), &mut visited.clone()));
        }
    }
    let mut max_distance = 0;
    for distance in neighbor_distances {
        if distance > max_distance {
            max_distance = distance;
        }
    }
    if let Some(start_distance) = current_node.start_distance {
        max_distance += start_distance;
    }
    return max_distance;
}

#[derive(Debug)]
struct Node {
    position: (isize, isize),
    neighbors: Vec<(isize, isize, u32)>,
    start_distance: Option<u32>,
    end_distance: Option<u32>,
}

fn build_groups(grid: &HashMap<(isize, isize), char>, start: (isize, isize), end: (isize, isize), start_direction: Direction, groups: &mut HashMap<(isize, isize), Group>, longest_path: &mut HashMap<(isize, isize), (isize, isize)>, direction_map: &HashMap<Direction, (isize, isize)>) -> usize {
    let mut new_group = Group {
        id: start,
        points: HashSet::new(),
        next_groups: Vec::new(),
        exit: false,
    };
    let mut previous = start;
    let first_v = direction_map.get(&start_direction).unwrap();
    let mut current = (start.0 + first_v.0, start.1 + first_v.1);
    let mut num_exits = 0;
    new_group.points.insert(current);
    let mut child_groups = HashMap::new();
    loop {
        for direction in [Direction::UP, Direction::LEFT, Direction::RIGHT, Direction::DOWN] {
            let v = direction_map.get(&direction).unwrap();
            let new_position = (current.0 + v.0, current.1 + v.1);
            // Never go back to start position for the group
            if new_position != start {
                if let Some(spot) = grid.get(&new_position) {
                    match spot {
                        '.' => {
                            if !new_group.points.contains(&new_position) {
                                new_group.points.insert(new_position);
                                current = new_position;
                            }
                        },
                        '>' | '^' | '<' | 'v' => { 
                            if is_exit(&direction, spot) {
                                if !new_group.points.contains(&new_position) {
                                    num_exits += 1;
                                    new_group.points.insert(new_position);
                                    new_group.next_groups.push(new_position);
                                    child_groups.insert(new_position, build_groups(grid, new_position, end, direction, groups, longest_path, direction_map));
                                }
                            }
                        },
                        _ => {},
                    }
                }
            }
        }
        if current == previous || num_exits > 0 {
            break;
        }
        previous = current;
    }
    let max_length_from_here;
    if current == end {
        new_group.exit = true;
        max_length_from_here = new_group.points.len();
    } else if child_groups.is_empty() {
        max_length_from_here = 0;
    } else {
        // calculate max length of child groups
        let mut max = 0;
        let mut max_key = (0,0);
        for key in child_groups.keys() {
            let child_group_size = child_groups.get(key).unwrap();
            if *child_group_size >= max {
                max = *child_group_size;
                max_key = *key;
            }
        }
        // note that the current group's length is counting all the exits, but since we only took 1 we only want to count 1
        // So we subtract all but 1 of the exits
        max_length_from_here = (new_group.points.len() - (child_groups.len() - 1)) + max;
        longest_path.insert(new_group.id, max_key);
    }
    groups.insert(new_group.id, new_group);
    return max_length_from_here;
}

fn is_exit(direction: &Direction, slope: &char) -> bool {
    (direction == &Direction::UP && slope == &'^') ||
        (direction == &Direction::LEFT && slope == &'<') ||
        (direction == &Direction::RIGHT && slope == &'>') ||
        (direction == &Direction::DOWN && slope == &'v')
}

#[derive(Debug)]
struct Group {
    id: (isize, isize),
    points: HashSet<(isize, isize)>,
    next_groups: Vec<(isize, isize)>,
    exit: bool,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    UP,
    RIGHT,
    LEFT,
    DOWN,
}