use core::num;
use std::{ops::{Range, RangeInclusive}, collections::{HashMap, HashSet}};

use regex::Regex;
use crate::lib;

pub fn four_one() {
    let input = lib::read_input("src/four/input.txt");
    let mut total_score: u64 = 0;
    let number_regex = Regex::new(r"\d+").unwrap();
    for card in input.lines() {
        let mut num_wins: u32 = 0;
        let mut winning_numbers_set: HashSet<u32> = HashSet::new();
        let card_data: Vec<_> = card.split(":").collect::<Vec<_>>()[1].split("|").collect();
        let winning_numbers = card_data[0];
        let my_card = card_data[1];
        number_regex.find_iter(winning_numbers).for_each(|n| {winning_numbers_set.insert(n.as_str().parse().unwrap());});
        number_regex.find_iter(my_card).for_each(|n| {
            if winning_numbers_set.contains(&n.as_str().parse().unwrap()) {
                num_wins += 1;
            }
        });
        // println!("number of wins: {num_wins}");
        if num_wins != 0 {
            total_score += 2_u32.pow(num_wins-1) as u64;
        }
    }
    println!("My total card score is: {total_score}");
}

pub fn four_two() {
    let input = lib::read_input("src/four/input.txt");
    let mut total_copies: u32 = 0;
    let number_regex = Regex::new(r"\d+").unwrap();
    let mut copy_data: Vec<u32> = Vec::new();
    for _ in input.lines() {
        copy_data.push(1);
    }
    for (i, card) in input.lines().enumerate() {
        let mut num_wins: usize = 0;
        let mut winning_numbers_set: HashSet<u32> = HashSet::new();
        let card_data: Vec<_> = card.split(":").collect::<Vec<_>>()[1].split("|").collect();
        let winning_numbers = card_data[0];
        let my_card = card_data[1];
        number_regex.find_iter(winning_numbers).for_each(|n| {winning_numbers_set.insert(n.as_str().parse().unwrap());});
        number_regex.find_iter(my_card).for_each(|n| {
            if winning_numbers_set.contains(&n.as_str().parse().unwrap()) {
                num_wins += 1;
            }
        });
        let num_copies = *copy_data.get(i).unwrap();
        for j in (i+1)..(i+1+num_wins) {
            match copy_data.get_mut(j) {
                None => copy_data.push(num_copies),
                Some(c) => *c += num_copies, 
            }
        }
        total_copies += num_copies;
    }
    println!("My total number of cards is: {total_copies}");
}