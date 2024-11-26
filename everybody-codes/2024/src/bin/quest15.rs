use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Type {
    Space,
    Herb(char),
}

fn parse_map(input: &str) -> (HashMap<Coord, Type>, Coord) {
    let mut map = HashMap::new();
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

fn get_herb_path_len(map: &HashMap<Coord, Type>, start: Coord, end: Coord) -> usize {
    let mut to_search = VecDeque::from([(start, 0)]);
    let mut seen = HashSet::from([start]);
    while let Some((node, node_distance)) = to_search.pop_front() {
        if node == end {
            return node_distance;
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
    unreachable!();
}

// Nice.
fn get_herb_combos(map: &HashMap<Coord, Type>, start_pos: Coord) -> Vec<(Coord, Coord)> {
    [start_pos]
        .into_iter()
        .chain(
            map.iter()
                .filter(|(_, t)| matches!(t, Type::Herb(..)))
                .map(|(c, _)| *c),
        )
        .combinations(2)
        .filter(|combo| map[&combo[0]] != map[&combo[1]])
        .map(|combo| (combo[0], combo[1]))
        .collect()
}

fn get_herb_graph_vertices(
    map: &HashMap<Coord, Type>,
    pairs: &[(Coord, Coord)],
) -> HashMap<(Coord, Coord), usize> {
    pairs
        .iter()
        .flat_map(|pair| {
            let len = get_herb_path_len(map, pair.0, pair.1);
            [((pair.0, pair.1), len), ((pair.1, pair.0), len)]
        })
        .collect()
}

fn herb_tsp(
    herb_vertices: &HashMap<(Coord, Coord), usize>,
    start_pos: Coord,
    position: Coord,
    remaining_destinations: &HashMap<Coord, Type>,
) -> usize {
    if remaining_destinations.is_empty() {
        return herb_vertices[&(position, start_pos)];
    }
    let mut min = usize::MAX;
    for (coord, type_) in remaining_destinations {
        let this_len = herb_vertices[&(position, *coord)];
        let new_remaining_destinations = remaining_destinations
            .iter()
            .filter(|(_, t)| *t != type_)
            .map(|(c, t)| (*c, *t))
            .collect();
        min = usize::min(
            min,
            this_len
                + herb_tsp(
                    herb_vertices,
                    start_pos,
                    *coord,
                    &new_remaining_destinations,
                ),
        );
    }
    min
}

fn get_herb_round_trip_len(input: &str) -> usize {
    let (map, start_pos) = parse_map(input);
    let herbs: HashMap<Coord, Type> = map
        .iter()
        .filter(|(_, t)| matches!(t, Type::Herb(..)))
        .map(|(c, t)| (*c, *t))
        .collect();
    let herb_combos = get_herb_combos(&map, start_pos);
    let herb_vertices = get_herb_graph_vertices(&map, &herb_combos);

    herb_tsp(&herb_vertices, start_pos, start_pos, &herbs)
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
