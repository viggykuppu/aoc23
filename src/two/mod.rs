use std::{fs::File, io::Read};

use regex::{Regex, Captures};

use crate::lib;

pub fn two_one() {
    let game_data = lib::read_input("src/two/input.txt");
    let max_green = 13;
    let max_red = 12;
    let max_blue = 14;
    let mut sum = 0;
    for (_, game) in game_data.lines().enumerate() {
        let game_id_regex = Regex::new(r"Game (\d+):").unwrap();
        let game_id = game_id_regex.captures(game).unwrap()[1].parse::<u32>().unwrap();
        let mut valid_game = true;
      //   println!("Game id: {game_id} ---------");

        for (_, game) in game.split(";").enumerate() {
            let (r,g,b) = get_color_values(game);
            //  println!("g: {g}, b: {b}, r: {r}");
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
    let game_data = lib::read_input("src/two/input.txt");
    let mut sum = 0;
    for (_, game) in game_data.lines().enumerate() {
        let mut min_g = 0;
        let mut min_r = 0;
        let mut min_b = 0;
      //   println!("Game id: {game_id} ---------");

        for (_, game) in game.split(";").enumerate() {
            //  println!("g: {g}, b: {b}, r: {r}");
            let (r,g,b) = get_color_values(game);
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

fn get_color_values(haystack: &str) -> (u32, u32, u32) {
   let game_regex = Regex::new(r"(?<g>\d+) green|(?<r>\d+) red|(?<b>\d+) blue").unwrap();
   let caps: Vec<_> = game_regex.captures_iter(haystack).collect();
   let (mut r,mut g,mut b) = (0,0,0);
   caps.iter().for_each(|c| {
      get_color_value(c, "r", &mut r);
      get_color_value(c, "g", &mut g);
      get_color_value(c, "b", &mut b);
   });
   return (r,g,b);
}

fn get_color_value(c: &Captures<'_>, name: &str, v: &mut u32) {
   if let Some(n) = c.name(name) {
      *v += n.as_str().parse::<u32>().unwrap();
   }
}