use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::io;

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Clear,
    Wall,
    Teleport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

type Grid = Vec<Vec<State>>;

struct Maze {
    map: Grid,
    teleports: HashMap<Coord, Coord>,
    outer_teleports: HashSet<Coord>,
    start: Coord,
    end: Coord,
}

fn adjacent_coords(limit: Coord, p: Coord) -> Vec<Coord> {
    [
        Coord {
            x: p.x.wrapping_sub(1),
            y: p.y,
        },
        Coord {
            x: p.x,
            y: p.y.wrapping_sub(1),
        },
        Coord { x: p.x + 1, y: p.y },
        Coord { x: p.x, y: p.y + 1 },
    ]
    .iter()
    .filter(move |&x| x.x < limit.x && x.y < limit.y)
    .copied()
    .collect()
}

fn parse_map(input_str: &str) -> Maze {
    let mut teleport_mappings: HashMap<[u8; 2], Vec<Coord>> = HashMap::new();
    let mut outer_teleports: HashSet<Coord> = HashSet::new();
    let input_lines: Vec<_> = input_str.lines().map(|l| l.as_bytes()).collect();
    let limit = Coord {
        x: input_lines[0].len(),
        y: input_lines.len(),
    };
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
                        let coord = Coord { x, y };
                        let adj_coords = adjacent_coords(limit, coord);
                        if adj_coords.iter().any(|c| input_lines[c.y][c.x] == b'.') {
                            let adj_letter_coord = adj_coords
                                .iter()
                                .find(|c| input_lines[c.y][c.x].is_ascii_uppercase())
                                .unwrap();
                            let adj_letter = input_lines[adj_letter_coord.y][adj_letter_coord.x];

                            let str_vec = if adj_letter_coord.x < x || adj_letter_coord.y < y {
                                [adj_letter, *c]
                            } else {
                                [*c, adj_letter]
                            };

                            let is_outer = adj_letter_coord.x == 0
                                || adj_letter_coord.x == l.len() - 1
                                || adj_letter_coord.y == 0
                                || adj_letter_coord.y == input_lines.len() - 1;
                            teleport_mappings.entry(str_vec).or_default().push(coord);
                            if is_outer {
                                outer_teleports.insert(coord);
                            }
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
    let aa_portal = teleport_mappings[b"AA"][0];
    let zz_portal = teleport_mappings[b"ZZ"][0];
    let mut teleports = HashMap::new();
    for (n, coords) in teleport_mappings.iter() {
        assert!(coords.len() == 2 || n == b"AA" || n == b"ZZ");
        if coords.len() == 2 {
            teleports.insert(coords[0], coords[1]);
            teleports.insert(coords[1], coords[0]);
        }
    }
    Maze {
        map,
        teleports,
        outer_teleports,
        start: aa_portal, // note: actual start position is adjacent
        end: zz_portal,
    }
}

fn open_adjacents(map: &Grid, pos: Coord) -> Vec<Coord> {
    let limit = Coord {
        x: map[0].len(),
        y: map.len(),
    };
    adjacent_coords(limit, pos)
        .into_iter()
        .filter(|c| map[c.y][c.x] != State::Wall)
        .collect()
}

fn maybe_teleport(maze: &Maze, pos: Coord) -> Coord {
    // move out of teleport position - teleports only have one open adjacent position
    maze.teleports
        .get(&pos)
        .map(|&dest| open_adjacents(&maze.map, dest)[0])
        .unwrap_or(pos)
}

fn flood_fill(maze: &Maze) -> usize {
    let mut to_search: Vec<_> = open_adjacents(&maze.map, maze.start);
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
                open_adjacents(&maze.map, c)
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

fn flood_fill_with_depth(maze: &Maze) -> usize {
    let mut to_search: Vec<(Coord, usize)> = open_adjacents(&maze.map, maze.start)
        .iter()
        .map(|c| (*c, 0))
        .collect();
    let mut searched: HashSet<(Coord, usize)> = HashSet::new();
    searched.insert((maze.start, 0));

    let mut iteration_count = 0;
    while !to_search.is_empty() {
        if to_search.iter().any(|&(c, n)| c == maze.end && n == 0) {
            // first step doesn't count (should start at position adjacent)
            return iteration_count - 1;
        }
        searched.extend(to_search.clone());
        let next_to_search: Vec<_> = to_search
            .iter()
            .flat_map(|&(c, n)| {
                open_adjacents(&maze.map, c)
                    .iter()
                    .filter_map(|&c| {
                        if maze.teleports.contains_key(&c) {
                            if n == 0 && maze.outer_teleports.contains(&c) {
                                None
                            } else {
                                let new_n = if maze.outer_teleports.contains(&c) {
                                    n - 1
                                } else {
                                    n + 1
                                };
                                Some((maybe_teleport(maze, c), new_n))
                            }
                        } else {
                            Some((c, n))
                        }
                    })
                    .filter(|coord_depth| !searched.contains(coord_depth))
                    .collect::<Vec<_>>()
            })
            .collect();
        to_search = next_to_search;
        iteration_count += 1;
    }
    unreachable!("Run out of places to search!");
}

fn main() -> io::Result<()> {
    let input_str: String =
        fs::read_to_string(env::args().nth(1).expect("Incorrect number of arguments"))?;

    let maze = parse_map(&input_str);

    let num_steps = flood_fill(&maze);
    println!(
        "Number of steps needed to reach the end goal: {}",
        num_steps
    );

    let num_recursive_steps = flood_fill_with_depth(&maze);
    println!(
        "Number of steps needed to reach the end goal with recursion: {}",
        num_recursive_steps
    );
    Ok(())
}
