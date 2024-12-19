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

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct CoordDir {
    pos: Coord,
    dir: Dir,
}

// Used for ordered btreeset/binaryheap purposes
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct PathNode {
    cost: usize,
    node: CoordDir,
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

fn get_next_coord(cd: CoordDir) -> Coord {
    match cd.dir {
        Dir::Up => Coord {
            x: cd.pos.x,
            y: cd.pos.y - 1,
        },
        Dir::Down => Coord {
            x: cd.pos.x,
            y: cd.pos.y + 1,
        },
        Dir::Left => Coord {
            x: cd.pos.x - 1,
            y: cd.pos.y,
        },
        Dir::Right => Coord {
            x: cd.pos.x + 1,
            y: cd.pos.y,
        },
    }
}

fn find_all_shortest_paths(
    wall_map: &FxHashSet<Coord>,
    start_pos: Coord,
    end_pos: Coord,
) -> (usize, usize) {
    let mut dist: FxHashMap<CoordDir, usize> = FxHashMap::default();
    dist.insert(
        CoordDir {
            pos: start_pos,
            dir: Dir::Right,
        },
        0,
    );
    let mut prev: FxHashMap<CoordDir, Vec<CoordDir>> = FxHashMap::default();

    let mut min_cost_found = None;
    // Q, ish
    let mut to_search = BTreeSet::from([PathNode {
        cost: 0,
        node: CoordDir {
            pos: start_pos,
            dir: Dir::Right,
        },
    }]);
    let mut searched = FxHashSet::default();
    while let Some(PathNode { cost, node: u }) = to_search.pop_first() {
        if u.pos == end_pos {
            // Trying to start *from* the end position,
            // so must have found all best routes *to* it already
            min_cost_found = Some(cost);
            break;
        }

        searched.insert(u);

        // Build neighbour edges.
        let next_forward = Some(CoordDir {
            pos: get_next_coord(u),
            dir: u.dir,
        });
        // Don't turn if it's pointless
        let rot_left = Some(CoordDir {
            pos: u.pos,
            dir: u.dir.turn_left(),
        })
        .filter(|cd| !wall_map.contains(&get_next_coord(*cd)));
        let rot_right = Some(CoordDir {
            pos: u.pos,
            dir: u.dir.turn_right(),
        })
        .filter(|cd| !wall_map.contains(&get_next_coord(*cd)));

        for v in [next_forward, rot_left, rot_right].into_iter().flatten() {
            if !wall_map.contains(&v.pos) && !searched.contains(&v) {
                let alt = dist[&u] + if v.dir == u.dir { 1 } else { 1000 };
                to_search.insert(PathNode { cost: alt, node: v });

                let dist_v = dist.entry(v).or_insert(usize::MAX);
                if alt <= *dist_v {
                    *dist_v = alt;
                    prev.entry(v).or_default().push(u);
                }
            }
        }
    }

    // BFS search of prev tree to find all best route tiles
    let mut total_tiles: FxHashSet<Coord> = FxHashSet::default();
    {
        let mut to_search: BTreeSet<_> = prev.keys().filter(|c| c.pos == end_pos).collect();
        while let Some(u) = to_search.pop_first() {
            total_tiles.insert(u.pos);
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
