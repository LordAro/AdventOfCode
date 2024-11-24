use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
    z: isize,
}

fn parse_instructions(input: &str) -> Vec<Vec<(u8, isize)>> {
    input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|instr| (instr.as_bytes()[0], instr[1..].parse::<isize>().unwrap()))
                .collect()
        })
        .collect()
}

fn grow_branch(instrs: &[(u8, isize)]) -> Vec<Coord> {
    instrs
        .iter()
        .scan(Coord { x: 0, y: 0, z: 0 }, |pos, instr| {
            let new_positions: Vec<_> = (1..=instr.1)
                .map(|n| match instr.0 {
                    b'U' => Coord {
                        x: pos.x,
                        y: pos.y + n,
                        z: pos.z,
                    },
                    b'D' => Coord {
                        x: pos.x,
                        y: pos.y - n,
                        z: pos.z,
                    },
                    b'R' => Coord {
                        x: pos.x + n,
                        y: pos.y,
                        z: pos.z,
                    },
                    b'L' => Coord {
                        x: pos.x - n,
                        y: pos.y,
                        z: pos.z,
                    },
                    b'F' => Coord {
                        x: pos.x,
                        y: pos.y,
                        z: pos.z + n,
                    },
                    b'B' => Coord {
                        x: pos.x,
                        y: pos.y,
                        z: pos.z - n,
                    },
                    _ => unreachable!(),
                })
                .collect();
            *pos = *new_positions.last().unwrap();
            Some(new_positions)
        })
        .flatten()
        .collect()
}

fn grow_tree(all_instrs: &[Vec<(u8, isize)>]) -> (HashSet<Coord>, HashSet<Coord>) {
    let mut segments = HashSet::new();
    let mut leaves = HashSet::new();
    for branch_instr in all_instrs {
        let branch_coords = grow_branch(branch_instr);
        segments.extend(&branch_coords);
        leaves.insert(*branch_coords.last().unwrap());
    }
    (segments, leaves)
}

fn get_min_sap_murkiness(branch_instrs: &[Vec<(u8, isize)>]) -> usize {
    let (tree_segments, leaf_segments) = grow_tree(branch_instrs);

    let num_trunk_segments = tree_segments
        .iter()
        .filter(|n| n.x == 0 && n.z == 0)
        .count();

    let mut trunk_distances = HashMap::new();
    // find all trunk distances for each leaf at once, rather than repeating for each trunk/leaf combination
    for leaf in leaf_segments {
        let mut num_inserts = 0;
        let mut to_search = VecDeque::from([(leaf, 0)]);
        let mut seen = HashSet::from([leaf]);
        while let Some((node, node_distance)) = to_search.pop_front() {
            if node.x == 0 && node.z == 0 {
                *trunk_distances.entry(node).or_default() += node_distance;
                num_inserts += 1;
                if num_inserts == num_trunk_segments {
                    // done, no need to search any further
                    break;
                }
            }

            let neighbours = [
                Coord {
                    x: node.x + 1,
                    y: node.y,
                    z: node.z,
                },
                Coord {
                    x: node.x - 1,
                    y: node.y,
                    z: node.z,
                },
                Coord {
                    x: node.x,
                    y: node.y + 1,
                    z: node.z,
                },
                Coord {
                    x: node.x,
                    y: node.y - 1,
                    z: node.z,
                },
                Coord {
                    x: node.x,
                    y: node.y,
                    z: node.z + 1,
                },
                Coord {
                    x: node.x,
                    y: node.y,
                    z: node.z - 1,
                },
            ];
            for neighbour in neighbours {
                if tree_segments.contains(&neighbour) && !seen.contains(&neighbour) {
                    to_search.push_back((neighbour, node_distance + 1));
                    seen.insert(neighbour);
                }
            }
        }
    }
    *trunk_distances.values().min().unwrap()
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_instrs = &parse_instructions(&fs::read_to_string(p1_input_filename)?);
    let (p1_tree_segments, _) = grow_tree(p1_instrs);
    let p1_max_height = p1_tree_segments.iter().max_by_key(|c| c.y).unwrap().y;

    println!("P1: Plant final height: {p1_max_height}");

    let p2_branch_instrs = &parse_instructions(&fs::read_to_string(p2_input_filename)?);
    let (p2_tree_segments, _) = grow_tree(p2_branch_instrs);
    println!("P2: Total unique segments: {}", p2_tree_segments.len());

    let p3_branch_instrs = parse_instructions(&fs::read_to_string(p3_input_filename)?);
    let p3_min_murkiness_level: usize = get_min_sap_murkiness(&p3_branch_instrs);
    println!("P3: Minimum sap murkiness: {p3_min_murkiness_level}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let instrs = &parse_instructions("U5,R3,D2,L5,U4,R5,D2")[0];
        let tree_idxs = grow_branch(&instrs);
        let max_height = tree_idxs.iter().max_by_key(|c| c.y).unwrap().y;
        assert_eq!(max_height, 7);
    }

    #[test]
    fn ex2() {
        let input = "U5,R3,D2,L5,U4,R5,D2
U6,L1,D2,R3,U2,L1";
        let branch_instrs = parse_instructions(input);
        let (tree_segments, _) = grow_tree(&branch_instrs);
        assert_eq!(tree_segments.len(), 32);
    }

    #[test]
    fn ex3() {
        let input = "U5,R3,D2,L5,U4,R5,D2
U6,L1,D2,R3,U2,L1";
        let branch_instrs = parse_instructions(input);
        let min_murkiness_level: usize = get_min_sap_murkiness(&branch_instrs);
        assert_eq!(min_murkiness_level, 5);
    }

    #[test]
    fn ex3b() {
        let input = "U20,L1,B1,L2,B1,R2,L1,F1,U1
U10,F1,B1,R1,L1,B1,L1,F1,R2,U1
U30,L2,F1,R1,B1,R1,F2,U1,F1
U25,R1,L2,B1,U1,R2,F1,L2
U16,L1,B1,L1,B3,L1,B1,F1";
        let branch_instrs = parse_instructions(input);
        let min_murkiness_level: usize = get_min_sap_murkiness(&branch_instrs);
        assert_eq!(min_murkiness_level, 46);
    }
}
