use std::env;
use std::fs;
use std::io;

fn is_safe(report: &[i32]) -> bool {
    let is_ascending = report[0] < report[1];
    for arr in report.windows(2) {
        let a = arr[0];
        let b = arr[1];
        if a == b || (a < b) != is_ascending || a.abs_diff(b) > 3 {
            return false;
        }
    }
    true
}

fn is_tolerated_safe(report: &[i32]) -> bool {
    if is_safe(report) {
        return true;
    }
    // ok, remove an item and try again
    // Not especially efficient, but good enough
    for i in 0..report.len() {
        let new_arr: Vec<_> = report[..i]
            .iter()
            .chain(report[i + 1..].iter())
            .copied()
            .collect();
        if is_safe(&new_arr) {
            return true;
        }
    }
    false
}

fn main() -> io::Result<()> {
    let input: String = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?;

    let level_reports: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.split(' ').map(|n| n.parse().unwrap()).collect())
        .collect();

    let safe_reports = level_reports.iter().filter(|r| is_safe(r)).count();

    println!("P1: Number of safe reports: {safe_reports}");

    let mostly_safe_reports = level_reports
        .iter()
        .filter(|r| is_tolerated_safe(r))
        .count();
    println!("P2: Number of tolerated safe reports: {mostly_safe_reports}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        assert_eq!(is_safe(&[7, 6, 4, 2, 1]), true);
        assert_eq!(is_safe(&[1, 2, 7, 8, 9]), false);
        assert_eq!(is_safe(&[9, 7, 6, 2, 1]), false);
        assert_eq!(is_safe(&[1, 3, 2, 4, 5]), false);
        assert_eq!(is_safe(&[8, 6, 4, 4, 1]), false);
        assert_eq!(is_safe(&[1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn ex2() {
        assert_eq!(is_tolerated_safe(&[7, 6, 4, 2, 1]), true);
        assert_eq!(is_tolerated_safe(&[1, 2, 7, 8, 9]), false);
        assert_eq!(is_tolerated_safe(&[9, 7, 6, 2, 1]), false);
        assert_eq!(is_tolerated_safe(&[1, 3, 2, 4, 5]), true);
        assert_eq!(is_tolerated_safe(&[8, 6, 4, 4, 1]), true);
        assert_eq!(is_tolerated_safe(&[1, 3, 6, 7, 9]), true);
    }
}
