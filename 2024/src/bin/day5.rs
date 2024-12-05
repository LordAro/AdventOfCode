use std::cmp::Ordering;
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

fn get_sorter_func(rules: &HashSet<(i32, i32)>) -> impl Fn(&i32, &i32) -> Ordering + use<'_> {
    move |a, b| {
        if rules.contains(&(*a, *b)) {
            Ordering::Less
        } else if rules.contains(&(*b, *a)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

fn get_correct_middle_sum(rules: &HashSet<(i32, i32)>, all_orderings: &[Vec<i32>]) -> i32 {
    // for some reason is_sorted_by requires a bool return value rather than a Ordering
    let bool_sorter_func = |a, b| get_sorter_func(rules)(a, b).is_le();
    all_orderings
        .iter()
        .filter(|o| o.is_sorted_by(bool_sorter_func))
        .map(|o| o[o.len() / 2])
        .sum()
}

fn get_corrected_middle_sum(rules: &HashSet<(i32, i32)>, all_orderings: &[Vec<i32>]) -> i32 {
    let bool_sorter_func = |a, b| get_sorter_func(rules)(a, b).is_le();
    all_orderings
        .iter()
        .filter(|o| !o.is_sorted_by(bool_sorter_func))
        .map(|o| {
            let mut new_o = o.clone();
            new_o.sort_by(get_sorter_func(rules));
            new_o
        })
        .map(|o| o[o.len() / 2])
        .sum()
}

fn main() -> io::Result<()> {
    let input: String = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?;

    let (rules, initial_orderings) = parse_input(&input);
    println!(
        "P1: Sum of middle pages of correct orderings: {}",
        get_correct_middle_sum(&rules, &initial_orderings)
    );
    println!(
        "P2: Sum of middle pages of correct orderings: {}",
        get_corrected_middle_sum(&rules, &initial_orderings)
    );
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
        assert_eq!(get_correct_middle_sum(&rules, &initial_orderings), 143);
    }

    #[test]
    fn ex2() {
        let (rules, initial_orderings) = parse_input(EX_INPUT);
        assert_eq!(get_corrected_middle_sum(&rules, &initial_orderings), 123);
    }
}
