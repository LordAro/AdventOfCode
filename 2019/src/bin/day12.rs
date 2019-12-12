use std::collections::HashSet;
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

fn gcd(a: isize, b: isize) -> isize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (isize::min(x, y), isize::max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: isize, b: isize) -> isize {
    (a * b).abs() / gcd(a, b)
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

fn get_next_state(previous: &Vec<Moon>) -> Vec<Moon> {
    let mut moons = previous.clone();
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
    moons
}

fn main() {
    let initial_state: Vec<_> = BufReader::new(
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
    //let moons = vec![
    //    Moon {
    //        pos: Coord { x: -1, y: 0, z: 2 },
    //        velo: Coord { x: 0, y: 0, z: 0 },
    //    },
    //    Moon {
    //        pos: Coord {
    //            x: 2,
    //            y: -10,
    //            z: -7,
    //        },
    //        velo: Coord { x: 0, y: 0, z: 0 },
    //    },
    //    Moon {
    //        pos: Coord { x: 4, y: -8, z: 8 },
    //        velo: Coord { x: 0, y: 0, z: 0 },
    //    },
    //    Moon {
    //        pos: Coord { x: 3, y: 5, z: -1 },
    //        velo: Coord { x: 0, y: 0, z: 0 },
    //    },
    //];

    let mut moons = initial_state.clone();
    for _ in 0..1000 {
        moons = get_next_state(&moons);
    }
    let total_energy: isize = moons
        .iter()
        .map(|m| m.potential_energy() * m.kinetic_energy())
        .sum();
    println!("Total energy of system: {}", total_energy);

    let mut states_x: HashSet<Vec<_>> = HashSet::new();
    let mut states_y: HashSet<Vec<_>> = HashSet::new();
    let mut states_z: HashSet<Vec<_>> = HashSet::new();
    let mut repeat_x = 0;
    let mut repeat_y = 0;
    let mut repeat_z = 0;

    let mut moons = initial_state.clone();
    let mut i = 0;
    loop {
        let xs = moons.iter().flat_map(|m| vec![m.pos.x, m.velo.x]).collect();
        let ys = moons.iter().flat_map(|m| vec![m.pos.y, m.velo.y]).collect();
        let zs = moons.iter().flat_map(|m| vec![m.pos.z, m.velo.z]).collect();
        if repeat_x == 0 {
            if !states_x.contains(&xs) {
                states_x.insert(xs);
            } else {
                repeat_x = i;
            }
        }
        if repeat_y == 0 {
            if !states_y.contains(&ys) {
                states_y.insert(ys);
            } else {
                repeat_y = i;
            }
        }
        if repeat_z == 0 {
            if !states_z.contains(&zs) {
                states_z.insert(zs);
            } else {
                repeat_z = i;
            }
        }
        if repeat_x != 0 && repeat_y != 0 && repeat_z != 0 {
            break;
        }

        moons = get_next_state(&moons);
        i += 1;
    }
    let repeat = lcm(lcm(repeat_x, repeat_y), repeat_z);
    println!("Universe repeats after {} steps", repeat);
}
