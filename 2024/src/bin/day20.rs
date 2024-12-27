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

fn coord_neighbours(max_bound: Coord, c: Coord) -> impl Iterator<Item = Coord> {
    [
        c.y.checked_sub(1).map(|y| Coord { x: c.x, y }),
        (c.y < max_bound.y).then_some(Coord { x: c.x, y: c.y + 1 }),
        c.x.checked_sub(1).map(|x| Coord { x, y: c.y }),
        (c.x < max_bound.x).then_some(Coord { x: c.x + 1, y: c.y }),
    ]
    .into_iter()
    .flatten()
}

fn flood_fill(
    max_bound: Coord,
    walls: &FxHashSet<Coord>,
    start_pos: Coord,
    find_walls: bool,
    distance_limit: Option<usize>,
) -> FxHashMap<Coord, usize> {
    let mut searched: FxHashMap<Coord, usize> = FxHashMap::default();
    let mut to_search = BTreeSet::from([(0, start_pos)]);
    while let Some((cost, u)) = to_search.pop_first() {
        if distance_limit.is_some_and(|dl| cost >= dl) {
            break;
        }
        searched.insert(u, cost);
        for v in coord_neighbours(max_bound, u)
            .filter(|v| walls.contains(v) == find_walls && !searched.contains_key(v))
        {
            to_search.insert((cost + 1, v));
        }
    }
    searched
}

fn get_all_path_differences(
    max_bound: Coord,
    walls: &FxHashSet<Coord>,
    start_pos: Coord,
    end_pos: Coord,
    max_cheat_len: usize,
) -> FxHashMap<isize, usize> {
    let distances_from_start = flood_fill(max_bound, walls, start_pos, false, None);
    let distances_from_end = flood_fill(max_bound, walls, end_pos, false, None);

    let mut all_possible_wall_skips: FxHashSet<(Coord, Coord, usize)> = FxHashSet::default();
    for cheat_start in distances_from_start.keys() {
        // find reachable walls within max_cheat_len limit
        // TODO cheats don't *have* to use walls. It's just a general noclip
        let possible_exit_distances =
            flood_fill(max_bound, walls, *cheat_start, true, Some(max_cheat_len));
        let possible_exits: Vec<_> = possible_exit_distances
            .iter()
            .filter(|(c, _)| *c != cheat_start)
            .flat_map(|(c, dist)| {
                coord_neighbours(max_bound, *c)
                    .filter(|c| !walls.contains(c))
                    .map(move |exit| (exit, dist + 1))
            })
            .collect();
        for (possible_exit, dist) in possible_exits {
            all_possible_wall_skips.insert((*cheat_start, possible_exit, dist));
        }
    }

    let length_no_cheat = distances_from_start[&end_pos];

    let mut route_diff_count: FxHashMap<isize, usize> = FxHashMap::default();
    for (wall_skip_start, wall_skip_end, skip_dist) in all_possible_wall_skips {
        let route_len =
            distances_from_start[&wall_skip_start] + skip_dist + distances_from_end[&wall_skip_end];
        let route_diff = route_len as isize - length_no_cheat as isize;
        if route_diff == -74 {
            println!("{:?} -> {:?}", wall_skip_start, wall_skip_end);
        }
        *route_diff_count.entry(route_diff).or_default() += 1;
    }
    route_diff_count
}

fn main() -> io::Result<()> {
    let (walls, start_pos, end_pos) = parse_grid(&fs::read_to_string(
        env::args().nth(1).expect("missing cli argument"),
    )?);

    let max_x = walls.iter().max_by_key(|c| c.x).unwrap().x;
    let max_y = walls.iter().max_by_key(|c| c.y).unwrap().y;
    let max_bound = Coord { x: max_x, y: max_y };

    let p1_path_diffs = get_all_path_differences(max_bound, &walls, start_pos, end_pos, 2);
    let count_100ps_saving: usize = p1_path_diffs
        .iter()
        .filter(|(&k, _)| k <= -100)
        .map(|(_, n)| n)
        .sum();
    println!("P1: Number of (length 2) cheats saving at least 100ps: {count_100ps_saving}");
    let p2_path_diffs = get_all_path_differences(max_bound, &walls, start_pos, end_pos, 20);
    let count_100ps_saving: usize = p2_path_diffs
        .iter()
        .filter(|(&k, _)| k <= -100)
        .map(|(_, n)| n)
        .sum();
    println!("P2: Number of (length 20) cheats saving at least 100ps: {count_100ps_saving}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        let (walls, start_pos, end_pos) = parse_grid(input);
        let max_x = walls.iter().max_by_key(|c| c.x).unwrap().x;
        let max_y = walls.iter().max_by_key(|c| c.y).unwrap().y;
        let max_bound = Coord { x: max_x, y: max_y };
        let p1_path_diffs = get_all_path_differences(max_bound, &walls, start_pos, end_pos, 2);
        let saving: usize = p1_path_diffs
            .iter()
            .filter(|(&k, _)| k <= -20)
            .map(|(_, n)| n)
            .sum();
        assert_eq!(saving, 5);
    }

    #[test]
    fn ex2() {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        let (walls, start_pos, end_pos) = parse_grid(input);
        let max_x = walls.iter().max_by_key(|c| c.x).unwrap().x;
        let max_y = walls.iter().max_by_key(|c| c.y).unwrap().y;
        let max_bound = Coord { x: max_x, y: max_y };
        let p2_path_diffs = get_all_path_differences(max_bound, &walls, start_pos, end_pos, 20);
        for (k, v) in &p2_path_diffs {
            if *k <= -74 {
                println!("{k}: {v}");
            }
        }
        let saving: usize = p2_path_diffs
            .iter()
            .filter(|(&k, _)| k <= -74)
            .map(|(_, n)| n)
            .sum();
        assert_eq!(saving, 32);
    }
}
