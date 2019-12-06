use std::cmp::min;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn count_orbits(map: &HashMap<String, String>, key: &str) -> usize {
    if map[key] == "COM" {
        1
    } else {
        1 + count_orbits(map, &map[key])
    }
}

fn get_orbits(map: &HashMap<String, String>, key: &str) -> Vec<String> {
    if key == "COM" {
        vec!["COM".to_string()]
    } else {
        [
            get_orbits(map, &map[key]).as_slice(),
            vec![key.to_string()].as_slice(),
        ]
        .concat()
    }
}

fn main() -> io::Result<()> {
    let orbits: Vec<_> = BufReader::new(
        File::open(
            &env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .map(|s| {
        let v: Vec<_> = s.unwrap().split(')').map(|s| String::from(s)).collect();
        (v[1].clone(), v[0].clone())
    })
    .collect();

    let orbit_map: HashMap<_, _> = orbits.into_iter().collect();
    let number_of_orbits: usize = orbit_map.keys().map(|k| count_orbits(&orbit_map, k)).sum();
    println!("Number of orbits: {}", number_of_orbits);

    let san_orbits = get_orbits(&orbit_map, "SAN");
    let you_orbits = get_orbits(&orbit_map, "YOU");
    for i in 0..min(san_orbits.len(), you_orbits.len()) {
        if san_orbits[i] != you_orbits[i] {
            // -2 to exclude YOU & SAN
            println!(
                "Number of orbital moves: {}",
                san_orbits.len() - i + you_orbits.len() - i - 2
            );
            break;
        }
    }
    Ok(())
}
