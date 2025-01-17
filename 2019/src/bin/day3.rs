use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy)]
struct Coord {
    signal_delay: usize,
    x: i32,
    y: i32,
}

// Big hacks to make the HashSet ignore the signal delay when comparing/hashing Coords
impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Coord {}

impl Hash for Coord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn get_route<T: BufRead>(reader: &mut std::io::Lines<T>) -> Vec<(char, i32)> {
    reader
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| (s.chars().next().unwrap(), s[1..].parse::<i32>().unwrap()))
        .collect()
}

fn get_change_dir(dir: char) -> Option<(i32, i32)> {
    match dir {
        'U' => Some((0, 1)),
        'L' => Some((1, 0)),
        'D' => Some((0, -1)),
        'R' => Some((-1, 0)),
        _ => None,
    }
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(
        File::open(
            env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines();

    let route1 = get_route(&mut reader);
    let route2 = get_route(&mut reader);

    let mut visited_coords: HashSet<Coord> = HashSet::new();
    let mut cur_pos = Coord {
        signal_delay: 0,
        x: 0,
        y: 0,
    };
    for (dir, dist) in &route1 {
        let change = get_change_dir(*dir).unwrap();
        for _ in 0..*dist {
            cur_pos = Coord {
                signal_delay: cur_pos.signal_delay + 1,
                x: cur_pos.x + change.0,
                y: cur_pos.y + change.1,
            };
            if !visited_coords.contains(&cur_pos) {
                visited_coords.insert(cur_pos);
            }
        }
    }

    cur_pos = Coord {
        signal_delay: 0,
        x: 0,
        y: 0,
    };
    let mut min_dist = i32::MAX;
    let mut min_signal_delay = usize::MAX;
    for (dir, dist) in &route2 {
        let change = get_change_dir(*dir).unwrap();
        for _ in 0..*dist {
            cur_pos.signal_delay += 1;
            cur_pos.x += change.0;
            cur_pos.y += change.1;
            if visited_coords.contains(&cur_pos) {
                let man_dist = cur_pos.x.abs() + cur_pos.y.abs();
                if man_dist < min_dist {
                    min_dist = man_dist;
                }
                let previous_signal_delay = visited_coords.get(&cur_pos).unwrap().signal_delay;
                if previous_signal_delay + cur_pos.signal_delay < min_signal_delay {
                    min_signal_delay = previous_signal_delay + cur_pos.signal_delay;
                }
            }
        }
    }
    println!("Wires cross at distance: {}", min_dist);
    println!("Minimum signal delay: {}", min_signal_delay);
    Ok(())
}
