extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_route_hash(key: &[u8], route: &str) -> String {
    let mut hasher = Md5::new();
    hasher.input(key);
    hasher.input(route.as_bytes());
    return hasher.result_str().chars().take(4).collect();
}

fn get_new_pos(p: (u64, u64), dir: char) -> (u64, u64) {
    match dir {
        'U' => (p.0, p.1 - 1),
        'D' => (p.0, p.1 + 1),
        'L' => (p.0 - 1, p.1),
        'R' => (p.0 + 1, p.1),
        _ => unreachable!(),
    }
}

fn is_door_open(c: char) -> bool {
    matches!(c, 'b'..='f')
}

// bfs
fn find_all_routes(key: &[u8]) -> Vec<String> {
    let mut routes = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(("".to_string(), (0, 0)));
    while !queue.is_empty() {
        let (route, pos) = queue.pop_front().unwrap();
        let h = get_route_hash(key, &route);
        // foo
        if pos == (3, 3) {
            routes.push(route);
            continue;
        }
        if pos.1 != 0 && is_door_open(h.chars().next().unwrap()) {
            // U
            queue.push_back((route.clone() + "U", get_new_pos(pos, 'U')));
        }
        if pos.1 != 3 && is_door_open(h.chars().nth(1).unwrap()) {
            // D
            queue.push_back((route.clone() + "D", get_new_pos(pos, 'D')));
        }
        if pos.0 != 0 && is_door_open(h.chars().nth(2).unwrap()) {
            // L
            queue.push_back((route.clone() + "L", get_new_pos(pos, 'L')));
        }
        if pos.0 != 3 && is_door_open(h.chars().nth(3).unwrap()) {
            // R
            queue.push_back((route.clone() + "R", get_new_pos(pos, 'R')));
        }
    }
    routes
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    let input = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap();
    let key = input.as_bytes();

    let routes = find_all_routes(key);

    println!("Shortest route: {} ({})", routes[0], routes[0].len());
    println!(
        "Longest route: {} ({})",
        routes.last().unwrap(),
        routes.last().unwrap().len()
    );
}
