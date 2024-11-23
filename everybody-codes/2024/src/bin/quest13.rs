use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use std::io;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Chamber {
    Wall,
    Platform(u8),
}

fn parse_chamber(input: &str) -> (HashMap<Coord, Chamber>, HashSet<Coord>, HashSet<Coord>) {
    let mut starts = HashSet::new();
    let mut ends = HashSet::new();
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = Coord { x, y };
            if c == 'S' {
                starts.insert(coord);
            } else if c == 'E' {
                ends.insert(coord);
            }
            map.insert(
                coord,
                match c {
                    '#' | ' ' => Chamber::Wall,
                    'S' | 'E' => Chamber::Platform(0),
                    '0'..='9' => Chamber::Platform(c as u8 - b'0'),
                    _ => unreachable!(),
                },
            );
        }
    }
    (map, starts, ends)
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

#[derive(Debug, PartialEq, Eq)]
struct PathNode {
    cost: usize,
    coord: Coord,
    height: u8,
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost
            .cmp(&other.cost)
            .then(self.coord.cmp(&other.coord))
            .then(self.height.cmp(&other.height))
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_shortest_path(
    map: &HashMap<Coord, Chamber>,
    start_pos: Coord,
    possible_end_positions: HashSet<Coord>,
) -> usize {
    let mut to_search = BTreeSet::new();
    to_search.insert(PathNode {
        cost: 0,
        coord: start_pos,
        height: 0,
    });
    let mut searched: HashSet<Coord> = HashSet::new();

    while let Some(pn) = to_search.pop_first() {
        //println!("{pn:?}");
        if possible_end_positions.contains(&pn.coord) {
            // height always 0, no need to add it on
            return pn.cost;
        }

        searched.insert(pn.coord); // can't go back to different node
        for n in get_neighbour_coords(pn.coord)
            .into_iter()
            .flatten()
            .filter(|n| !searched.contains(n))
        {
            if let Some(Chamber::Platform(neighbour_height)) = map.get(&n) {
                // allow portals to wrap us around
                let abs_height_diff = pn.height.abs_diff(*neighbour_height);
                let wrap_height_diff = u8::max(pn.height, *neighbour_height)
                    .abs_diff(u8::min(pn.height, *neighbour_height) + 10);
                let height_diff = u8::min(abs_height_diff, wrap_height_diff) as usize;
                to_search.insert(PathNode {
                    cost: pn.cost + height_diff + 1,
                    coord: n,
                    height: *neighbour_height,
                });
            }
        }
    }
    unreachable!()
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_input = fs::read_to_string(p1_input_filename)?;
    let (p1_chamber, p1_starts, p1_ends) = parse_chamber(&p1_input);
    let p1_path_distance =
        find_shortest_path(&p1_chamber, p1_starts.into_iter().next().unwrap(), p1_ends);
    println!("P1: Shortest path: {p1_path_distance}");

    let p2_input = fs::read_to_string(p2_input_filename)?;
    let (p2_chamber, p2_starts, p2_ends) = parse_chamber(&p2_input);
    let p2_path_distance =
        find_shortest_path(&p2_chamber, p2_starts.into_iter().next().unwrap(), p2_ends);
    println!("P2: Shortest path: {p2_path_distance}");

    let p3_input = fs::read_to_string(p3_input_filename)?;
    let (p3_chamber, p3_starts, p3_ends) = parse_chamber(&p3_input);
    // invert start/end
    let p3_path_distance =
        find_shortest_path(&p3_chamber, p3_ends.into_iter().next().unwrap(), p3_starts);
    println!("P3: Shortest path: {p3_path_distance}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input_str = "#######
#6769##
S50505E
#97434#
#######";
        let (chamber, starts, ends) = parse_chamber(&input_str);
        let path_distance = find_shortest_path(&chamber, starts.into_iter().next().unwrap(), ends);
        assert_eq!(path_distance, 28);
    }
}
