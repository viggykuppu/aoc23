use std::{fs::File, io::Read};

use regex::Regex;


pub fn two_one() {
    let game_data = read_input();
    let max_green = 13;
    let max_red = 12;
    let max_blue = 14;
    let mut sum = 0;
    for (_, game) in game_data.lines().enumerate() {
        let game_id_regex = Regex::new(r"Game (\d+):").unwrap();
        let game_id = game_id_regex.captures(game).unwrap()[1].parse::<u32>().unwrap();
        let mut valid_game = true;
        println!("Game id: {game_id} ---------");

        for (_, game) in game.split(";").enumerate() {
             let g = match Regex::new(r"(\d+) green").unwrap().captures(game) {
                None => 0,
                Some(cap) => cap[1].parse::<u32>().unwrap()
             };
             let r = match Regex::new(r"(\d+) red").unwrap().captures(game) {
                None => 0,
                Some(cap) => cap[1].parse::<u32>().unwrap()
             };
             let b = match Regex::new(r"(\d+) blue").unwrap().captures(game) {
                None => 0,
                Some(cap) => cap[1].parse::<u32>().unwrap()
             };
             println!("g: {g}, b: {b}, r: {r}");
             if g > max_green || b > max_blue || r > max_red {
                valid_game = false;
             }
        }
        if valid_game {
            sum += game_id;
        }
    }
    println!("Valid gameid sum is: {sum}");
}

pub fn two_two() {
    let game_data = read_input();
    let mut sum = 0;
    for (_, game) in game_data.lines().enumerate() {
        let mut power = 0;
        let mut min_g = 0;
        let mut min_r = 0;
        let mut min_b = 0;
        let game_id_regex = Regex::new(r"Game (\d+):").unwrap();
        let game_id = game_id_regex.captures(game).unwrap()[1].parse::<u32>().unwrap();
        println!("Game id: {game_id} ---------");

        for (_, game) in game.split(";").enumerate() {
             let g = match Regex::new(r"(\d+) green").unwrap().captures(game) {
                None => 0,
                Some(cap) => cap[1].parse::<u32>().unwrap()
             };
             let r = match Regex::new(r"(\d+) red").unwrap().captures(game) {
                None => 0,
                Some(cap) => cap[1].parse::<u32>().unwrap()
             };
             let b = match Regex::new(r"(\d+) blue").unwrap().captures(game) {
                None => 0,
                Some(cap) => cap[1].parse::<u32>().unwrap()
             };
             println!("g: {g}, b: {b}, r: {r}");
             if g > min_g {
                min_g = g;
             }
             if b > min_b {
                min_b = b;
             }
             if r > min_r {
                min_r = r;
             }
        }
        sum += min_g*min_r*min_b;
    }
    println!("Game power sum is: {sum}");
}

fn read_input() -> String {
    let mut file = File::open("src/two/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}