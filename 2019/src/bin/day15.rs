use std::collections::HashMap;
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

// Horrendously inefficient BFS, needs replacing
fn get_route(source: Coord, target: Coord, known_positions: &HashMap<Coord, State>) -> Vec<Coord> {
    let mut to_search = VecDeque::new();
    to_search.push_back(vec![source]);

    while !to_search.is_empty() {
        let search_path = to_search.pop_front().unwrap();
        let search_node = search_path.last().unwrap();

        if *search_node == target {
            //println!("{:?}", search_path);
            return search_path;
        }

        if *known_positions.get(&search_node).unwrap_or(&State::Wall) == State::Wall {
            continue;
        }

        for adj in get_adjacents(*search_node).into_iter() {
            if !search_path.contains(adj) {
                let mut v = search_path.clone();
                v.push(*adj);
                to_search.push_back(v);
            }
        }
    }
    print_positions(&known_positions);
    panic!("Unable to find route between {:?} and {:?}", source, target);
}

fn insert_unknown_positions(
    unknowns: &mut VecDeque<Coord>,
    knowns: &HashMap<Coord, State>,
    pos: Coord,
) {
    for adj in get_adjacents(pos).into_iter() {
        if !knowns.contains_key(adj) {
            unknowns.push_back(*adj);
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
    let mut unknown_positions: VecDeque<Coord> = VecDeque::new();
    known_positions.insert(droid_pos, State::Clear);
    insert_unknown_positions(&mut unknown_positions, &known_positions, droid_pos);

    let mut mach = intcode::Machine::new(&program, &[]);
    'outer: while !unknown_positions.is_empty() {
        let res = mach.run();
        assert!(res == intcode::RunRetVal::NeedsInput);
        let mut last_pos = droid_pos;
        let target_pos = unknown_positions.pop_front().unwrap();
        //println!(
        //    "Trying to find route from {:?} to {:?}",
        //    droid_pos, target_pos
        //);
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
                        1 => {
                            // moved successfully
                            droid_pos = next_pos;
                            known_positions.insert(droid_pos, State::Clear);
                            insert_unknown_positions(
                                &mut unknown_positions,
                                &known_positions,
                                droid_pos,
                            );
                        }
                        2 => {
                            droid_pos = next_pos;
                            known_positions.insert(droid_pos, State::Oxygen);
                            insert_unknown_positions(
                                &mut unknown_positions,
                                &known_positions,
                                droid_pos,
                            );
                            //break 'outer; // Done, now find shortest route
                        } // moved and found destination
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
    print_positions(&known_positions);
}
