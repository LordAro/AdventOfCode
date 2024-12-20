use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Clear,
    Wall,
    Key(char),
    Door(char),
    Me,
}

type Coord = (usize, usize);
type Grid = Vec<Vec<State>>;

struct Maze {
    map: Grid,
    keys: HashMap<char, Coord>,
    doors: HashMap<char, Coord>,
}

fn parse_map<T: AsRef<str>>(input_lines: &[T]) -> (Maze, Coord) {
    let mut keys: HashMap<char, Coord> = HashMap::new();
    let mut doors: HashMap<char, Coord> = HashMap::new();
    let mut me: Coord = Default::default();
    let map = input_lines
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.as_ref()
                .chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => State::Wall,
                    '.' => State::Clear,
                    '@' => {
                        me = (x, y);
                        State::Me
                    }
                    'a'..='z' => {
                        keys.insert(c, (x, y));
                        State::Key(c)
                    }
                    'A'..='Z' => {
                        doors.insert(c, (x, y));
                        State::Door(c)
                    }
                    _ => panic!("Unrecognised character {}", c),
                })
                .collect()
        })
        .collect();
    (Maze { map, keys, doors }, me)
}

fn open_adjacents(pos: Coord, map: &Grid) -> Vec<Coord> {
    let mut ret = vec![];
    if pos.1 > 0 && map[pos.1 - 1][pos.0] != State::Wall {
        ret.push((pos.0, pos.1 - 1));
    }
    if pos.1 < map.len() - 1 && map[pos.1 + 1][pos.0] != State::Wall {
        ret.push((pos.0, pos.1 + 1));
    }
    if pos.0 > 0 && map[pos.1][pos.0 - 1] != State::Wall {
        ret.push((pos.0 - 1, pos.1));
    }
    if pos.0 < map[pos.1].len() - 1 && map[pos.1][pos.0 + 1] != State::Wall {
        ret.push((pos.0 + 1, pos.1));
    }
    ret
}

fn _print_map(positions: &Grid, route: &[Coord]) {
    for (row_ix, row) in positions.iter().enumerate() {
        for (col_ix, cell) in row.iter().enumerate() {
            if route.contains(&(col_ix, row_ix)) {
                print!("*");
            } else {
                match cell {
                    State::Clear => print!("."),
                    State::Wall => print!("#"),
                    State::Me => print!("@"),
                    State::Key(k) => print!("{}", k),
                    State::Door(d) => print!("{}", d),
                }
            }
        }
        println!();
    }
}

fn get_route(
    maze: &Maze,
    unlocked_doors: &[&Coord],
    source: Coord,
    target: Coord,
) -> Option<Vec<Coord>> {
    let mut came_from: HashMap<Coord, Coord> = HashMap::new();
    let mut open_set = VecDeque::new();
    open_set.push_back(source);

    let mut g_score = HashMap::new();
    g_score.insert(source, 0);

    while !open_set.is_empty() {
        let current = open_set.pop_front().unwrap();
        if current == target {
            let mut total_path = vec![current];
            let mut current = current;
            while came_from.contains_key(&current) {
                current = came_from[&current];
                total_path.push(current);
            }
            total_path.reverse();
            return Some(total_path);
        }

        for adj in open_adjacents(current, &maze.map).into_iter() {
            if let State::Door(_) = maze.map[adj.1][adj.0] {
                if !unlocked_doors.contains(&&adj) {
                    continue;
                }
            }
            let dist = g_score[&current] + 1;
            if &dist < g_score.get(&adj).unwrap_or(&isize::MAX) {
                came_from.insert(adj, current);
                g_score.insert(adj, dist);
                open_set.push_back(adj);
            }
        }
    }
    None
}

