use std::env;
use std::fs;
use std::io;

fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|l| {
            let (total, numbs) = l.split_once(": ").unwrap();
            let total = total.parse::<usize>().unwrap();
            let numbs = numbs
                .split(' ')
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            (total, numbs)
        })
        .collect()
}

fn concat(a: usize, b: usize) -> usize {
    a * 10_usize.pow(b.ilog10() + 1) + b
}

fn can_sum<const WITH_CONCAT: bool>(
    target_total: usize,
    total: usize,
    remaining_ints: &[usize],
) -> bool {
    if total > target_total {
        return false;
    }
    if remaining_ints.is_empty() {
        return target_total == total;
    }

    can_sum::<WITH_CONCAT>(
        target_total,
        total + remaining_ints[0],
        &remaining_ints[1..],
    ) || can_sum::<WITH_CONCAT>(
        target_total,
        total * remaining_ints[0],
        &remaining_ints[1..],
    ) || WITH_CONCAT
        && can_sum::<WITH_CONCAT>(
            target_total,
            concat(total, remaining_ints[0]),
            &remaining_ints[1..],
        )
}

fn main() -> io::Result<()> {
    let input: String = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?;

    let calibration_equations = parse_input(&input);

    let calibration_total: usize = calibration_equations
        .iter()
        .filter(|(total, ints)| can_sum::<false>(*total, 0, ints))
        .map(|(total, _)| total)
        .sum();

    println!("P1: Total calibration result: {calibration_total}");

    let concat_calibration_total: usize = calibration_equations
        .iter()
        .filter(|(total, ints)| can_sum::<true>(*total, 0, ints))
        .map(|(total, _)| total)
        .sum();
    println!("P2: Total concat calibration result: {concat_calibration_total}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EX_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn ex1() {
        let calibration_result: usize = parse_input(EX_INPUT)
            .iter()
            .filter(|(total, ints)| can_sum::<false>(*total, 0, ints))
            .map(|(total, _)| total)
            .sum();
        assert_eq!(calibration_result, 3749);
    }
}
