use itertools::Itertools;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

fn parse_input(input: &str) -> (HashSet<(i32, i32)>, Vec<Vec<i32>>) {
    let mut rules = HashSet::default();
    let mut initial_orderings = vec![];
    for line in input.lines() {
        match line.chars().nth(2) {
            Some('|') => {
                let pair = line.split_once('|').unwrap();
                rules.insert((pair.0.parse().unwrap(), pair.1.parse().unwrap()));
            }
            Some(',') => {
                initial_orderings.push(line.split(',').map(|n| n.parse().unwrap()).collect());
            }
            _ => (),
        }
    }
    (rules, initial_orderings)
}

fn get_first_incorrect_pair(
    rules: &HashSet<(i32, i32)>,
    ordering: &[i32],
) -> Option<(usize, usize)> {
    ordering
        .iter()
        .enumerate()
        .combinations(2)
        .find(|page_pair| !rules.contains(&(*page_pair[0].1, *page_pair[1].1)))
        .map(|page_pair| (page_pair[0].0, page_pair[1].0))
}

fn get_correct_middle(rules: &HashSet<(i32, i32)>, ordering: &[i32]) -> Option<i32> {
    if get_first_incorrect_pair(rules, ordering).is_none() {
        Some(ordering[(ordering.len() - 1) / 2])
    } else {
        None
    }
}

fn fix_ordering(rules: &HashSet<(i32, i32)>, ordering: &[i32]) -> Vec<i32> {
    let mut new_ordering = ordering.to_owned();
    while let Some((idx_a, idx_b)) = get_first_incorrect_pair(rules, &new_ordering) {
        new_ordering.swap(idx_a, idx_b);
    }
    new_ordering
}

fn main() -> io::Result<()> {
    let input: String = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?;

    let (rules, initial_orderings) = parse_input(&input);
    let correct_middle_sum: i32 = initial_orderings
        .iter()
        .filter_map(|o| get_correct_middle(&rules, o))
        .sum();
    println!("P1: Sum of middle pages of correct orderings: {correct_middle_sum}");

    let corrected_middle_sum: i32 = initial_orderings
        .iter()
        .filter(|o| get_first_incorrect_pair(&rules, o).is_some())
        .map(|o| fix_ordering(&rules, o))
        .filter_map(|o| get_correct_middle(&rules, &o))
        .sum();
    println!("P1: Sum of middle pages of corrected orderings: {corrected_middle_sum}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EX_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn ex1() {
        let (rules, initial_orderings) = parse_input(EX_INPUT);

        let correct_middle_sum: i32 = initial_orderings
            .iter()
            .filter_map(|o| get_correct_middle(&rules, o))
            .sum();
        assert_eq!(correct_middle_sum, 143);
    }

    #[test]
    fn ex2() {
        let (rules, initial_orderings) = parse_input(EX_INPUT);

        let corrected_middle_sum: i32 = initial_orderings
            .iter()
            .filter(|o| get_first_incorrect_pair(&rules, o).is_some())
            .map(|o| fix_ordering(&rules, o))
            .filter_map(|o| get_correct_middle(&rules, &o))
            .sum();
        assert_eq!(corrected_middle_sum, 123);
    }
}
