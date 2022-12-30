extern crate itertools;
extern crate regex;

use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Floor = Vec<(String, char)>;
type State = (Vec<Floor>, usize); // Pair of a Vector of floors (4), and the current floor

fn item_combo(item_indexes: &Floor) -> Vec<Floor> {
    item_indexes
        .iter()
        .cloned()
        .combinations(1)
        .chain(item_indexes.iter().cloned().combinations(2))
        .collect()
}

fn is_goal(state: &State) -> bool {
    state.1 == state.0.len() - 1 && state.0[..state.1].iter().flat_map(|v| v.iter()).count() == 0
}

fn is_valid(state: &State) -> bool {
    for floor in &state.0 {
        let (gen, mc): (Floor, Floor) = floor.iter().cloned().partition(|&(_, ref t)| *t == 'g');
        let mc_no_match = mc
            .iter()
            .filter(|&&(ref m, _)| !gen.iter().any(|&(ref g, _)| m == g));
        if mc_no_match.count() != 0 && !gen.is_empty() {
            return false;
        }
    }
    true
}

fn hashed_form(state: &State) -> (Vec<(usize, usize)>, usize) {
    let unique_elems = state
        .0
        .iter()
        .flat_map(|f| f.iter().map(|&(ref e, _)| e.clone()))
        .unique();
    let mut hash_pairs: Vec<_> = unique_elems
        .map(|elem| {
            let g_floor = state
                .0
                .iter()
                .position(|f| f.iter().any(|&(ref g, ref t)| elem == *g && *t == 'g'))
                .unwrap();
            let m_floor = state
                .0
                .iter()
                .position(|f| f.iter().any(|&(ref g, ref t)| elem == *g && *t == 'm'))
                .unwrap();
            (g_floor, m_floor)
        })
        .collect();
    hash_pairs.sort();

    (hash_pairs, state.1)
}

fn move_floor(initial: &State, change: &Floor, up: bool) -> Option<State> {
    if up && initial.1 == 3 || !up && initial.1 == 0 {
        return None;
    }
    let updown: isize = if up { 1 } else { -1 };

    let mut new_state = initial.clone();
    new_state.0[initial.1].retain(|x| !&change.contains(x));
    new_state.0[(initial.1 as isize + updown) as usize].extend(change.clone());
    new_state.1 = (initial.1 as isize + updown) as usize;
    if !is_valid(&new_state) {
        return None;
    }
    Some(new_state)
}

fn move_elevator(initial: State) -> usize {
    let mut cache: HashSet<_> = HashSet::new();
    cache.insert(hashed_form(&initial));

    let mut new_states: Vec<State> = vec![initial];
    let mut depth = 0;
    while !new_states.is_empty() {
        let mut queue: Vec<State> = vec![];
        for state in new_states.drain(..) {
            for mv in item_combo(&state.0[state.1]) {
                // up
                if let Some(new_state) = move_floor(&state, &mv, true) {
                    let hash = hashed_form(&new_state);
                    if !cache.contains(&hash) {
                        cache.insert(hash);
                        queue.push(new_state);
                    }
                }

                // down
                if let Some(new_state) = move_floor(&state, &mv, false) {
                    let hash = hashed_form(&new_state);
                    if !cache.contains(&hash) {
                        cache.insert(hash);
                        queue.push(new_state);
                    }
                }
            }
        }
        depth += 1;
        for s in &queue {
            if is_goal(s) {
                return depth;
            }
        }
        new_states = queue;
    }
    depth // ??
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }
    let input = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap());

    // take first 2 characters
    let generator_re = Regex::new(r"([a-z]{2})[a-z]+ generator").unwrap();
    let microchip_re = Regex::new(r"([a-z]{2})[a-z]+-compatible microchip").unwrap();

    // Parse
    let mut floors: State = (vec![vec![]; 4], 0);
    for (floor_no, floor_opt) in input.lines().enumerate() {
        let floor_str = floor_opt.unwrap();
        for caps in microchip_re.captures_iter(&floor_str) {
            let mc = caps.at(1).unwrap().to_string();
            floors.0[floor_no].push((mc, 'm'));
        }
        for caps in generator_re.captures_iter(&floor_str) {
            let g = caps.at(1).unwrap().to_string();
            floors.0[floor_no].push((g, 'g'));
        }
    }

    let depth = move_elevator(floors.clone());
    println!("Move count: {}", depth);

    // Part 2
    // Adds elerium & dilithium components to floor 0
    floors.0[0].extend(vec![
        ("el".to_string(), 'g'),
        ("el".to_string(), 'm'),
        ("di".to_string(), 'g'),
        ("di".to_string(), 'm'),
    ]);
    let depth2 = move_elevator(floors.clone());
    println!("Actual move count: {}", depth2);
}
