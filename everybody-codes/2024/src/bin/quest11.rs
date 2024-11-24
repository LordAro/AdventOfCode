use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::io;

fn run_day<'a>(
    termite_map: &HashMap<&'a str, Vec<&'a str>>,
    termites: &HashMap<&'a str, usize>,
) -> HashMap<&'a str, usize> {
    let mut next_gen = HashMap::with_capacity(termites.len());
    for (t, val) in termites {
        for spawn in &termite_map[t] {
            *next_gen.entry(*spawn).or_insert(0) += val;
        }
    }
    next_gen
}

fn run_simulation(termite_map: &HashMap<&str, Vec<&str>>, num_days: usize, start: &str) -> usize {
    let mut termites = HashMap::from([(start, 1)]);
    for _ in 0..num_days {
        termites = run_day(termite_map, &termites);
    }
    termites.values().sum()
}

fn read_termite_map(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|l| {
            let (parent, children) = l.split_once(':').unwrap();
            (parent, children.split(',').collect::<Vec<_>>())
        })
        .collect()
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_input_str = fs::read_to_string(p1_input_filename)?;
    let p1_input_map = read_termite_map(&p1_input_str);
    let p1_pop_sum = run_simulation(&p1_input_map, 4, "A");
    println!("P1: Number of termites: {p1_pop_sum}");

    let p2_input_str = fs::read_to_string(p2_input_filename)?;
    let p2_input_map = read_termite_map(&p2_input_str);
    let p2_pop_sum = run_simulation(&p2_input_map, 10, "Z");
    println!("P2: Number of termites: {p2_pop_sum}");

    let p3_input_str = fs::read_to_string(p3_input_filename)?;
    let p3_input_map = read_termite_map(&p3_input_str);
    let (min_pop, max_pop) = p3_input_map
        .keys()
        .map(|k| run_simulation(&p3_input_map, 20, k))
        .minmax()
        .into_option()
        .unwrap();

    println!(
        "P3: Termite population difference: {} ({} - {})",
        max_pop - min_pop,
        max_pop,
        min_pop
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input_map = HashMap::from([
            ("A", vec!["B", "C"]),
            ("B", vec!["C", "A"]),
            ("C", vec!["A"]),
        ]);
        let population = run_simulation(&input_map, 4, "A");
        let expected = 8;
        assert_eq!(population, expected);
    }
}
