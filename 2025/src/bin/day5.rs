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
            (s.parse::<usize>().unwrap(), e.parse::<usize>().unwrap())
        })
        .collect();
    ranges.sort();

    let p1_fresh_ingredients: usize = ingredient_ids_str
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .filter(|i| ranges.iter().any(|r| *i >= r.0 && *i <= r.1))
        .count();

    let mut p2_total_fresh_ids = 0;
    let mut i = 0;
    while i < ranges.len() {
        let (start, mut end) = ranges[i];

        // merge
        while i < ranges.len() - 1 && ranges[i + 1].0 <= end + 1 {
            end = end.max(ranges[i + 1].1); // ignore if totally enclosed
            i += 1;
        }

        p2_total_fresh_ids += end - start + 1; // don't need to actually keep the ids themselves
        i += 1;
    }

    println!("P1: Number of fresh ingredients: {p1_fresh_ingredients}");
    println!("P2: Total fresh ingredient IDs: {p2_total_fresh_ids}");
    Ok(())
}
