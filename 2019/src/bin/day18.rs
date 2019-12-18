use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = (usize, usize);

#[derive(Debug, PartialEq)]
enum State {
    Clear,
    Wall,
    Key(char),
    Door(char),
    Me,
}

fn get_adjacents(pos: Coord) -> [Coord; 4] {
    [
        (pos.0, pos.1 - 1), // north
        (pos.0, pos.1 + 1), // south
        (pos.0 - 1, pos.1), // west
        (pos.0 + 1, pos.1), // east
    ]
}

fn print_map(positions: &Vec<Vec<State>>) {
    for j in 0..positions.len() {
        for i in 0..positions[j].len() {
            match positions[j][i] {
                State::Clear => print!("."),
                State::Wall => print!("#"),
                State::Me => print!("@"),
                State::Key(k) => print!("{}", k),
                State::Door(d) => print!("{}", d),
            }
        }
        println!();
    }
}

fn get_route(source: Coord, target: Coord, known_positions: &HashMap<Coord, State>) -> Vec<Coord> {
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
            return total_path;
        }
        if known_positions.get(&current).unwrap_or(&State::Wall) == &State::Wall {
            continue;
        }

        for adj in get_adjacents(current).into_iter() {
            let dist = g_score[&current] + 1;
            if &dist < g_score.get(&adj).unwrap_or(&isize::max_value()) {
                came_from.insert(*adj, current);
                g_score.insert(*adj, dist);
                open_set.push_back(*adj);
            }
        }
    }
    panic!("Unable to find route between {:?} and {:?}", source, target);
}

fn insert_unknown_positions(unknowns: &mut Vec<Coord>, knowns: &HashMap<Coord, State>, pos: Coord) {
    for adj in get_adjacents(pos).into_iter() {
        if !knowns.contains_key(adj) {
            unknowns.push(*adj);
        }
    }
}

fn get_dir(a: Coord, b: Coord) -> usize {
    get_adjacents(a)
        .iter()
        .enumerate()
        .filter(|&(_, adj)| adj == &b)
        .map(|(i, _)| i + 1) // 1 based
        .next()
        .unwrap()
}

fn main() {
    let map: Vec<Vec<_>> = BufReader::new(
        File::open(
            &env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .map(|l| {
        l.unwrap()
            .chars()
            .map(|c| match c {
                '#' => State::Wall,
                '.' => State::Clear,
                '@' => State::Me,
                'a'..='z' => State::Key(c),
                'A'..='Z' => State::Door(c),
                _ => panic!("Unrecognised character {}", c),
            })
            .collect()
    })
    .collect();

    let targets: HashMap<_, _> = map
        .iter()
        .enumerate()
        .flat_map(|(j, r)| {
            r.iter().enumerate().filter_map(move |(i, e)| match e {
                State::Key(_) | State::Door(_) => Some(((i, j), e)),
                _ => None,
            })
        })
        .collect();

    println!("{:?}", targets);
    print_map(&map);
}