// basic flood fill
fn find_accessible_keys(
    map: &Vec<Vec<State>>,
    unlocked_doors: &[&Coord],
    start_positions: &[Coord],
) -> Vec<char> {
    let mut to_search: Vec<_> = start_positions
        .iter()
        .flat_map(|&p| open_adjacents(p, map))
        .collect();
    let mut searched: HashSet<Coord> = HashSet::from_iter(start_positions.iter().cloned());
    let mut found_keys = vec![];
    while let Some(c) = to_search.pop() {
        match map[c.1][c.0] {
            State::Key(k) => found_keys.push(k),
            State::Door(_) => {
                if !unlocked_doors.contains(&&c) {
                    continue;
                }
            }
            _ => (),
        }
        searched.insert(c);

        to_search.extend(
            open_adjacents(c, map)
                .iter()
                .filter(|d| !searched.contains(d)),
        );
    }
    found_keys
}

fn get_possible_routes(
    maze: &Maze,
    unlocked_doors: &[&Coord],
    start_positions: &[Coord],
    destinations: &[char],
) -> Vec<(char, Vec<Coord>)> {
    let remaining_key_coords: HashSet<Coord> = destinations.iter().map(|k| maze.keys[k]).collect();

    start_positions
        .iter()
        .cartesian_product(destinations.iter())
        .flat_map(|(&start, &dest)| {
            get_route(maze, unlocked_doors, start, maze.keys[&dest]).map(|route| (dest, route))
        })
        .filter(|(_, r)| {
            // if the route contains another key (that we haven't collected yet),
            // we can't be doing the optimal route
            r[1..r.len() - 1]
                .iter()
                .all(|c| !remaining_key_coords.contains(c))
        })
        .collect()
}

// get all accessible keys
// find routes from keys to their matching door (if possible) & keys
// do tsp on graph
// repeat

type CacheType = HashMap<(Vec<Coord>, Vec<char>, BTreeSet<char>), Vec<Coord>>;

