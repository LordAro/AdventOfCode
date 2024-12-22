use memoise::memoise_map;
use std::env;
use std::fs;
use std::io;

#[memoise_map(design)]
fn can_make_design(fragments: &[&str], design: String) -> usize {
    if design.is_empty() {
        return 1;
    }

    fragments
        .iter()
        .filter_map(|frag| {
            design
                .strip_prefix(frag)
                .map(|suffix| can_make_design(fragments, suffix.to_string()))
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?;
    let (towel_pattern_line, desired_designs) = input.split_once("\n\n").unwrap();
    let mut towel_patterns: Vec<_> = towel_pattern_line.split(", ").collect();
    // sort by length so we automatically match the longest substring first
    towel_patterns.sort_by_key(|p| -(p.len() as i64));
    let desired_designs: Vec<_> = desired_designs.lines().collect();

    let design_counts: Vec<_> = desired_designs
        .iter()
        .map(|d| can_make_design(&towel_patterns, d.to_string()))
        .collect();
    let num_possible_designs = design_counts.iter().filter(|n| **n > 0).count();
    let total_designs: usize = design_counts.iter().sum();
    println!("P1: Number of possible towel designs: {num_possible_designs}");
    println!("P2: Total number of possible towel designs: {total_designs}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let fragments = ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
        assert_eq!(can_make_design(&fragments, "brwrr".to_string()), 2);
        assert_eq!(can_make_design(&fragments, "bggr".to_string()), 1);
        assert_eq!(can_make_design(&fragments, "gbbr".to_string()), 4);
        assert_eq!(can_make_design(&fragments, "rrbgbr".to_string()), 6);
        assert_eq!(can_make_design(&fragments, "ubwu".to_string()), 0);
        assert_eq!(can_make_design(&fragments, "bwurrg".to_string()), 1);
        assert_eq!(can_make_design(&fragments, "brgr".to_string()), 2);
        assert_eq!(can_make_design(&fragments, "bbrwb".to_string()), 0);
    }
}
