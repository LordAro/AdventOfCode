use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::BTreeSet;
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

// Used for ordered btreeset/binaryheap purposes
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct PathNode {
    cost: usize,
    node: (Coord, Dir),
}

fn parse_grid(input: &str) -> (FxHashSet<Coord>, Coord, Coord) {
    let mut start = None;
    let mut end = None;
    let mut map = FxHashSet::default();
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

fn get_next_coord(c: Coord, d: Dir) -> Coord {
    match d {
        Dir::Up => Coord { x: c.x, y: c.y - 1 },
        Dir::Down => Coord { x: c.x, y: c.y + 1 },
        Dir::Left => Coord { x: c.x - 1, y: c.y },
        Dir::Right => Coord { x: c.x + 1, y: c.y },
    }
}

fn find_all_shortest_paths(
    wall_map: &FxHashSet<Coord>,
    start_pos: Coord,
    end_pos: Coord,
) -> (usize, usize) {
    let mut dist: FxHashMap<(Coord, Dir), usize> = FxHashMap::default();
    dist.insert((start_pos, Dir::Right), 0);
    let mut prev: FxHashMap<(Coord, Dir), Vec<(Coord, Dir)>> = FxHashMap::default();

    let mut min_cost_found = None;
    // Q, ish
    let mut to_search = BTreeSet::from([PathNode {
        cost: 0,
        node: (start_pos, Dir::Right),
    }]);
    let mut searched = FxHashSet::default();
    while let Some(PathNode {
        cost,
        node: (u_pos, u_dir),
    }) = to_search.pop_first()
    {
        if u_pos == end_pos {
            min_cost_found = Some(cost);
            break;
        }

        searched.insert((u_pos, u_dir));

        let next_forward = (get_next_coord(u_pos, u_dir), u_dir);
        let rot_left = (u_pos, u_dir.turn_left());
        let rot_right = (u_pos, u_dir.turn_right());
        for v in [next_forward, rot_left, rot_right] {
            if !wall_map.contains(&v.0) && !searched.contains(&v) {
                let alt = dist[&(u_pos, u_dir)] + if v.1 == u_dir { 1 } else { 1000 };
                to_search.insert(PathNode { cost: alt, node: v });

                let dist_v = dist.entry(v).or_insert(usize::MAX);
                if alt <= *dist_v {
                    *dist_v = alt;
                    prev.entry(v).or_default().push((u_pos, u_dir));
                }
            }
        }
    }

    let mut total_tiles: FxHashSet<Coord> = FxHashSet::default();
    {
        let mut to_search: BTreeSet<_> = prev.keys().filter(|c| c.0 == end_pos).collect();
        while let Some(u) = to_search.pop_first() {
            total_tiles.insert(u.0);
            if prev.contains_key(u) {
                to_search.extend(prev[u].iter());
            }
        }
    }
    (min_cost_found.unwrap(), total_tiles.len())
}

fn main() -> io::Result<()> {
    let (grid, start_pos, end_pos) = parse_grid(&fs::read_to_string(
        env::args().nth(1).expect("missing cli argument"),
    )?);
    let (path_cost, total_tiles) = find_all_shortest_paths(&grid, start_pos, end_pos);
    println!("P1: Shortest path cost: {path_cost}");
    println!("P2: Number of best path tiles: {total_tiles}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex2a() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        let (grid, start_pos, end_pos) = parse_grid(input);
        let (path_cost, total_tiles) = find_all_shortest_paths(&grid, start_pos, end_pos);
        assert_eq!(path_cost, 7036);
        assert_eq!(total_tiles, 45);
    }

    #[test]
    fn ex2b() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        let (grid, start_pos, end_pos) = parse_grid(input);
        let (path_cost, total_tiles) = find_all_shortest_paths(&grid, start_pos, end_pos);
        assert_eq!(path_cost, 11048);
        assert_eq!(total_tiles, 64);
    }

    #[test]
    fn ex2c() {
        let input = "#####
#.E.#
#.#.#
#...#
##S##
#####";
        let (grid, start_pos, end_pos) = parse_grid(input);
        let (path_cost, total_tiles) = find_all_shortest_paths(&grid, start_pos, end_pos);
        assert_eq!(path_cost, 4005);
        assert_eq!(total_tiles, 9);
    }
}
