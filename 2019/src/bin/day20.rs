use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Clear,
    Wall,
    Teleport,
}

type Coord = (usize, usize);
type Grid = Vec<Vec<State>>;

struct Maze {
    map: Grid,
    teleports: HashMap<Coord, Coord>,
    start: Coord,
    end: Coord,
}

fn parse_map(input_lines: &[Vec<u8>]) -> Maze {
    let mut teleport_mappings: HashMap<[u8; 2], Vec<Coord>> = HashMap::new();
    let map = input_lines
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .map(|(x, c)| match c {
                    b' ' => State::Wall,
                    b'#' => State::Wall,
                    b'.' => State::Clear,
                    b'A'..=b'Z' => {
                        // use the letter next to the maze to build the full name of the teleport
                        let adjs_minus: Vec<_> = [
                            x.checked_sub(1)
                                .and_then(|x2| input_lines.get(y).and_then(|r| r.get(x2).copied())),
                            y.checked_sub(1)
                                .and_then(|y2| input_lines.get(y2).and_then(|r| r.get(x).copied())),
                        ]
                        .iter()
                        .flatten()
                        .copied()
                        .collect();
                        let adjs_plus: Vec<_> = [
                            input_lines.get(y).and_then(|r| r.get(x + 1).copied()),
                            input_lines.get(y + 1).and_then(|r| r.get(x).copied()),
                        ]
                        .iter()
                        .flatten()
                        .copied()
                        .collect();
                        if adjs_minus.contains(&b'.') {
                            let adj_letter =
                                adjs_plus.iter().find(|&b| b.is_ascii_uppercase()).unwrap();
                            let str_vec = [*adj_letter, *c];
                            teleport_mappings.entry(str_vec).or_default().push((x, y));
                            State::Teleport
                        } else if adjs_plus.contains(&b'.') {
                            let adj_letter =
                                adjs_minus.iter().find(|&b| b.is_ascii_uppercase()).unwrap();
                            let str_vec = [*c, *adj_letter];
                            teleport_mappings.entry(str_vec).or_default().push((x, y));
                            State::Teleport
                        } else {
                            State::Wall
                        }
                    }
                    _ => panic!("Unrecognised character {}", c),
                })
                .collect()
        })
        .collect();
    let mut teleports = HashMap::new();
    for (n, coords) in teleport_mappings.iter() {
        assert!(coords.len() == 2 || n == b"AA" || n == b"ZZ");
        if coords.len() == 2 {
            teleports.insert(coords[0], coords[1]);
            teleports.insert(coords[1], coords[0]);
        }
    }
    let aa_portal = teleport_mappings[b"AA"][0];
    let zz_portal = teleport_mappings[b"ZZ"][0];
    Maze {
        map,
        teleports,
        start: aa_portal, // note: actual start position is adjacent
        end: zz_portal,
    }
}

fn open_adjacents(maze: &Maze, pos: Coord) -> Vec<Coord> {
    let mut ret = vec![];
    if pos.1 > 0 && maze.map[pos.1 - 1].len() > pos.0 && maze.map[pos.1 - 1][pos.0] != State::Wall {
        ret.push((pos.0, pos.1 - 1));
    }
    if pos.1 < maze.map.len() - 1
        && maze.map[pos.1 + 1].len() > pos.0
        && maze.map[pos.1 + 1][pos.0] != State::Wall
    {
        ret.push((pos.0, pos.1 + 1));
    }
    if pos.0 > 0 && maze.map[pos.1].len() > pos.0 - 1 && maze.map[pos.1][pos.0 - 1] != State::Wall {
        ret.push((pos.0 - 1, pos.1));
    }
    if pos.0 < maze.map[pos.1].len() - 1
        && maze.map[pos.1].len() > pos.0 + 1
        && maze.map[pos.1][pos.0 + 1] != State::Wall
    {
        ret.push((pos.0 + 1, pos.1));
    }
    ret
}

fn maybe_teleport(maze: &Maze, pos: Coord) -> Coord {
    // move out of teleport position - teleports only have one open adjacent position
    maze.teleports
        .get(&pos)
        .map(|dest| open_adjacents(maze, *dest)[0])
        .unwrap_or(pos)
}

fn flood_fill(maze: &Maze) -> usize {
    let mut to_search: Vec<_> = open_adjacents(maze, maze.start);
    let mut searched: HashSet<Coord> = HashSet::new();
    searched.insert(maze.start);
    let mut iteration_count = 0;
    while !to_search.is_empty() {
        if to_search.contains(&maze.end) {
            break;
        }
        searched.extend(to_search.clone());
        let next_to_search: Vec<_> = to_search
            .iter()
            .flat_map(|&c| {
                open_adjacents(maze, c)
                    .iter()
                    .map(|&c| maybe_teleport(maze, c))
                    .filter(|d| !searched.contains(d))
                    .collect::<Vec<_>>()
            })
            .collect();
        to_search = next_to_search;
        iteration_count += 1;
    }
    iteration_count - 1 // first step doesn't count (should start at position adjacent)
}

fn main() {
    let input_lines: Vec<_> = BufReader::new(
        File::open(
            env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .map(|l| l.unwrap().as_bytes().to_vec())
    .collect();

    let maze = parse_map(&input_lines);

    let num_steps = flood_fill(&maze);
    println!(
        "Number of steps needed to reach the end goal: {}",
        num_steps
    );
}
