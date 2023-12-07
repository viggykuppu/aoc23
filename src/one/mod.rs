use onig::*;
use std::collections::HashMap;

use aocd::*;

#[aocd(2023, 1)]
pub fn one() {
    let input = input!();
    let number_regex = Regex::new(r"\d").unwrap();
    let map = init_map();
    let sum = input.lines().fold(0, |acc, line| {
        let caps: Vec<_> = number_regex
            .find_iter(line)
            .map(|x| &line[x.0..x.1])
            .collect();
        acc + 10 * parse_number(caps[0], &map) + parse_number(caps[caps.len() - 1], &map)
    });
    submit!(1, sum);
}

#[aocd(2023, 1)]
pub fn two() {
    let input = input!();
    let number_regex =
        Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))").unwrap();
    let map = init_map();
    let sum = input.lines().fold(0, |acc: u32, line| {
        let caps: Vec<_> = number_regex.captures_iter(line).collect();
        acc + 10 * parse_number(caps.first().unwrap().at(1).unwrap(), &map)
            + parse_number(caps.last().unwrap().at(1).unwrap(), &map)
    });
    submit!(2, sum);
}

fn parse_number(s: &str, map: &HashMap<&'static str, u32>) -> u32 {
    let number_text_regex = Regex::new(r"one|two|three|four|five|six|seven|eight|nine").unwrap();
    match number_text_regex.find(s) {
        None => s.parse::<u32>().unwrap(),
        Some(_m) => map.get(s).unwrap().to_owned(),
    }
}

fn init_map() -> HashMap<&'static str, u32> {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);
    map
}
