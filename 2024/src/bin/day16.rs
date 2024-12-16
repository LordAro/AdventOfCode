use std::cmp::Ordering;
use std::collections::{BTreeSet, HashSet};
use std::env;
use std::fs;
use std::io;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PathNode {
    cost: usize,
    coord: Coord,
    dir: Dir,
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost
            .cmp(&other.cost)
            .then(self.coord.cmp(&other.coord))
            .then(self.dir.cmp(&other.dir))
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_grid(input: &str) -> (HashSet<Coord>, Coord, Coord) {
    let mut start = None;
    let mut end = None;
    let mut map = HashSet::default();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = Coord { x, y };
            if c == 'S' {
                start = Some(coord);
            } else if c == 'E' {
                end = Some(coord);
            } else if c == '#' {
                map.insert(coord);
            }
        }
    }
    (map, start.unwrap(), end.unwrap())
}

fn find_shortest_path(
    wall_map: &HashSet<Coord>,
    start_pos: Coord,
    end_pos: Coord,
) -> Option<usize> {
    let mut to_search = BTreeSet::new();
    to_search.insert(PathNode {
        cost: 0,
        coord: start_pos,
        dir: Dir::Right,
    });
    let mut searched: HashSet<_> = HashSet::new();

    while let Some(pn) = to_search.pop_first() {
        //println!("{pn:?}");
        if pn.coord == end_pos {
            return Some(pn.cost);
        }

        searched.insert((pn.coord, pn.dir)); // never faster to go back over where we've already been

        let next_coord = match pn.dir {
            Dir::Up => Coord {
                x: pn.coord.x,
                y: pn.coord.y - 1,
            },
            Dir::Down => Coord {
                x: pn.coord.x,
                y: pn.coord.y + 1,
            },
            Dir::Left => Coord {
                x: pn.coord.x - 1,
                y: pn.coord.y,
            },
            Dir::Right => Coord {
                x: pn.coord.x + 1,
                y: pn.coord.y,
            },
        };
        if !wall_map.contains(&next_coord) && !searched.contains(&(next_coord, pn.dir)) {
            to_search.insert(PathNode {
                cost: pn.cost + 1,
                coord: next_coord,
                dir: pn.dir,
            });
        }

        let left = pn.dir.turn_left();
        if !searched.contains(&(pn.coord, left)) {
            to_search.insert(PathNode {
                cost: pn.cost + 1000,
                coord: pn.coord,
                dir: left,
            });
        }
        let right = pn.dir.turn_right();
        if !searched.contains(&(pn.coord, left)) {
            to_search.insert(PathNode {
                cost: pn.cost + 1000,
                coord: pn.coord,
                dir: right,
            });
        }
    }
    None
}

fn main() -> io::Result<()> {
    let (grid, start_pos, end_pos) = parse_grid(&fs::read_to_string(
        env::args().nth(1).expect("missing cli argument"),
    )?);
    let min_path_distance = find_shortest_path(&grid, start_pos, end_pos).unwrap();
    println!("P1: Shortest path: {min_path_distance}");
    Ok(())
}
