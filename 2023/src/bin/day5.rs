use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

struct RangeMapping {
    dest_start: i64,
    range: std::ops::Range<i64>,
}

fn do_mapping(mappings: &[RangeMapping], r: std::ops::Range<i64>) -> std::ops::Range<i64> {
    let enclosing_range = mappings
        .iter()
        .find(|m| m.range.contains(&r.start) && m.range.contains(&(r.end - 1)));
    // we already know that all the ranges we find are fully enclosed, due to usage of split_range
    // (and p1 ranges are all 1-length)
    match enclosing_range {
        Some(m) => {
            let offset = m.dest_start - m.range.start;
            r.start + offset..r.end + offset
        }
        None => r.clone(),
    }
}

fn split_range(mappings: &[RangeMapping], r: std::ops::Range<i64>) -> Vec<std::ops::Range<i64>> {
    let intersecting_mappings: Vec<_> = mappings
        .iter()
        .filter(|m| {
            m.range.contains(&r.start)
                || m.range.contains(&(r.end - 1))
                || r.contains(&m.range.start)
                || r.contains(&(m.range.end - 1))
        })
        .collect();

    let mut points = vec![r.start, r.end];
    for m in &intersecting_mappings {
        // covers both the case where a range is entirely enclosed, and also partially enclosed
        if r.contains(&(m.range.end - 1)) {
            points.push(m.range.end);
        }
        if r.contains(&m.range.start) {
            points.push(m.range.start);
        }
    }
    points.sort();
    points.dedup(); // avoids constructing 0-length ranges

    // reconstruct split ranges
    points
        .windows(2)
        .map(|w| match w {
            &[s, e] => s..e,
            _ => unreachable!(),
        })
        .collect()
}

fn get_location_numbers(
    almanac: &[Vec<RangeMapping>],
    start_range: std::ops::Range<i64>,
) -> Vec<std::ops::Range<i64>> {
    almanac.iter().fold(vec![start_range], |ranges, mappings| {
        ranges
            .iter()
            .flat_map(|r| {
                let split_ranges = split_range(mappings, r.clone());
                split_ranges
                    .iter()
                    .map(|r2| do_mapping(mappings, r2.clone()))
                    .collect::<Vec<_>>()
            })
            .collect()
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

    // parse, mini state machine type thing
    let mut input_seeds: Vec<i64> = vec![];
    let mut almanac: Vec<Vec<RangeMapping>> = vec![];
    for line in input_data {
        if input_seeds.is_empty() {
            input_seeds = line
                .split_ascii_whitespace()
                .filter_map(|n| n.parse::<i64>().ok())
                .collect();
        } else if line.is_empty() {
            // nothing
        } else if line.contains("map") {
            almanac.push(vec![]);
        } else {
            let parsed_numbers: Vec<_> = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            almanac.last_mut().unwrap().push(RangeMapping {
                dest_start: parsed_numbers[0],
                range: parsed_numbers[1]..parsed_numbers[1] + parsed_numbers[2],
            });
        }
    }

    // part 1, use 1-length ranges so we can reuse the solution easily
    let minimum_location = input_seeds
        .iter()
        .flat_map(|&s| get_location_numbers(&almanac, s..s + 1))
        .map(|r| r.start)
        .min()
        .unwrap();

    println!("Lowest location number: {}", minimum_location);

    // part 2, convert seeds to their range form
    let ranged_seeds: Vec<_> = input_seeds
        .chunks_exact(2)
        .map(|arr| match arr {
            &[s, l] => s..s + l,
            _ => unreachable!(),
        })
        .collect();

    let ranged_locations: Vec<_> = ranged_seeds
        .iter()
        .flat_map(|s| get_location_numbers(&almanac, s.clone()))
        .collect();

    println!("{:?}", ranged_locations);

    let minimum_ranged_location = ranged_locations.iter().map(|r| r.start).min().unwrap();
    println!(
        "Lowest location number using ranges: {}",
        minimum_ranged_location
    );

    Ok(())
}
