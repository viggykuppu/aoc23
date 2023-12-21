use std::collections::HashMap;
use num::Integer;

use aocd::*;
use regex::Regex;

#[aocd(2023, 20)]
pub fn one() {
    let binding = input!();
    let mut modules = get_modules(&binding);
    let mut message_queue = Vec::new();
    let mut total_low_pulses = 0;
    let mut total_high_pulses = 0;
    for i in 0..1000 {
        message_queue.push(Pulse {
            v: false,
            from: "button",
            to: "broadcaster",
        });
        while let Some(message) = message_queue.pop() {
            // println!("trying to send message to: {}", message.to);
            if message.v {
                total_high_pulses += 1;
            } else {
                total_low_pulses += 1;
            }
            if let Some(module) = modules.get_mut(message.to) {
                module.as_mut().process(message, &mut message_queue);
            }
        }
    }
    submit!(1, total_low_pulses*total_high_pulses);
}

#[aocd(2023, 20)]
pub fn two() {
    let binding = input!();
    let mut modules = get_modules(&binding);
    let mut message_queue = Vec::new();
    let mut num_button_presses = 1;
    let mut cycle_detection_map: HashMap<&str, usize> = HashMap::new();
    let mut cycle_map: HashMap<&str, usize> = HashMap::new();
    let mut offset_map: HashMap<&str, usize> = HashMap::new();
    let mut kc_map: HashMap<&str, bool>;
    'outer: loop {
        message_queue.push(Pulse {
            v: false,
            from: "button",
            to: "broadcaster",
        });
        while let Some(message) = message_queue.pop() {
            // Cycle detection code
            if message.to == "kc"  {
                if message.v {
                    if let Some(entry) = cycle_detection_map.get(message.from) {
                        if *entry != num_button_presses {
                            cycle_map.insert(message.from, num_button_presses - entry);
                        }
                    } else {
                        offset_map.insert(message.from, num_button_presses);
                    }
                    cycle_detection_map.insert(message.from, num_button_presses);
                    if cycle_map.len() == 4 {
                        break 'outer;
                    }
                }
            }
            if let Some(module) = modules.get_mut(message.to) {
                module.as_mut().process(message, &mut message_queue);
            };
        }
        num_button_presses += 1; 
    }

    let minimum_button_presses = cycle_map.values().fold(1, |acc, offset| {
        acc.lcm(offset)
    });
    submit!(2, minimum_button_presses);
}

fn get_modules<'a>(input: &'a String) -> HashMap<&'a str, Box<dyn Module<'a> + 'a>> {
    let mut modules: HashMap<&str, Box<dyn Module>> = HashMap::new();
    let module_regex = Regex::new(r"([\&%])*(.*) -> (.*)").unwrap();
    let mut conjunction_inputs: HashMap<&str, Vec<&str>> = HashMap::new();
    input.lines().for_each(|line| {
        let caps = module_regex.captures_iter(line).collect::<Vec<_>>();
        let caps = caps.get(0).unwrap();
        if let Some(op) = caps.get(1) {
            if op.as_str() == "&" {
                conjunction_inputs.insert(caps.get(2).unwrap().as_str(), Vec::new());
            }
        }
    });
    input.lines().for_each(|line| {
        let caps = module_regex.captures_iter(line).collect::<Vec<_>>();
        let caps = caps.get(0).unwrap();
        let outputs = caps.get(3).unwrap();
        let outputs = outputs.as_str().split(',').map(|m| m.trim());
        for output in outputs {
            if let Some(entry) = conjunction_inputs.get_mut(output) {
                entry.push(&caps.get(2).unwrap().as_str());
            }
        }
    });
    input.lines().for_each(|line| {
        let caps = module_regex.captures_iter(line).collect::<Vec<_>>();
        let caps = caps.get(0).unwrap();
        let outputs: Vec<_> = caps.get(3).unwrap().as_str().split(',').map(|m| m.trim()).collect();
        let label = caps.get(2).unwrap().as_str();
        if let Some(op) = caps.get(1) {
            match op.as_str() {
                "&" => {
                    let mut memory = HashMap::new();
                    for input in conjunction_inputs.get(label).unwrap() {
                        memory.insert(*input, false);
                    }
                    modules.insert(label, Box::new(Conjunction {
                        label: label,
                        outputs: outputs,
                        memory: memory,
                    }));
                },
                "%" => {
                    modules.insert(label, Box::new(FlipFlop {
                        label: label,
                        state: false,
                        outputs: outputs,
                    }));
                },
                _ => unreachable!()
            }
        } else {
            modules.insert(caps.get(2).unwrap().as_str(), Box::new(Broadcaster {
                outputs: outputs,
            }));
        }
    });
    modules
}

#[derive(Debug)]
struct Pulse<'a> {
    v: bool,
    from: &'a str,
    to: &'a str,
}

struct Broadcaster<'a> {
    outputs: Vec<&'a str>
}

impl<'a> Module<'a> for Broadcaster<'a> {
    fn process(&mut self, msg: Pulse, queue: &mut Vec<Pulse<'a>>) {
        for output in &self.outputs {
            let new_pulse = Pulse {
                v: false,
                from: "broadcaster",
                to: output,
            };
            queue.insert(0, new_pulse);
        }
    }

    fn print(&self) -> String {
        format!("broadcaster! outputs: {:?}", self.outputs)
    }

    fn get_memory(&self) -> Option<&HashMap<&'a str, bool>> {
        None
    }
}

struct FlipFlop<'a> {
    label: &'a str,
    state: bool,
    outputs: Vec<&'a str>,
}

impl<'a> Module<'a> for FlipFlop<'a> {
    fn process(&mut self, msg: Pulse, queue: &mut Vec<Pulse<'a>>) {
        if !msg.v {
            self.state = !self.state;
            for output in &self.outputs {
                let new_pulse = Pulse {
                    v: self.state,
                    from: self.label,
                    to: output,
                };
                queue.insert(0, new_pulse);
            }
        }
    }

    fn print(&self) -> String {
        format!("flipflop! label: {}, state: {:?}, outputs: {:?}", self.label, self.state, self.outputs)
    }

    fn get_memory(&self) -> Option<&HashMap<&'a str, bool>> {
        None
    }
}

struct Conjunction<'a> {
    label: &'a str,
    memory: HashMap<&'a str, bool>,
    outputs: Vec<&'a str>,
}

impl<'a> Module<'a> for Conjunction<'a> {
    fn process(&mut self, msg: Pulse<'a>, queue: &mut Vec<Pulse<'a>>) {
        self.memory.insert(msg.from, msg.v);
        let mut is_memory_all_high_pulses = true;
        for v in self.memory.values() {
            is_memory_all_high_pulses = is_memory_all_high_pulses && *v;
        }
        for output in &self.outputs {
            let new_pulse = Pulse {
                v: !is_memory_all_high_pulses,
                from: self.label,
                to: output,
            };
            queue.insert(0, new_pulse);
        }
    }

    fn print(&self) -> String {
        format!("conjunction! label: {}, memory: {:?}, outputs: {:?}", self.label, self.memory, self.outputs)
    }

    fn get_memory(&self) -> Option<&HashMap<&'a str, bool>> {
        Some(&self.memory)
    }
}

trait Module<'a> {
    fn process(&mut self, msg: Pulse<'a>, queue: &mut Vec<Pulse<'a>>);
    fn get_memory(&self) -> Option<&HashMap<&str, bool>>;
    fn print(&self) -> String;
}