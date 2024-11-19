use std::collections::HashMap;
use std::fs;
use std::io;

fn run_day<'a>(
    termite_map: &HashMap<&'a str, Vec<&'a str>>,
    termites: &HashMap<&'a str, usize>,
) -> HashMap<&'a str, usize> {
    let mut new_numbers: HashMap<&str, usize> = termite_map.keys().map(|t| (*t, 0)).collect();
    for (t, val) in termites {
        for spawn in termite_map.get(t).unwrap() {
            new_numbers.entry(spawn).and_modify(|cur_val| {
                *cur_val += val;
            });
        }
    }
    new_numbers
}

fn read_termite_map(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|l| {
            let mut spl = l.split(':');
            (
                spl.next().unwrap(),
                spl.next().unwrap().split(",").collect::<Vec<_>>(),
            )
        })
        .collect()
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_input_str = fs::read_to_string(p1_input_filename)?;
    let p1_input_map = read_termite_map(&p1_input_str);
    let mut termites = HashMap::from([("A", 1)]);
    for _ in 0..4 {
        termites = run_day(&p1_input_map, &termites);
    }
    println!(
        "P1: Number of termites: {}",
        termites.values().sum::<usize>()
    );

    let p2_input_str = fs::read_to_string(p2_input_filename)?;
    let p2_input_map = read_termite_map(&p2_input_str);
    let mut termites = HashMap::from([("Z", 1)]);
    for _ in 0..10 {
        termites = run_day(&p2_input_map, &termites);
    }
    println!(
        "P2: Number of termites: {}",
        termites.values().sum::<usize>()
    );

    let p3_input_str = fs::read_to_string(p3_input_filename)?;
    let p3_input_map = read_termite_map(&p3_input_str);
    let mut min_pop = usize::MAX;
    let mut max_pop = usize::MIN;
    for k in p3_input_map.keys() {
        let mut termites = HashMap::from([(*k, 1)]);
        for _ in 0..20 {
            termites = run_day(&p3_input_map, &termites);
        }
        let total_pop = termites.values().sum();
        min_pop = usize::min(min_pop, total_pop);
        max_pop = usize::max(max_pop, total_pop);
    }
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
            ("B", vec!["C", "A", "A"]),
            ("C", vec!["A"]),
        ]);
        let mut termites = HashMap::from([("A", 1)]);
        termites = run_day(&input_map, &termites);
        let expected = HashMap::from([("A", 0), ("B", 1), ("C", 1)]);
        assert_eq!(termites, expected);
    }
}
