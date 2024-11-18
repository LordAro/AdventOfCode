use memoise::memoise;
use std::fs;
use std::io;

#[memoise(sparkball)]
fn get_beetle_count(available_stamps: &[usize], sparkball: usize) -> Option<usize> {
    if sparkball == 0 {
        return Some(0);
    }
    let mut min_count = None;
    for stamp in available_stamps {
        let n = sparkball
            .checked_sub(*stamp)
            .and_then(|n| get_beetle_count(available_stamps, n));
        if let Some(n) = n {
            if min_count.is_none() || n < min_count? {
                min_count = Some(n + 1);
            }
        }
    }
    min_count
}

fn get_beetle_count_many(available_stamps: &[usize], sparkballs: &[usize]) -> usize {
    sparkballs
        .iter()
        .map(|&sb| get_beetle_count(available_stamps, sb).unwrap())
        .sum()
}

fn get_beetle_count_pair(available_stamps: &[usize], sparkball: usize) -> usize {
    let mut min = usize::MAX;
    for pair_a in (sparkball - 100) / 2..=sparkball / 2 {
        let pair_b = sparkball - pair_a;
        if pair_a.abs_diff(pair_b) > 100 {
            // Hack
            continue;
        }
        let count = get_beetle_count(available_stamps, pair_a).unwrap()
            + get_beetle_count(available_stamps, pair_b).unwrap();
        if count < min {
            min = count;
        }
    }
    min
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_sparkballs: Vec<_> = fs::read_to_string(p1_input_filename)?
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    let p1_stamps = [10, 5, 3, 1];
    println!(
        "P1: Minimum number of beetles: {}",
        get_beetle_count_many(&p1_stamps, &p1_sparkballs)
    );

    get_beetle_count_reset(); // cache reset

    let p2_sparkballs: Vec<_> = fs::read_to_string(p2_input_filename)?
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    let p2_stamps = [30, 25, 24, 20, 16, 15, 10, 5, 3, 1];
    println!(
        "P2: Minimum number of beetles: {}",
        get_beetle_count_many(&p2_stamps, &p2_sparkballs)
    );

    get_beetle_count_reset(); // cache reset

    let p3_sparkballs: Vec<_> = fs::read_to_string(p3_input_filename)?
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    let p3_stamps = [
        1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101,
    ];
    // preseed memoisation cache to prevent recursion-based stack overflow
    for n in 0..p3_sparkballs.iter().max().unwrap() / 2 {
        get_beetle_count(&p3_stamps, n);
    }
    let p3_beetle_count_sum: usize = p3_sparkballs
        .iter()
        .map(|&sb| get_beetle_count_pair(&p3_stamps, sb))
        .sum();
    println!("P3: Minimum number of beetles: {p3_beetle_count_sum}",);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let sparkballs = [2, 4, 7, 16];
        let p1_stamps = [10, 5, 3, 1];
        assert_eq!(get_beetle_count_many(&p1_stamps, &sparkballs), 10);
    }

    #[test]
    fn ex2() {
        let sparkballs = [33, 41, 55, 99];
        let p2_stamps = [30, 25, 24, 20, 16, 15, 10, 5, 3, 1];
        assert_eq!(get_beetle_count(&p2_stamps, 33), Some(2));
        assert_eq!(get_beetle_count(&p2_stamps, 41), Some(2));
        assert_eq!(get_beetle_count(&p2_stamps, 55), Some(2));
        assert_eq!(get_beetle_count(&p2_stamps, 99), Some(4));
        assert_eq!(get_beetle_count_many(&p2_stamps, &sparkballs), 10);
    }

    #[test]
    fn ex3() {
        let p3_stamps = [
            1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101,
        ];
        let sparkballs = [156488, 352486, 546212];
        // preseed cache
        for n in 0..sparkballs.iter().max().unwrap() / 2 {
            get_beetle_count(&p3_stamps, n);
        }
        assert_eq!(get_beetle_count_pair(&p3_stamps, 156488), 775 + 775);
        assert_eq!(get_beetle_count_pair(&p3_stamps, 352486), 1745 + 1745);
        assert_eq!(get_beetle_count_pair(&p3_stamps, 546212), 2705 + 2704);
    }
}
