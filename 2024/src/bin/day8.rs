use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::io;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coord {
    type Output = Self;
    fn sub(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i32> for Coord {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Coord {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

fn parse_input(input: &str) -> (HashMap<char, Vec<Coord>>, Coord) {
    let mut antennas: HashMap<char, Vec<Coord>> = HashMap::default();
    let mut max_coord = Coord { x: 0, y: 0 };
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '.' => (),
                _ => antennas.entry(c).or_default().push(Coord {
                    x: x as i32,
                    y: y as i32,
                }),
            }
            max_coord = Coord {
                x: x as i32,
                y: y as i32,
            };
        }
    }
    (antennas, max_coord)
}

// array so we can use flat_map
fn get_antinode_loc(a: Coord, b: Coord) -> [Coord; 2] {
    let diff = Coord {
        x: a.x - b.x,
        y: a.y - b.y,
    };
    [a + diff, b - diff]
}

fn get_antinode_locs(antennas: &HashMap<char, Vec<Coord>>) -> HashSet<Coord> {
    antennas
        .values()
        .flat_map(|antenna_locs| {
            antenna_locs
                .iter()
                .combinations(2)
                .flat_map(|pair| get_antinode_loc(*pair[0], *pair[1]))
        })
        .collect()
}

fn get_resonate_antinode_loc(a: Coord, b: Coord, max: Coord) -> Vec<Coord> {
    let diff = Coord {
        x: a.x - b.x,
        y: a.y - b.y,
    };
    let mut antinodes = Vec::new();
    for n in 0.. {
        let antinode = a + diff * n;
        if antinode.x < 0 || antinode.y < 0 || antinode.x > max.x || antinode.y > max.y {
            break;
        }
        antinodes.push(antinode);
    }
    for n in 0.. {
        let antinode = b - diff * n;
        if antinode.x < 0 || antinode.y < 0 || antinode.x > max.x || antinode.y > max.y {
            break;
        }
        antinodes.push(antinode);
    }
    antinodes
}

fn get_resonate_antinode_locs(
    antennas: &HashMap<char, Vec<Coord>>,
    max_coord: Coord,
) -> HashSet<Coord> {
    antennas
        .values()
        .flat_map(|antenna_locs| {
            antenna_locs
                .iter()
                .combinations(2)
                .flat_map(|pair| get_resonate_antinode_loc(*pair[0], *pair[1], max_coord))
        })
        .collect()
}

fn main() -> io::Result<()> {
    let (input_antennas, max_coord) = parse_input(&fs::read_to_string(
        env::args().nth(1).expect("missing cli argument"),
    )?);

    let antinode_locs = get_antinode_locs(&input_antennas);
    let num_antinodes_bounded: usize = antinode_locs
        .iter()
        .filter(|c| c.x >= 0 && c.y >= 0 && c.x <= max_coord.x && c.y <= max_coord.y)
        .count();
    println!("P1: Number of unique antinode locations: {num_antinodes_bounded}");

    let resonate_antinode_locs = get_resonate_antinode_locs(&input_antennas, max_coord);
    println!(
        "P2: Number of unique resonate antinode locations: {}",
        resonate_antinode_locs.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EX_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn ex1() {
        let (input_antennas, max_coord) = parse_input(EX_INPUT);
        let antinode_locs = get_antinode_locs(&input_antennas);
        let num_antinodes_bounded: usize = antinode_locs
            .iter()
            .filter(|c| c.x >= 0 && c.y >= 0 && c.x <= max_coord.x && c.y <= max_coord.y)
            .count();
        assert_eq!(num_antinodes_bounded, 14);
    }
    #[test]
    fn ex2() {
        let (input_antennas, max_coord) = parse_input(EX_INPUT);
        let antinode_locs = get_resonate_antinode_locs(&input_antennas, max_coord);
        assert_eq!(antinode_locs.len(), 34);
    }
}
