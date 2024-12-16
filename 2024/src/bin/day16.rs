use rustc_hash::FxHashSet;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
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

#[derive(Debug, PartialEq, Eq, Clone)]
struct PathObj {
    cost: usize,
    dir: Dir,
    nodes: Vec<Coord>,
}

impl Ord for PathObj {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost) // reverse order for min-heap
            .then(other.nodes.cmp(&self.nodes))
    }
}

impl PartialOrd for PathObj {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
) -> Vec<PathObj> {
    let mut to_search = BinaryHeap::new();
    to_search.push(PathObj {
        cost: 0,
        dir: Dir::Right,
        nodes: vec![start_pos],
    });

    let mut min_path_cost = None;
    let mut min_paths = vec![];

    //let mut m = 1;

    while let Some(pn) = to_search.pop() {
        //println!("{pn:?}");
        //if pn.cost > m * 1000 {
        //    println!(
        //        "current cost: {}, remaining paths to search: {}\n{:?}",
        //        pn.cost,
        //        to_search.len(),
        //        pn.nodes
        //    );
        //    m += 1;
        //}
        if min_path_cost.is_some_and(|mpc| pn.cost > mpc) {
            // no further possible results
            break;
        }
        let cur_pos = pn.nodes.last().unwrap();
        if *cur_pos == end_pos {
            min_path_cost = Some(pn.cost);
            min_paths.push(pn);
            continue;
        }

        // this iterator will get slow with long lists,
        // but we're not expecting paths to get all that long
        // reversed as duplicate coords are more likely to be near where are currently
        let next_coord = get_next_coord(*cur_pos, pn.dir);
        if !pn.nodes.iter().rev().any(|n| *n == next_coord) && !wall_map.contains(&next_coord) {
            let mut new_nodes = pn.nodes.clone();
            new_nodes.push(next_coord);
            to_search.push(PathObj {
                cost: pn.cost + 1,
                dir: pn.dir,
                nodes: new_nodes,
            });
        }

        // no point turning such that we're facing a wall
        let left = pn.dir.turn_left();
        let left_next = get_next_coord(*cur_pos, left);
        if !pn.nodes.iter().rev().any(|n| *n == left_next) && !wall_map.contains(&left_next) {
            let mut new_nodes = pn.nodes.clone();
            new_nodes.push(left_next);
            to_search.push(PathObj {
                cost: pn.cost + 1001,
                dir: left,
                nodes: new_nodes,
            });
        }
        let right = pn.dir.turn_right();
        let right_next = get_next_coord(*cur_pos, right);
        if !pn.nodes.iter().rev().any(|n| *n == right_next) && !wall_map.contains(&right_next) {
            let mut new_nodes = pn.nodes.clone();
            new_nodes.push(right_next);
            to_search.push(PathObj {
                cost: pn.cost + 1001,
                dir: right,
                nodes: new_nodes,
            });
        }
    }
    min_paths
}

fn get_total_path_tiles(paths: &[PathObj]) -> usize {
    let all_tiles: FxHashSet<_> = paths.iter().flat_map(|path| path.nodes.iter()).collect();
    all_tiles.len()
}

fn main() -> io::Result<()> {
    let (grid, start_pos, end_pos) = parse_grid(&fs::read_to_string(
        env::args().nth(1).expect("missing cli argument"),
    )?);
    let all_paths = find_all_shortest_paths(&grid, start_pos, end_pos);
    println!("P1: Shortest path cost: {}", all_paths[0].cost);
    println!(
        "P2: Number of best path tiles: {}",
        get_total_path_tiles(&all_paths)
    );
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
        let all_paths = find_all_shortest_paths(&grid, start_pos, end_pos);
        assert_eq!(all_paths[0].cost, 7036);
        assert_eq!(get_total_path_tiles(&all_paths), 45);
        //assert_eq!(1, 0);
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
        let all_paths = find_all_shortest_paths(&grid, start_pos, end_pos);
        assert_eq!(all_paths[0].cost, 11048);
        assert_eq!(get_total_path_tiles(&all_paths), 64);
    }
}
