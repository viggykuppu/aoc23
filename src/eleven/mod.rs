use std::{collections::HashSet, ops::ControlFlow};

use aocd::*;

#[aocd(2023,11)]
pub fn one() {
    let binding = input!();
    let mut data: Vec<Vec<char>> = binding.lines().map(|line| line.chars().collect()).collect();
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    data.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, c)| {
            if *c == '#' {
                galaxies.push((i,j));
            }
        });
    });

    // add expanded columns
    let initial_galaxy_columns: HashSet<usize> = HashSet::from_iter(galaxies.iter().map(|g| g.1));
    let mut empty_columns: Vec<_> = (0..data[0].len()).filter(|i| !initial_galaxy_columns.contains(i)).collect();
    empty_columns.sort();
    empty_columns.iter().enumerate().for_each(|(x, e)| {
        for i in 0..data.len() {
            data[i].insert(*e+x, '.');
        }
    });
    // add expanded rows
    let initial_galaxy_rows: HashSet<usize> = HashSet::from_iter(galaxies.iter().map(|g| g.0));
    let mut empty_rows: Vec<_> = (0..data.len()).filter(|i| !initial_galaxy_rows.contains(i)).collect();
    empty_rows.sort();
    empty_rows.iter().enumerate().for_each(|(x, e)| {
        let mut new_row: Vec<char> = Vec::new();
        for _ in 0..data[0].len() {
            new_row.push('.');
        }
        data.insert(*e+x, new_row);
    });

    // rebuild galaxies with new positions
    galaxies.clear();
    data.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, c)| {
            if *c == '#' {
                galaxies.push((i,j));
            }
        });
    });

    let mut total_distance = 0;
    for i in 0..galaxies.len() {
        for j in (i+1)..galaxies.len() {
            let g1 = galaxies[i];
            let g2 = galaxies[j];
            let distance = (g1.0 as i32 - g2.0 as i32).abs() + (g1.1 as i32 - g2.1 as i32).abs();
            total_distance += distance;
        }
    }

    galaxies.sort_by(|g1, g2| {
        return g1.1.cmp(&g2.1);
    });
    submit!(1, total_distance);
}

#[aocd(2023,11)]
pub fn two() {
    let binding = input!();
    let data: Vec<Vec<char>> = binding.lines().map(|line| line.chars().collect()).collect();
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    data.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, c)| {
            if *c == '#' {
                galaxies.push((i,j));
            }
        });
    });

    let expansion_factor = 999999_usize;

    // add expanded columns
    let initial_galaxy_columns: HashSet<usize> = HashSet::from_iter(galaxies.iter().map(|g| g.1));
    let mut empty_columns: Vec<_> = (0..data[0].len()).filter(|i| !initial_galaxy_columns.contains(i)).collect();
    // sort galaxies by column value to go from left to right in galaxy columns
    galaxies.sort_by(|g1, g2| {
        return g1.1.cmp(&g2.1);
    });
    empty_columns.sort();
    galaxies.iter_mut().for_each(|galaxy| {
        empty_columns.iter().enumerate().try_for_each(|(x, e)| {
            if galaxy.1 < *e {
                galaxy.1 += x*expansion_factor;
                return ControlFlow::Break(());
            } else if x == empty_columns.len() - 1 {
                galaxy.1 += (x+1_usize)*expansion_factor;
                return ControlFlow::Break(());
            }
            ControlFlow::Continue(())
        });
    });

    // // add expanded rows
    let initial_galaxy_rows: HashSet<usize> = HashSet::from_iter(galaxies.iter().map(|g| g.0));
    let mut empty_rows: Vec<_> = (0..data.len()).filter(|i| !initial_galaxy_rows.contains(i)).collect();
    // sort galaxies by row value to go from left to right in galaxy columns
    galaxies.sort_by(|g1, g2| {
        return g1.0.cmp(&g2.0);
    });
    empty_rows.sort();
    galaxies.iter_mut().for_each(|galaxy| {
        empty_rows.iter().enumerate().try_for_each(|(x, e)| {
            if galaxy.0 < *e {
                galaxy.0 += x*expansion_factor;
                return ControlFlow::Break(());
            } else if x == empty_rows.len() - 1 {
                galaxy.0 += (x+1_usize)*expansion_factor;
                return ControlFlow::Break(());
            }
            ControlFlow::Continue(())
        });
    });
    galaxies.sort_by(|g1, g2| {
        return g1.1.cmp(&g2.1);
    });
    
    let mut total_distance = 0;
    for i in 0..galaxies.len() {
        for j in (i+1)..galaxies.len() {
            let g1 = galaxies[i];
            let g2 = galaxies[j];
            let distance = (g1.0 as i64 - g2.0 as i64).abs() + (g1.1 as i64 - g2.1 as i64).abs();
            total_distance += distance;
        }
    }
    submit!(2, total_distance);
}

fn print_galaxies(galaxies: &Vec<(usize,usize)>) {
    galaxies.iter().for_each(|g| {
        print!("{:?}",g);
    });
    println!("");
}