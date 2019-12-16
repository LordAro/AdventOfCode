use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate advent_of_code;
use advent_of_code::intcode;

type Coord = (i32, i32);

#[derive(PartialEq, Debug)]
enum State {
    Clear,
    Wall,
    Oxygen,
}

fn get_adjacents(pos: Coord) -> [Coord; 4] {
    [
        (pos.0, pos.1 - 1), // north
        (pos.0, pos.1 + 1), // south
        (pos.0 - 1, pos.1), // west
        (pos.0 + 1, pos.1), // east
    ]
}

fn print_positions(positions: &HashMap<Coord, State>) {
    let min_x_coord = positions.keys().min_by_key(|c| c.0).unwrap().0;
    let min_y_coord = positions.keys().min_by_key(|c| c.1).unwrap().1;
    let max_x_coord = positions.keys().max_by_key(|c| c.0).unwrap().0;
    let max_y_coord = positions.keys().max_by_key(|c| c.1).unwrap().1;
    for j in min_y_coord..=max_y_coord {
        for i in min_x_coord..=max_x_coord {
            match positions.get(&(i, j)) {
                Some(State::Clear) => {
                    if (i, j) == (0, 0) {
                        print!("D");
                    } else {
                        print!(".")
                    }
                }
                Some(State::Wall) => print!("#"),
                Some(State::Oxygen) => print!("O"),
                None => print!(" "),
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
    print_positions(&known_positions);
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
    let program_str = BufReader::new(
        File::open(
            &env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .next()
    .unwrap()
    .unwrap();

    let program = intcode::read_input(&program_str);

    let mut droid_pos = (0, 0);

    let mut known_positions: HashMap<Coord, State> = HashMap::new();
    let mut unknown_positions = Vec::new();
    known_positions.insert(droid_pos, State::Clear);
    insert_unknown_positions(&mut unknown_positions, &known_positions, droid_pos);

    let mut mach = intcode::Machine::new(&program, &[]);
    while !unknown_positions.is_empty() {
        let res = mach.run();
        assert!(res == intcode::RunRetVal::NeedsInput);
        let mut last_pos = droid_pos;
        let target_pos = unknown_positions.pop().unwrap();
        for p in get_route(droid_pos, target_pos, &known_positions).drain(1..) {
            let movement_dir = get_dir(last_pos, p) as isize;
            mach.push_input(movement_dir);
            last_pos = p;
            match mach.run() {
                intcode::RunRetVal::Halted | intcode::RunRetVal::NeedsInput => unreachable!(),
                intcode::RunRetVal::Output(x) => {
                    let next_pos = p;
                    match x {
                        0 => {
                            // wall
                            known_positions.insert(next_pos, State::Wall);
                        }
                        1 | 2 => {
                            // moved successfully
                            droid_pos = next_pos;
                            known_positions.insert(
                                droid_pos,
                                if x == 1 { State::Clear } else { State::Oxygen },
                            );
                            insert_unknown_positions(
                                &mut unknown_positions,
                                &known_positions,
                                droid_pos,
                            );
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }

    let oxygen_pos = known_positions
        .iter()
        .find(|&(_, t)| t == &State::Oxygen)
        .unwrap()
        .0;
    println!(
        "Minimum distance to oxygen: {}",
        get_route((0, 0), *oxygen_pos, &known_positions).len() - 1
    );

    let mut minutes = 0;
    let mut searched = HashSet::new();
    let mut to_process = HashSet::new();
    to_process.insert(*oxygen_pos);
    while !to_process.is_empty() {
        let mut next_positions = HashSet::new();
        for c in to_process {
            searched.insert(c);
            for adj in get_adjacents(c).iter() {
                if searched.contains(adj)
                    || known_positions.get(adj).unwrap_or(&State::Wall) == &State::Wall
                {
                    continue;
                }
                next_positions.insert(*adj);
            }
        }
        to_process = next_positions;
        minutes += 1;
    }
    println!("Minutes to refill oxygen: {}", minutes - 1);
}
