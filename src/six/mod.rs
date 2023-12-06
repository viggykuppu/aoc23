use regex::Regex;
use aocd::*;

#[aocd(2023,6)]
pub fn one() {
    let input= input!();
    let lines:Vec<_> = input.lines().collect();
    let number_regex = Regex::new(r"\d+").unwrap();
    let times:Vec<_> = number_regex.find_iter(lines.first().unwrap()).map(|time| time.as_str().parse::<u32>().unwrap()).collect();
    let records:Vec<_> = number_regex.find_iter(lines.last().unwrap()).map(|time| time.as_str().parse::<u32>().unwrap()).collect();
    
    let record_beats = times.iter().enumerate().fold(1, |acc, (i, time)|{
        let num_records =(1..(*time as usize)).fold(0, |acc, speed_usize| {
            let speed = speed_usize as u32;
            let distance = speed*(time-speed);
            if distance > *records.get(i).unwrap() {
                acc + 1
            } else {
                acc
            }
        });
        acc*num_records
    });
    submit!(1, record_beats);
}

#[aocd(2023,6)]
pub fn two() {
    let input= input!();
    let lines:Vec<_> = input.lines().collect();
    let number_regex = Regex::new(r"\d+").unwrap();

    let time = number_regex.find_iter(lines.first().unwrap()).fold(String::new(), |acc, time_chunk| {
        acc + time_chunk.as_str()
    }).parse::<u64>().unwrap();

    let record = number_regex.find_iter(lines.last().unwrap()).fold(String::new(), |acc, record_chunk| {
        acc + record_chunk.as_str()
    }).parse::<u64>().unwrap();
    
    let record_beats = (1..(time as usize)).fold(0, |acc, speed_usize| {
        let speed = speed_usize as u64;
        let distance = speed*(time-speed);
        if distance > record {
            acc + 1
        } else {
            acc
        }
    });
    submit!(2, record_beats);
}