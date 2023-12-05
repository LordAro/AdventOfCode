el  2m50.892s
se std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Range {
    dest_start: u64,
    src_start: u64,
    length: u64,
}

fn get_location_number(almanac: &Vec<Vec<Range>>, n: u64) -> u64 {
    almanac.iter().fold(n, |n, mappings| {
        let matched_mapping = mappings
            .iter()
            .filter(|m| n >= m.src_start && n < m.src_start + m.length)
            .next();
        match matched_mapping {
            Some(m) => m.dest_start + (n - m.src_start),
            None => n,
        }
    })
}

fn main() -> io::Result<()> {
    let input_data: Vec<String> = BufReader::new(
        File::open(
            env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .map(|l| l.unwrap())
    .collect();

    //    let input_data: Vec<&str> = vec![
    //        "seeds: 79 14 55 13",
    //        "",
    //        "seed-to-soil map:",
    //        "50 98 2",
    //        "52 50 48",
    //        "",
    //        "soil-to-fertilizer map:",
    //        "0 15 37",
    //        "37 52 2",
    //        "39 0 15",
    //        "",
    //        "fertilizer-to-water map:",
    //        "49 53 8",
    //        "0 11 42",
    //        "42 0 7",
    //        "57 7 4",
    //        "",
    //        "water-to-light map:",
    //        "88 18 7",
    //        "18 25 70",
    //        "",
    //        "light-to-temperature map:",
    //        "45 77 23",
    //        "81 45 19",
    //        "68 64 13",
    //        "",
    //        "temperature-to-humidity map:",
    //        "0 69 1",
    //        "1 0 69",
    //        "",
    //        "humidity-to-location map:",
    //        "60 56 37",
    //        "56 93 4",
    //    ];

    let mut input_seeds: Vec<u64> = vec![];
    let mut almanac: Vec<Vec<Range>> = vec![];
    for line in input_data {
        if input_seeds.is_empty() {
            input_seeds = line
                .split_ascii_whitespace()
                .filter_map(|n| n.parse::<u64>().ok())
                .collect();
        } else if line.is_empty() {
            // nothing
        } else if line.contains("map") {
            almanac.push(vec![]);
        } else {
            let parsed_numbers: Vec<_> = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
            almanac.last_mut().unwrap().push(Range {
                dest_start: parsed_numbers[0],
                src_start: parsed_numbers[1],
                length: parsed_numbers[2],
            });
        }
    }

    let locations: Vec<_> = input_seeds
        .iter()
        .map(|&s| get_location_number(&almanac, s))
        .collect();

    println!(
        "Lowest location number: {}",
        locations.iter().min().unwrap()
    );

    let ranged_seeds: Vec<_> = input_seeds
        .chunks_exact(2)
        .flat_map(|arr| match arr {
            &[s, l] => s..s + l,
            _ => panic!("Unmatched"),
        })
        .collect();

    let minimum_ranged_location = ranged_seeds
        .iter()
        .map(|&s| get_location_number(&almanac, s))
        .min()
        .unwrap();
    println!("{}", minimum_ranged_location);

    Ok(())
}
