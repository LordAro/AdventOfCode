use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::ops::RangeInclusive;

fn range_overlaps(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    return (b.start() >= a.start() && b.start() <= a.end())
        || (b.end() >= a.start() && b.end() <= a.end())
        || (b.start() - 1 == *a.end());
}

fn merge_range(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> RangeInclusive<u32> {
    return u32::min(*a.start(), *b.start())..=u32::max(*a.end(), *b.end());
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    let input = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();

    let mut blocked_ranges: Vec<RangeInclusive<u32>> = input
        .iter()
        .map(|l| {
            let mut it = l.split('-');
            return it.next().unwrap().parse::<u32>().unwrap()
                ..=it.next().unwrap().parse::<u32>().unwrap();
        })
        .collect();

    blocked_ranges.sort_by(|a, b| a.start().partial_cmp(b.start()).unwrap());

    let mut merged_ranges: Vec<RangeInclusive<u32>> = Vec::new();
    for r in &blocked_ranges {
        let mut found_match = false;
        for mr in &mut merged_ranges {
            if range_overlaps(mr, r) {
                *mr = merge_range(mr, r);
                found_match = true;
                break;
            }
        }
        if !found_match {
            merged_ranges.push(r.clone());
        }
    }

    // bit of a hack
    //  - merged_ranges is sorted
    //  - assumes starts at 0, ends at u32::max
    let num_allowed_ips: u32 = merged_ranges
        .windows(2)
        .map(|win| win[1].start() - win[0].end() - 1)
        .sum();

    println!("First allowed IP: {}", merged_ranges[0].end() + 1);
    println!("Number of allowed IPs: {}", num_allowed_ips);
}
