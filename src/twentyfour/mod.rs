use aocd::*;

use nalgebra::{Matrix2, Vector3, Matrix4, Vector4, Matrix1x4, Matrix1};

use regex::Regex;
use z3::{Solver, Context, Config, ast::{Int, Ast}};

#[aocd(2023,24)]
pub fn one() {
    let binding = input!();
    let hail_regex = Regex::new(r"(-{0,1}\d+, -{0,1}\d+, -{0,1}\d+) @ (-{0,1}\d+,(\s)+-{0,1}\d+,(\s)+-{0,1}\d+)").unwrap();
    let hail: Vec<_> = binding.lines().map(|line| {
        let caps: Vec<_> = hail_regex.captures_iter(line).collect();
        let position: Vec<f64> = caps[0][1].split(",").map(|p| {
            p.trim().parse::<f64>().unwrap()
        }).collect();
        let velocity: Vec<f64> = caps[0][2].split(",").map(|p| {
            p.trim().parse::<f64>().unwrap()
        }).collect();
        HailStone {
            position: Vector3::<f64>::new(position[0],position[1],position[2]),
            velocity: Vector3::<f64>::new(velocity[0],velocity[1],velocity[2]),
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
            if me.is_parallel(other) {
                println!("parallel velocities {i} and {j}");
            }
            if me.is_perpendicular(other) {
                println!("perpindicular velocities {i} and {j}");
            }
        }
    }
    submit!(1, total_collisions);
}

#[aocd(2023,24)]
pub fn two() {
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
            position: Vector3::<f64>::new(position[0],position[1],position[2]),
            velocity: Vector3::<f64>::new(velocity[0],velocity[1],velocity[2]),
        }
    }).collect();
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);
    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");
    let z = Int::new_const(&ctx, "z");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    for stone in &hail[0..3] {
        let xi = Int::from_i64(&ctx, stone.position.x as i64);
        let yi = Int::from_i64(&ctx, stone.position.y as i64);
        let zi = Int::from_i64(&ctx, stone.position.z as i64);
        let vxi = Int::from_i64(&ctx, stone.velocity.x as i64);
        let vyi = Int::from_i64(&ctx, stone.velocity.y as i64);
        let vzi = Int::from_i64(&ctx, stone.velocity.z as i64);
        let ti = Int::fresh_const(&ctx, "ti");
        solver.assert(&(&x + &vx * &ti)._eq(&(xi + vxi*&ti)));
        solver.assert(&(&y + &vy * &ti)._eq(&(yi + vyi*&ti)));
        solver.assert(&(&z + &vz * &ti)._eq(&(zi + vzi*&ti)));
    }
    solver.check();
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&x).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&y).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&z).unwrap().as_i64().unwrap();
    submit!(2, x+y+z);
    
}

#[derive(Debug)]
struct HailStone {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
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
            return xy.x >= 0.0 && xy.y >= 0.0 && x >= bounds.0 && x <= bounds.1 && y >= bounds.0 && y <= bounds.1;
        }
        
        return false;
    }

    fn is_parallel(&self, other: &HailStone) -> bool {
        self.velocity.cross(&other.velocity).magnitude() == 0.0
    }

    fn is_perpendicular(&self, other: &HailStone) -> bool {
        let cross = self.velocity.dot(&other.velocity);
        cross == 0.0
    }
}