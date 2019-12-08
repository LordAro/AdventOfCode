use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_orbits(map: &HashMap<String, String>, key: &str) -> usize {
    if map[key] == "COM" {
        1
    } else {
        1 + count_orbits(map, &map[key])
    }
}

fn get_orbits<'a>(map: &'a HashMap<String, String>, key: &'a str) -> Vec<&'a str> {
    if key == "COM" {
        vec!["COM"]
    } else {
        [get_orbits(map, &map[key]).as_slice(), vec![key].as_slice()].concat()
    }
}

fn main() {
    let orbits: Vec<(String, String)> = BufReader::new(
        File::open(
            &env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .map(|s| {
        let v: Vec<_> = s.unwrap().split(')').map(|s| s.to_string()).collect();
        (v[1].clone(), v[0].clone())
    })
    .collect();

    let orbit_map: HashMap<_, _> = orbits.into_iter().collect();
    let number_of_orbits: usize = orbit_map.keys().map(|k| count_orbits(&orbit_map, k)).sum();
    println!("Number of orbits: {}", number_of_orbits);

    let san_orbits = get_orbits(&orbit_map, "SAN");
    let you_orbits = get_orbits(&orbit_map, "YOU");
    for i in 0..usize::min(san_orbits.len(), you_orbits.len()) {
        if san_orbits[i] != you_orbits[i] {
            // -2 to exclude YOU & SAN
            println!(
                "Number of orbital moves: {}",
                san_orbits.len() - i + you_orbits.len() - i - 2
            );
            break;
        }
    }
}