fn get_shortest_route(
    cache: &mut CacheType,
    maze: &Maze,
    collected_keys: &BTreeSet<char>,
    start_positions: &[Coord],
) -> Vec<Coord> {
    // collected everything, we're done
    if collected_keys.len() == maze.keys.len() {
        return vec![];
    }
    let unlocked_doors: Vec<_> = collected_keys
        .iter()
        .filter(|&k| maze.doors.contains_key(&k.to_ascii_uppercase()))
        .map(|&k| k.to_ascii_uppercase())
        .collect();

    if let Some(route) = cache.get(&(
        start_positions.to_vec(),
        unlocked_doors.clone(),
        collected_keys.clone(),
    )) {
        return route.clone();
    }

    let unlocked_positions: Vec<_> = unlocked_doors
        .iter()
        .map(|d| maze.doors.get(d).unwrap()) // already verified existence using unlocked_doors
        .collect();

    // don't search for the keys we've already collected
    let remaining_keys: Vec<_> =
        find_accessible_keys(&maze.map, &unlocked_positions, start_positions)
            .iter()
            .filter(|&x| !collected_keys.contains(x))
            .cloned()
            .collect();

    let possible_routes =
        get_possible_routes(maze, &unlocked_positions, start_positions, &remaining_keys);
    assert!(
        !possible_routes.is_empty(),
        "Could not find route from {:?} to {:?}",
        start_positions,
        remaining_keys
    );
    let res = possible_routes
        .iter()
        .map(|(dest_k, route_to_k)| {
            let mut new_collected_keys = collected_keys.clone();
            new_collected_keys.insert(*dest_k);
            let new_start_positions: Vec<_> = start_positions
                .iter()
                .map(|&p| {
                    if p == route_to_k[0] {
                        *route_to_k.last().unwrap()
                    } else {
                        p
                    }
                })
                .collect();

            // skip first position, counted in the last move of the previous route segment
            route_to_k[1..]
                .iter()
                .cloned()
                .chain(get_shortest_route(
                    cache,
                    maze,
                    &new_collected_keys,
                    &new_start_positions,
                ))
                .collect::<Vec<_>>()
        })
        .min_by_key(|r| r.len())
        .unwrap();
    cache.insert(
        (
            start_positions.to_vec(),
            unlocked_doors,
            collected_keys.clone(),
        ),
        res.clone(),
    );
    res
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
    .map(|l| l.unwrap())
    .collect();

    let (maze, me) = parse_map(&input_lines);
    //_print_map(&maze.map, &[]);

    let mut cache = HashMap::new();
    let final_route = get_shortest_route(&mut cache, &maze, &BTreeSet::new(), &[me]);
    //_print_map(&maze.map, &final_route);
    println!("Route length: {}", final_route.len());

    // ...      @#@
    // .@.  =>  ###
    // ...      @#@
    let mut adjusted_map = maze.map.clone();
    adjusted_map[me.1 - 1][me.0 - 1] = State::Me;
    adjusted_map[me.1 - 1][me.0] = State::Wall;
    adjusted_map[me.1 - 1][me.0 + 1] = State::Me;
    adjusted_map[me.1][me.0 - 1] = State::Wall;
    adjusted_map[me.1][me.0] = State::Wall;
    adjusted_map[me.1][me.0 + 1] = State::Wall;
    adjusted_map[me.1 + 1][me.0 - 1] = State::Me;
    adjusted_map[me.1 + 1][me.0] = State::Wall;
    adjusted_map[me.1 + 1][me.0 + 1] = State::Me;
    let adjusted_maze = Maze {
        map: adjusted_map,
        keys: maze.keys,
        doors: maze.doors,
    };

    let start_positions = [
        (me.0 - 1, me.1 - 1),
        (me.0 + 1, me.1 - 1),
        (me.0 - 1, me.1 + 1),
        (me.0 + 1, me.1 + 1),
    ];

    let mut cache2 = HashMap::new();
    let final_route = get_shortest_route(
        &mut cache2,
        &adjusted_maze,
        &BTreeSet::new(),
        &start_positions,
    );
    println!("Route length with robots: {}", final_route.len());
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input: Vec<_> = "#########\n\
                             #b.A.@.a#\n\
                             #########"
            .lines()
            .collect();
        let (maze, me) = parse_map(&input);
        let mut cache = HashMap::new();
        let final_route = get_shortest_route(&mut cache, &maze, &BTreeSet::new(), &[me]);
        assert_eq!(final_route.len(), 8);
    }

    #[test]
    fn ex2() {
        let input: Vec<_> = "########################\n\
                             #f.D.E.e.C.b.A.@.a.B.c.#\n\
                             ######################.#\n\
                             #d.....................#\n\
                             ########################"
            .lines()
            .collect();
        let (maze, me) = parse_map(&input);
        let mut cache = HashMap::new();
        let final_route = get_shortest_route(&mut cache, &maze, &BTreeSet::new(), &[me]);
        assert_eq!(final_route.len(), 86);
    }
    #[test]
    fn ex3() {
        let input: Vec<_> = "########################\n\
                             #...............b.C.D.f#\n\
                             #.######################\n\
                             #.....@.a.B.c.d.A.e.F.g#\n\
                             ########################"
            .lines()
            .collect();
        let (maze, me) = parse_map(&input);
        let mut cache = HashMap::new();
        let final_route = get_shortest_route(&mut cache, &maze, &BTreeSet::new(), &[me]);
        assert_eq!(final_route.len(), 132);
    }

    #[test]
    fn ex4() {
        let input: Vec<_> = "#################\n\
                             #i.G..c...e..H.p#\n\
                             ########.########\n\
                             #j.A..b...f..D.o#\n\
                             ########@########\n\
                             #k.E..a...g..B.n#\n\
                             ########.########\n\
                             #l.F..d...h..C.m#\n\
                             #################"
            .lines()
            .collect();
        let (maze, me) = parse_map(&input);
        let mut cache = HashMap::new();
        let final_route = get_shortest_route(&mut cache, &maze, &BTreeSet::new(), &[me]);
        assert_eq!(final_route.len(), 136);
    }

    #[test]
    fn ex5() {
        let input: Vec<_> = "########################\n\
                             #@..............ac.GI.b#\n\
                             ###d#e#f################\n\
                             ###A#B#C################\n\
                             ###g#h#i################\n\
                             ########################"
            .lines()
            .collect();
        let (maze, me) = parse_map(&input);
        let mut cache = HashMap::new();
        let final_route = get_shortest_route(&mut cache, &maze, &BTreeSet::new(), &[me]);
        assert_eq!(final_route.len(), 81);
    }
}
