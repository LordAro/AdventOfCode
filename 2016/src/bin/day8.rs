extern crate regex;

use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().cloned().collect::<String>());
    }
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }
    let input = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap());

    let mut grid = vec![vec![' '; 50]; 6]; // 50x6

    let rect_re = Regex::new(r"rect ([0-9]+)x([0-9]+)").unwrap(); // rect 1x1
    let rot_re = Regex::new(r"rotate (row|column) ([xy])=([0-9]+) by ([0-9]+)").unwrap(); // rotate row y=0 by 7

    for line in input.lines() {
        let l = line.unwrap();
        let cap = rect_re.captures(&l);
        if let Some(caps) = cap {
            let x: usize = caps.at(1).unwrap().parse().unwrap();
            let y: usize = caps.at(2).unwrap().parse().unwrap();

            for row in grid.iter_mut().take(y) {
                for cell in row.iter_mut().take(x) {
                    *cell = '#';
                }
            }
        } else {
            let caps = rot_re.captures(&l).unwrap();
            let dir: char = caps.at(2).unwrap().chars().next().unwrap();
            let idx: usize = caps.at(3).unwrap().parse().unwrap();
            let dist: usize = caps.at(4).unwrap().parse().unwrap();

            match dir {
                'x' => {
                    let orig: Vec<_> = grid
                        .iter()
                        .cloned()
                        .map(|row| row.into_iter().nth(idx).unwrap())
                        .collect();
                    for i in 0..grid.len() {
                        grid[(i + dist) % orig.len()][idx] = orig[i]
                    }
                }
                'y' => {
                    let orig = grid[idx].clone();
                    for i in 0..grid[idx].len() {
                        grid[idx][(i + dist) % orig.len()] = orig[i]
                    }
                }
                _ => panic!("Unknown dir: {}", dir),
            }
        }
    }

    let count: usize = grid
        .iter()
        .map(|row| row.iter().filter(|&&e| e == '#').count())
        .sum();
    println!("Number of lights: {}", count);
    println!("Read the code yourself:");
    print_grid(&grid);
}
