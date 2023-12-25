use std::{collections::{HashMap, HashSet, BinaryHeap}, cmp::Reverse};

use aocd::*;
use itertools::Itertools;
use regex::Regex;


#[aocd(2023,25)]
pub fn one() {
    let binding = input!();
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    let line_regex = Regex::new(r"(.*): (.*)").unwrap();
    let mut z = 0;
    binding.lines().for_each(|line| {
        let caps = line_regex.captures_iter(line).collect::<Vec<_>>();
        let first_chunk = &caps.get(0).unwrap().get(1).unwrap().as_str();
        let second_chunk = caps.get(0).unwrap().get(2).unwrap().as_str().split(' ');
        if !graph.contains_key(first_chunk) {
            graph.insert(first_chunk, HashSet::new());
        }
        for chunk in second_chunk {
            z += 1;
            if !graph.contains_key(chunk) {
                graph.insert(chunk, HashSet::new());
            }
            graph.get_mut(first_chunk).unwrap().insert(chunk);
            graph.get_mut(chunk).unwrap().insert(first_chunk);
        }
        
        // for chunk in second_chunk {
        //     println!("{first_chunk}--{chunk}");
        // }
    });
    // find the right edge to remove and remove it
    // each iteration in the loop removes one of the 3 edges
    // insight here is that each of the 3 edges removed are highly traversed on a dijkstras done on ever node
    // if we consider the final two subgraphs A & B, then the only connection b/t the two subgraphs are the 3 edges (x,y,z)
    // as a result all paths from a node in A to a node in B must go across one of (x,y,z)
    // So theoretically these nodes are highly traversed as long as |A| and |B| is not too wildly different I imagine
    // If we apply dijkstra's 3 times and remove the most traversed edge then we'll get the right 3 edges to remove
    for _ in 0..3 {
        let mut edge_frequency_map = HashMap::new();
        for node in graph.keys() {
            dijkstras(&graph, node, &mut edge_frequency_map);
        }
        let mut max_freq = 0;
        let mut max_freq_edge = &String::new();
        for key in edge_frequency_map.keys() {
            let freq = edge_frequency_map.get(key).unwrap();
            if *freq > max_freq {
                max_freq = *freq;
                max_freq_edge = key;
            }
        }
        // remove the most frequently traversed edge
        let node1 = max_freq_edge.split('-').collect::<Vec<_>>()[0];
        let node2 = max_freq_edge.split('-').collect::<Vec<_>>()[1];
        graph.get_mut(node1).unwrap().remove(node2);
        graph.get_mut(node2).unwrap().remove(node1);
    }
    let visited = dijkstras(&graph, graph.keys().find(|_| true).unwrap(), &mut HashMap::new());
    let product_of_two_graphs = visited.len() * (graph.keys().len() - visited.len());
    submit!(1, product_of_two_graphs);
    
}

fn dijkstras<'a>(graph: &HashMap<&'a str, HashSet<&'a str>>, start_node: &'a str, edge_frequency_map: &mut HashMap<String, u32>) -> HashSet<&'a str> {
    let mut visited = HashSet::new();
    let mut min_distance_map = HashMap::<&str, u32>::new();
    let mut to_visit = BinaryHeap::new();
    to_visit.push(Reverse(Node {
        id: start_node,
        distance: 0,
    }));
    visited.insert(start_node);
    min_distance_map.insert(start_node, 0);
    while let Some(current) = to_visit.pop() {
        if visited.len() == graph.keys().len() {
            break;
        }
        let current = current.0;
        for neighbor in graph.get(current.id).unwrap() {
            if !visited.contains(neighbor) {
                to_visit.push(Reverse(Node {
                    id: neighbor,
                    distance: current.distance + 1,
                }));
                visited.insert(neighbor);
                min_distance_map.insert(neighbor, current.distance + 1);
                let mut freq_key =  String::new();
                if current.id > neighbor {
                    freq_key.push_str(current.id);
                    freq_key.push_str("-");
                    freq_key.push_str(neighbor);
                } else {
                    freq_key.push_str(neighbor);
                    freq_key.push_str("-");
                    freq_key.push_str(current.id);
                }
                if let Some(freq) = edge_frequency_map.get_mut(&freq_key) {
                    *freq += 1;
                } else {
                    edge_frequency_map.insert(freq_key, 0);
                }
            }
        }
    }
    return visited;
}


#[derive(Debug, PartialEq, Eq)]
struct Node<'a> {
    id: &'a str,
    distance: u32,
}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}