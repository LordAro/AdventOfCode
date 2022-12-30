use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_linked(data: &Vec<Vec<usize>>, idx: usize, linked: &mut HashSet<usize>) {
    if !linked.insert(idx) {
        return;
    }
    data[idx].iter().for_each(|i| get_linked(data, *i, linked));
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided");
    }
    let input: Vec<Vec<_>> = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
        .lines()
        .map(|l| {
            l.unwrap()
                .split(" <-> ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    let mut groups: HashMap<_, HashSet<_>> = HashMap::new();
    for n in 0..input.len() {
        if groups.iter().flat_map(|(_, v)| v.iter()).any(|&x| x == n) {
            continue;
        }
        groups.insert(n, HashSet::new());
        get_linked(&input, n, groups.get_mut(&n).unwrap());
    }

    println!("Group 0 size: {}", groups.get(&0).unwrap().len());
    println!("Total groups: {}", groups.keys().count());
}
