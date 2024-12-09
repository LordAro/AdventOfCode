use itertools::Itertools;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Coord {
    x: i64,
    y: i64,
}

fn parse_input(input: &str) -> Vec<((char, usize), usize)> {
    input
        .lines()
        .map(|l| {
            let words: Vec<_> = l.split(' ').collect();
            (
                (words[0].chars().next().unwrap(), words[1].parse().unwrap()),
                usize::from_str_radix(&words[2][2..words[2].len() - 1], 16).unwrap(),
            )
        })
        .collect()
}

fn get_edges(instrs: &[(char, usize)]) -> HashSet<Coord> {
    let mut pos = Coord { x: 0, y: 0 };
    let mut lagoon_edges: HashSet<Coord> = HashSet::default();
    lagoon_edges.insert(pos);
    for (instr_dir, instr_count) in instrs {
        for _ in 0..*instr_count {
            pos = match instr_dir {
                'U' => Coord {
                    x: pos.x,
                    y: pos.y - 1,
                },
                'R' => Coord {
                    x: pos.x + 1,
                    y: pos.y,
                },
                'L' => Coord {
                    x: pos.x - 1,
                    y: pos.y,
                },
                'D' => Coord {
                    x: pos.x,
                    y: pos.y + 1,
                },
                _ => unreachable!(),
            };
            lagoon_edges.insert(pos);
        }
    }
    lagoon_edges
}

fn ray_cast_count(edges: &HashSet<Coord>, min_coord: Coord, max_coord: Coord) -> usize {
    let mut inner_count = 0;
    for y in min_coord.y + 1..max_coord.y {
        let mut crossings = 0;
        for (is_edge, coords) in &(min_coord.x..=max_coord.x)
            .map(|x| Coord { x, y })
            .chunk_by(|c| edges.contains(&c))
        {
            if is_edge {
                // points don't count as a crossing, so check that above and below has at least 1
                // edge as well
                let edge_coords: Vec<_> = coords.collect();
                if edge_coords.len() == 1
                    || (edge_coords
                        .iter()
                        .map(|c| Coord { x: c.x, y: c.y - 1 })
                        .any(|c| edges.contains(&c))
                        && edge_coords
                            .iter()
                            .map(|c| Coord { x: c.x, y: c.y + 1 })
                            .any(|c| edges.contains(&c)))
                {
                    crossings += 1;
                }
            } else if crossings % 2 == 1 {
                inner_count += coords.count();
            }
        }
    }
    inner_count
}

fn get_lagoon_size(instrs: &[(char, usize)]) -> usize {
    println!("begin");
    let lagoon_edges = get_edges(instrs);

    println!("edges: {}", lagoon_edges.len());
    let minmax_x = lagoon_edges
        .iter()
        .minmax_by_key(|c| c.x)
        .into_option()
        .unwrap();
    let minmax_y = lagoon_edges
        .iter()
        .minmax_by_key(|c| c.y)
        .into_option()
        .unwrap();
    let min_coord = Coord {
        x: minmax_x.0.x,
        y: minmax_y.0.y,
    };
    let max_coord = Coord {
        x: minmax_x.1.x,
        y: minmax_y.1.y,
    };

    println!("minmax: {:?} {:?}", min_coord, max_coord);
    let lagoon_inner_count = ray_cast_count(&lagoon_edges, min_coord, max_coord);
    println!("raycast: {}", lagoon_inner_count);
    lagoon_edges.len() + lagoon_inner_count
}

fn fix_instructions(instrs: &[((char, usize), usize)]) -> Vec<(char, usize)> {
    instrs
        .iter()
        .map(|(_, colour)| {
            let dir = match colour & 0xf {
                0 => 'R',
                1 => 'D',
                2 => 'L',
                3 => 'U',
                _ => unreachable!(),
            };
            let count = colour >> 4 as usize;
            (dir, count)
        })
        .collect()
}

fn main() -> io::Result<()> {
    let instructions = parse_input(&fs::read_to_string(
        env::args().nth(1).expect("Incorrect number of arguments"),
    )?);

    println!(
        "Size of lagoon: {}",
        get_lagoon_size(
            &instructions
                .iter()
                .map(|(dirlen, _)| *dirlen)
                .collect::<Vec<_>>()
        )
    );

    let corrected_instructions = fix_instructions(&instructions);
    println!(
        "True size of lagoon: {}",
        get_lagoon_size(&corrected_instructions)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EX_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn ex1() {
        let instructions: Vec<_> = parse_input(EX_INPUT)
            .iter()
            .map(|(dirlen, _)| *dirlen)
            .collect();
        assert_eq!(get_lagoon_size(&instructions), 62);
    }

    #[test]
    fn ex2() {
        let instructions = parse_input(EX_INPUT);
        let corrected_instructions = fix_instructions(&instructions);
        assert_eq!(get_lagoon_size(&corrected_instructions), 952408144115);
    }
}
