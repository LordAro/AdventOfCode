use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input: String = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?;

    let (ranges_str, ingredient_ids_str) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<_> = ranges_str
        .lines()
        .map(|l| {
            let (s, e) = l.split_once('-').unwrap();
            let s = s.parse::<usize>().unwrap();
            let e = e.parse::<usize>().unwrap();
            s..=e
        })
        .collect();
    ranges.sort_by(|a, b| a.start().cmp(b.start()).then(a.end().cmp(b.end())));

    let p1_fresh_ingredients: usize = ingredient_ids_str
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .filter(|i| ranges.iter().any(|r| r.contains(i)))
        .count();

    let mut merged_ranges = vec![];
    let mut i = 0;
    while i < ranges.len() {
        let start = ranges[i].start();
        let mut end = ranges[i].end();

        while i < ranges.len() - 1 && ranges[i + 1].end() <= end {
            i += 1;
        }
        while i < ranges.len() - 1 && *ranges[i + 1].start() <= end + 1 {
            end = ranges[i + 1].end();
            i += 1;
        }

        merged_ranges.push(start..=end);
        i += 1;
    }

    let p2_total_fresh_ids: usize = merged_ranges.iter().map(|r| *r.end() - *r.start() + 1).sum();
    println!("P1: Number of fresh ingredients: {p1_fresh_ingredients}");
    println!("P2: Total fresh ingredient IDs: {p2_total_fresh_ids}");
    Ok(())
}
