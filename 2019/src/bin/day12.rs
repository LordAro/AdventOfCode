use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate regex;
use regex::Regex;

extern crate itertools;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Moon {
    pos: Coord,
    velo: Coord,
}

fn get_new_velocity(m1: Moon, m2: Moon) -> Coord {
    Coord {
        x: if m1.pos.x > m2.pos.x {
            m1.velo.x - 1
        } else if m1.pos.x < m2.pos.x {
            m1.velo.x + 1
        } else {
            m1.velo.x
        },
        y: if m1.pos.y > m2.pos.y {
            m1.velo.y - 1
        } else if m1.pos.y < m2.pos.y {
            m1.velo.y + 1
        } else {
            m1.velo.y
        },
        z: if m1.pos.z > m2.pos.z {
            m1.velo.z - 1
        } else if m1.pos.z < m2.pos.z {
            m1.velo.z + 1
        } else {
            m1.velo.z
        },
    }
}

fn get_new_position(old_pos: Coord, velo: Coord) -> Coord {
    Coord {
        x: old_pos.x + velo.x,
        y: old_pos.y + velo.y,
        z: old_pos.z + velo.z,
    }
}

impl Moon {
    fn potential_energy(self) -> isize {
        self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
    }

    fn kinetic_energy(self) -> isize {
        self.velo.x.abs() + self.velo.y.abs() + self.velo.z.abs()
    }
}

fn get_state_after(initial_state: Vec<Moon>, iterations: usize) -> Vec<Moon> {
    let mut moons = initial_state.clone();
    for _ in 0..iterations {
        for moon_idxs in (0..moons.len()).combinations(2) {
            // Ick. Improve this?
            let mut m1 = moons[moon_idxs[0]];
            let mut m2 = moons[moon_idxs[1]];
            m1.velo = get_new_velocity(m1, m2);
            m2.velo = get_new_velocity(m2, m1);
            moons[moon_idxs[0]] = m1;
            moons[moon_idxs[1]] = m2;
        }
        for moon in moons.iter_mut() {
            moon.pos = get_new_position(moon.pos, moon.velo);
        }
    }
    moons
}

fn main() {
    let moons: Vec<_> = BufReader::new(
        File::open(
            &env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .map(|l| {
        let r = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
        let s = l.unwrap();
        let caps = r.captures(&s).unwrap();
        let x = caps.get(1).unwrap().as_str().parse().unwrap();
        let y = caps.get(2).unwrap().as_str().parse().unwrap();
        let z = caps.get(3).unwrap().as_str().parse().unwrap();
        Moon {
            pos: Coord { x: x, y: y, z: z },
            velo: Coord { x: 0, y: 0, z: 0 },
        }
    })
    .collect();

    let state_1000 = get_state_after(moons, 1000);
    let total_energy: isize = state_1000
        .iter()
        .map(|m| m.potential_energy() * m.kinetic_energy())
        .sum();
    println!("Total energy of system: {}", total_energy);
}
