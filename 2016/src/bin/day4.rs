use std::collections::btree_map::BTreeMap;
use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead};

struct Room {
    name: String,
    sector: u32,
    checksum: String,
}

// Return 5 most frequent letters
fn five_most_common(s: &str) -> String {
    // Store letter frequency in BTreeMap
    let mut count = BTreeMap::new();
    for c in s.chars() {
        if c == '-' {
            continue;
        }
        *count.entry(c).or_insert(0) += 1;
    }
    let mut count_vec: Vec<_> = count.into_iter().collect();
    // Reverse sort the vector of pairs by "value" (sorted by "key" in case of tie)
    count_vec.sort_by(|a, b| b.1.cmp(&a.1));
    return count_vec.iter().take(5).map(|&(k, _)| k).collect();
}

// Rotate a string n times (n == 13 would be rot13)
fn rotn(s: &str, n: u32) -> String {
    let base = 'a' as u8;
    s.chars()
        .map(|c| c as u8 - base)
        .map(|ord| ((ord as u32 + n) % 26) as u8)
        .map(|rot| (rot + base) as char)
        .collect()
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    let input = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap());

    let rooms: Vec<_> = input.lines()
        .map(|l| l.unwrap().rsplitn(2, '-').map(String::from).collect::<Vec<_>>())
        .map(|l| {
            let cc: Vec<_> = l[0].trim_right_matches(']').split('[').collect();
            Room {
                name: l[1].replace("-", ""),
                sector: cc[0].parse().unwrap(),
                checksum: String::from(cc[1]),
            }
        })
        .filter(|r| five_most_common(&r.name) == r.checksum) // Remove "decoys"
        .collect();
    let count: u32 = rooms.iter().map(|r| r.sector).sum();
    println!("Value of valid rooms: {}", count);

    let target_sector: u32 = rooms
        .iter()
        .filter(|r| rotn(&r.name, r.sector) == "northpoleobjectstorage")
        .map(|r| r.sector)
        .sum();
    println!("North pole object storage sector: {}", target_sector);
}
