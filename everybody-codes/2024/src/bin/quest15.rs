use fixedbitset::FixedBitSet;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use std::fs;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Type {
    Space,
    Herb(char),
}

fn parse_map(input: &str) -> (FxHashMap<Coord, Type>, Coord) {
    let mut map = FxHashMap::default();
    let mut start_pos = Coord { x: 0, y: 0 };
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = Coord { x, y };
            if y == 0 && c == '.' {
                // Entry point is single point at top edge
                start_pos = coord;
            }
            match c {
                '.' => _ = map.insert(coord, Type::Space),
                'A'..='Z' => _ = map.insert(coord, Type::Herb(c)),
                '#' | '~' => (),
                _ => unreachable!(),
            };
        }
    }
    (map, start_pos)
}

fn get_neighbour_coords(pos: Coord) -> [Option<Coord>; 4] {
    [
        pos.x.checked_sub(1).map(|x| Coord { x, y: pos.y }),
        pos.y.checked_sub(1).map(|y| Coord { x: pos.x, y }),
        Some(Coord {
            x: pos.x + 1,
            y: pos.y,
        }),
        Some(Coord {
            x: pos.x,
            y: pos.y + 1,
        }),
    ]
}

fn get_herb_distances(
    map: &FxHashMap<Coord, Type>,
    start: Coord,
    destinations: &[Coord],
) -> FxHashMap<(Coord, Coord), usize> {
    let mut to_search = VecDeque::from([(start, 0)]);
    let mut seen = FxHashSet::default();
    seen.insert(start);

    let mut pair_distances = FxHashMap::default();
    while let Some((node, node_distance)) = to_search.pop_front() {
        if destinations.contains(&node) {
            pair_distances.insert((start, node), node_distance);
            // done.
            if pair_distances.len() == destinations.len() {
                break;
            }
        }

        for neighbour in get_neighbour_coords(node)
            .into_iter()
            .flatten()
            .filter(|c| map.contains_key(c))
        {
            if !seen.contains(&neighbour) {
                to_search.push_back((neighbour, node_distance + 1));
                seen.insert(neighbour);
            }
        }
    }
    pair_distances
}

fn herb_tsp(
    herb_vertices: &FxHashMap<(Coord, Coord), usize>,
    herb_destinations: &FxHashMap<Coord, usize>,
    visited_herbs: &FixedBitSet,
    start_pos: Coord,
    position: Coord,
) -> usize {
    if visited_herbs.is_full() {
        // note, backwards as we never bothered calculating the opposite direction
        return herb_vertices[&(start_pos, position)];
    }
    let mut min = usize::MAX;
    for (herb_coord, herb_num) in herb_destinations {
        if visited_herbs[*herb_num] {
            continue;
        }
        let this_len = herb_vertices[&(position, *herb_coord)];
        let mut new_visited_herbs = visited_herbs.clone();
        new_visited_herbs.insert(*herb_num);
        min = usize::min(
            min,
            this_len
                + herb_tsp(
                    herb_vertices,
                    herb_destinations,
                    &new_visited_herbs,
                    start_pos,
                    *herb_coord,
                ),
        );
    }
    min
}

fn get_herb_round_trip_len(input: &str) -> usize {
    let (map, start_pos) = parse_map(input);
    let herbs: FxHashMap<Coord, Type> = map
        .iter()
        .filter(|(_, t)| matches!(t, Type::Herb(..)))
        .map(|(c, t)| (*c, *t))
        .collect();
    let mut herb_vertices =
        get_herb_distances(&map, start_pos, &herbs.keys().copied().collect::<Vec<_>>());
    for (herb_coord, herb_type) in &herbs {
        let valid_herb_destinations: Vec<_> = herbs
            .iter()
            .filter(|(_, t)| **t != *herb_type)
            .map(|(c, _)| *c)
            .collect();
        herb_vertices.extend(get_herb_distances(
            &map,
            *herb_coord,
            &valid_herb_destinations,
        ));
    }
    let herb_num_map: FxHashMap<Type, usize> = herbs
        .values()
        .unique()
        .enumerate()
        .map(|(i, c)| (*c, i))
        .collect();
    let herbs_by_idx = herbs.iter().map(|(c, t)| (*c, herb_num_map[t])).collect();

    let visited_herbs = FixedBitSet::with_capacity(herb_num_map.len());
    herb_tsp(
        &herb_vertices,
        &herbs_by_idx,
        &visited_herbs,
        start_pos,
        start_pos,
    )
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_path_len = get_herb_round_trip_len(&fs::read_to_string(p1_input_filename)?);
    println!("P1: Herb round trip length: {p1_path_len}");

    let p2_path_len = get_herb_round_trip_len(&fs::read_to_string(p2_input_filename)?);
    println!("P2: Herb round trip length: {p2_path_len}");

    let p3_path_len = get_herb_round_trip_len(&fs::read_to_string(p3_input_filename)?);
    println!("P3: Herb round trip length: {p3_path_len}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input = "#####.#####
#.........#
#.######.##
#.........#
###.#.#####
#H.......H#
###########";

        let min_path = get_herb_round_trip_len(input);
        assert_eq!(min_path, 26);
    }

    #[test]
    fn ex2() {
        let input = "##########.##########
#...................#
#.###.##.###.##.#.#.#
#..A#.#..~~~....#A#.#
#.#...#.~~~~~...#.#.#
#.#.#.#.~~~~~.#.#.#.#
#...#.#.B~~~B.#.#...#
#...#....BBB..#....##
#C............#....C#
#####################";
        let min_path = get_herb_round_trip_len(input);
        assert_eq!(min_path, 38);
    }
}
