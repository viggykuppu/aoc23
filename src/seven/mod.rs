use aocd::*;
use regex::Regex;

mod hand;

#[aocd(2023, 7)]
pub fn one() {
    let input = input!();
    let hand_regex = Regex::new(r"(.+) (\d+)").unwrap();
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            let caps: Vec<_> = hand_regex
                .captures_iter(line)
                .map(|c| c.extract::<2>())
                .collect();
            let mut hand_chars: Vec<_> = caps.get(0).unwrap().1[0]
                .chars()
                .map(|c| {
                    return if c == 'T' {
                        'a'
                    } else if c == 'J' {
                        'b'
                    } else if c == 'Q' {
                        'c'
                    } else if c == 'K' {
                        'd'
                    } else if c == 'A' {
                        'e'
                    } else {
                        c
                    };
                })
                .collect();
            hand::Hand::new(
                hand_chars,
                caps.get(0).unwrap().1[1].parse::<u32>().unwrap(),
            )
        })
        .collect();
    hands.sort();
    let total_points = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i as u32 + 1) * hand.bid);
    submit!(1, total_points);
}

#[aocd(2023, 7)]
pub fn two() {
    let input = input!();
    let hand_regex = Regex::new(r"(.+) (\d+)").unwrap();
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            let caps: Vec<_> = hand_regex
                .captures_iter(line)
                .map(|c| c.extract::<2>())
                .collect();
            let mut hand_chars: Vec<_> = caps.get(0).unwrap().1[0]
                .chars()
                .map(|c| {
                    return if c == 'T' {
                        'a'
                    } else if c == 'J' {
                        '1'
                    } else if c == 'Q' {
                        'c'
                    } else if c == 'K' {
                        'd'
                    } else if c == 'A' {
                        'e'
                    } else {
                        c
                    };
                })
                .collect();
            hand::Hand2::new(
                hand_chars,
                caps.get(0).unwrap().1[1].parse::<u32>().unwrap(),
            )
        })
        .collect();
    hands.sort();
    let total_points = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i as u32 + 1) * hand.bid);
    submit!(2, total_points);
}
