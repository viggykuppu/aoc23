use std::collections::HashSet;

use aocd::*;
use regex::Regex;

#[aocd(2023,4)]
pub fn one() {
    let input = input!();
    let number_regex = Regex::new(r"\d+").unwrap();
    let total_score: u64 = input.lines().fold(0, |acc, card| {
        let mut num_wins: u32 = 0;
        let mut winning_numbers_set: HashSet<u32> = HashSet::new();
        let card_data: Vec<_> = card.split(':').collect::<Vec<_>>()[1].split('|').collect();
        let winning_numbers = card_data[0];
        let my_card = card_data[1];
        number_regex.find_iter(winning_numbers).for_each(|n| {
            winning_numbers_set.insert(n.as_str().parse().unwrap());
        });
        number_regex.find_iter(my_card).for_each(|n| {
            if winning_numbers_set.contains(&n.as_str().parse().unwrap()) {
                num_wins += 1;
            }
        });
        // println!("number of wins: {num_wins}");
        if num_wins != 0 {
            acc + 2_u32.pow(num_wins - 1) as u64
        } else {
            acc
        }
    });
    println!("My total card score is: {total_score}");
}

#[aocd(2023,6)]
pub fn two() {
    let input = input!();
    let number_regex = Regex::new(r"\d+").unwrap();
    let mut copy_data: Vec<u32> = Vec::new();
    let total_copies = input.lines().enumerate().fold(0, |acc, (i, card)| {
        let mut num_wins: usize = 0;
        let mut winning_numbers_set: HashSet<u32> = HashSet::new();
        let card_data: Vec<_> = card.split(':').collect::<Vec<_>>()[1].split('|').collect();
        let winning_numbers = card_data[0];
        let my_card = card_data[1];
        number_regex.find_iter(winning_numbers).for_each(|n| {
            winning_numbers_set.insert(n.as_str().parse().unwrap());
        });
        number_regex.find_iter(my_card).for_each(|n| {
            if winning_numbers_set.contains(&n.as_str().parse().unwrap()) {
                num_wins += 1;
            }
        });
        if copy_data.len() <= i {
            copy_data.push(1);
        }
        let num_copies = *copy_data.get(i).unwrap();
        for j in (i + 1)..(i + 1 + num_wins) {
            match copy_data.get_mut(j) {
                None => copy_data.push(1 + num_copies),
                Some(c) => *c += num_copies,
            }
        }
        acc + num_copies
    });
    println!("My total number of cards is: {total_copies}");
}
