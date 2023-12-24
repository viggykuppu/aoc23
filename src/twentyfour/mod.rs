use aocd::*;
use itertools::Itertools;
use nalgebra::{SVector, Matrix3x2, Matrix3, Matrix2};
use num::iter::RangeInclusive;
use regex::Regex;

#[aocd(2023,24, "src/twentyfour/input.txt")]
pub fn one() {
    let binding = input!();
    let hail_regex = Regex::new(r"(-{0,1}\d+, -{0,1}\d+, -{0,1}\d+) @ (-{0,1}\d+, -{0,1}\d+, -{0,1}\d+)").unwrap();
    let hail: Vec<_> = binding.lines().map(|line| {
        let caps: Vec<_> = hail_regex.captures_iter(line).collect();
        let position: Vec<f64> = caps[0][1].split(",").map(|p| {
            p.trim().parse::<f64>().unwrap()
        }).collect();
        let velocity: Vec<f64> = caps[0][2].split(",").map(|p| {
            p.trim().parse::<f64>().unwrap()
        }).collect();
        HailStone {
            position: SVector::<f64, 3>::new(position[0],position[1],position[2]),
            velocity: SVector::<f64, 3>::new(velocity[0],velocity[1],velocity[2]),
        }
    }).collect();
    // println!("{:?}", hail);
    let mut total_collisions = 0;
    for i in 0..hail.len() {
        for j in (i+1)..hail.len() {
            let me = hail.get(i).unwrap();
            let other = hail.get(j).unwrap();
            // if me.collides(other, (7.0,27.0)) {
            if me.collides(other, (200000000000000.0,400000000000000.0)) {
                // println!("collided {i} and {j}");
                total_collisions += 1;
            }
        }
    }
    println!("total collisions: {total_collisions}");
    // submit!(1, total_collisions);
}

#[aocd(2023,24)]
pub fn two() {
    let binding = input!();
}

#[derive(Debug)]
struct HailStone {
    position: SVector<f64, 3>,
    velocity: SVector<f64, 3>,
}

impl HailStone {    
    fn collides(&self, other: &HailStone, bounds: (f64, f64)) -> bool {
        let p = other.position.xy() - self.position.xy();

        let z = Matrix2::from_columns(&[
            self.velocity.xy(),
            -1.0*other.velocity.xy()
        ]);
        let xy = z.lu().solve(&p);
        
        if let Some(xy) = xy {  
            let x = self.position.x + xy.x * self.velocity.x;
            let y = self.position.y + xy.x * self.velocity.y;   
            // println!("{}, {}", self.position.y + xy.x * self.velocity.y, other.position.y + xy.y * other.velocity.y);
            if (x - bounds.0).abs() < 100.0 {
                println!("close x to bounds");
            }
            if (y - bounds.0).abs() < 100.0 {
                println!("close y to bounds");
            }
            return xy.x >= 0.0 && xy.y >= 0.0 && x >= bounds.0 && x <= bounds.1 && y >= bounds.0 && y <= bounds.1;
        }
        
        return false;
    }
}