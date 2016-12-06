use std::collections::btree_map::BTreeMap;
use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead};

fn most_least_common(btm: BTreeMap<char, i32>) -> (char, char) {
    let mut count_vec: Vec<_> = btm.into_iter().collect();
    // Reverse sort the vector of pairs by "value" (sorted by "key" in case of tie)
    count_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let m = count_vec.first().map(|&(k, _)| k).unwrap();
    let l = count_vec.last().map(|&(k, _)| k).unwrap();
    return (m, l);
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    let input = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap());

    let mut cols: Vec<BTreeMap<char, i32>> = vec![];

    for line in input.lines() {
        for (i, c) in line.unwrap().chars().enumerate() {
            if i == cols.len() {
                cols.push(BTreeMap::new());
            }
            *cols[i].entry(c).or_insert(0) += 1;
        }
    }
    let mut most = String::new();
    let mut least = String::new();
    for c in cols {
        let (m, l) = most_least_common(c);
        most.push(m);
        least.push(l);
    }
    println!("Most common message: {}", most);
    println!("Least common message: {}", least);
}
