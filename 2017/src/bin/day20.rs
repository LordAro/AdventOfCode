extern crate itertools;
extern crate regex;

use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

#[derive(Debug, Eq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn manhattan_dist(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        self.manhattan_dist().cmp(&other.manhattan_dist())
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Particle {
    pos: Point,
    vel: Point,
    acc: Point,
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    let num_re = r"(-?[0-9]+)";
    let num3_re = format!(r"{},{},{}", num_re, num_re, num_re);
    let particle_re = Regex::new(&format!(
        r"p=<{}>, v=<{}>, a=<{}>",
        num3_re, num3_re, num3_re
    ))
    .unwrap();

    let mut input: Vec<Particle> = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let caps = particle_re.captures(&line).unwrap();

            Particle {
                pos: Point {
                    x: caps.get(1).unwrap().as_str().parse().unwrap(),
                    y: caps.get(2).unwrap().as_str().parse().unwrap(),
                    z: caps.get(3).unwrap().as_str().parse().unwrap(),
                },
                vel: Point {
                    x: caps.get(4).unwrap().as_str().parse().unwrap(),
                    y: caps.get(5).unwrap().as_str().parse().unwrap(),
                    z: caps.get(6).unwrap().as_str().parse().unwrap(),
                },
                acc: Point {
                    x: caps.get(7).unwrap().as_str().parse().unwrap(),
                    y: caps.get(8).unwrap().as_str().parse().unwrap(),
                    z: caps.get(9).unwrap().as_str().parse().unwrap(),
                },
            }
        })
        .collect();

    let max_acc_pos = input
        .iter()
        .enumerate()
        .min_by_key(|&(_, part)| &part.acc)
        .unwrap()
        .0;

    println!("Particle with minimum acceleration: {}", max_acc_pos);

    // Could do something involving maths to resolve collisions,
    // but instead just run simulation for a bit
    for _ in 0..100 {
        // Find collision positions
        let collisions: Vec<Point> = input
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.pos, &b.pos))
            .windows(2)
            .filter(|&win| win[0].pos == win[1].pos)
            .map(|win| win[0].pos)
            .collect();

        // Get rid of particles in collision points
        input.retain(|part| !collisions.contains(&part.pos));

        // Update particles
        input = input
            .iter()
            .map(|part| Particle {
                acc: part.acc,
                vel: part.vel + part.acc,
                pos: part.pos + part.vel + part.acc,
            })
            .collect();
    }

    println!("Remaining particles: {}", input.len());
}
