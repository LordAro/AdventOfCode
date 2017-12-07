extern crate itertools;

use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use itertools::Itertools;

fn get_total_weight(map: &HashMap<String, (i32, Vec<String>)>, item: &str) -> i32 {
    match map.get(item) {
        Some(&(weight, ref children)) => {
            return weight +
                children
                    .iter()
                    .map(|i| get_total_weight(map, i))
                    .sum::<i32>()
        }
        None => panic!("Unknown item: {}", item),
    };
}

fn get_child_weights(map: &HashMap<String, (i32, Vec<String>)>, item: &str) -> Vec<(String, i32)> {
    map.get(item)
        .unwrap()
        .1
        .iter()
        .map(|c| (c.clone(), get_total_weight(&map, c)))
        .collect()
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided");
    }
    let input: Vec<_> = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let splitted: Vec<Vec<_>> = input.iter().map(|l| l.split(" -> ").collect()).collect();
    let mut map: HashMap<String, (i32, Vec<String>)> = HashMap::new();
    for v in splitted.iter() {
        let mut it = v[0].split_whitespace();
        let key = it.next().unwrap();
        let weight: i32 = it.next()
            .unwrap()
            .chars()
            .skip(1)
            .take_while(|c| c.is_numeric())
            .collect::<String>()
            .parse()
            .unwrap();
        // Ew
        map.insert(key.to_string(), (
            weight,
            if v.len() == 2 {
                v[1].split(", ").map(|s| s.to_string()).collect()
            } else {
                Vec::new()
            },
        ));
    }

    let mut root = "";
    for (key, _) in &map {
        let mut has_parents = false;
        for (_, val) in &map {
            if val.1.iter().find(|s| s.as_str() == key) != None {
                has_parents = true;
                break;
            }
        }
        if !has_parents {
            root = key;
            break;
        }
    }
    println!("Root: {}", root);

    let mut node = root.to_string();
    loop {
        let child_weights = get_child_weights(&map, &node);
        if child_weights.len() < 2 {
            panic!("Node with {} children", child_weights.len());
        }

        if child_weights.iter().map(|p| p.1).all_equal() {
            break;
        }

        let f: Vec<_> = child_weights
            .iter()
            .skip(1)
            .filter(|c| child_weights[0].1 != c.1)
            .collect();
        node = if f.len() == 1 {
            f[0].0.clone()
        } else {
            child_weights[0].0.clone()
        };
    }

    let base_node_weight = &map.get(&node).unwrap().0;
    let node_weight = get_total_weight(&map, &node);
    // find parent
    let p_node = map.iter()
        .filter(|&(_, v)| v.1.iter().find(|s| s.as_str() == node) != None)
        .next()
        .unwrap()
        .clone();
    let other_weight = (p_node.1)
        .1
        .iter()
        .filter(|&ref c| c.as_str() != node.as_str())
        .map(|c| get_total_weight(&map, &c))
        .next()
        .unwrap();

    println!(
        "New weight for {}: {}",
        node,
        base_node_weight + other_weight - node_weight
    );
}
