use std::cell;
use std::fs;
use std::io;

fn flock_checksum(ducks: &[usize]) -> usize {
    ducks.iter().enumerate().map(|(i, n)| (i + 1) * n).sum()
}

fn phase_one(ducks: &mut [usize]) -> bool {
    let mut has_modified = false;
    // Magic needed to get mutable windows
    let slice_of_cells: &[cell::Cell<_>] = cell::Cell::from_mut(ducks).as_slice_of_cells();
    for ab in slice_of_cells.windows(2) {
        if ab[1] < ab[0] {
            ab[0].replace(ab[0].get() - 1);
            ab[1].replace(ab[1].get() + 1);
            has_modified = true;
        }
    }
    has_modified
}

fn phase_two(ducks: &mut [usize]) -> bool {
    let mut has_modified = false;
    let slice_of_cells: &[cell::Cell<_>] = cell::Cell::from_mut(ducks).as_slice_of_cells();
    for ab in slice_of_cells.windows(2) {
        if ab[1] > ab[0] {
            ab[0].replace(ab[0].get() + 1);
            ab[1].replace(ab[1].get() - 1);
            has_modified = true;
        }
    }
    has_modified
}

fn play_round(ducks: &mut [usize], round_limit: usize) -> usize {
    let mut round_num = 0;
    while round_num < round_limit && phase_one(ducks) {
        round_num += 1;
    }

    while round_num < round_limit && phase_two(ducks) {
        round_num += 1;
    }
    round_num
}

// Solution by /u/runarmod
// Key insight 1 for part 3 is that the input data numbers have ascending order,
// so we can skip right to phase 2.
// Key insight 2 is that each round in a phase effectively only moves one duck
// from one column to another.
// Key insight 3 is that in phase 2, when a column get the number of ducks each column ends with,
// it will never get more/less (it won't fluctuate).
// In other words, the number of ducks per column in phase 2 is either monotonically increasing
// or monotonically decreasing.
// Thus, the solution is to sum up the differences from the mean number of ducks per column for
// each column with number of ducks less than the mean (as each round will bring each of these
// columns one step closer to the mean).
fn phase_two_all(ducks: &[usize]) -> usize {
    let mean: usize = ducks.iter().sum::<usize>() / ducks.len();
    ducks.iter().filter(|d| **d < mean).map(|d| mean - d).sum()
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let mut p1_input: Vec<_> = fs::read_to_string(p1_input_filename)?
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    play_round(&mut p1_input, 10);
    let p1_checksum = flock_checksum(&p1_input);

    let mut p2_input: Vec<_> = fs::read_to_string(p2_input_filename)?
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();
    let p2_round_count = play_round(&mut p2_input, usize::MAX);

    let p3_input: Vec<_> = fs::read_to_string(p3_input_filename)?
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();
    let p3_round_count = phase_two_all(&p3_input);

    println!("P1: Flock checksum after 10 rounds: {p1_checksum}");
    println!("P2: Flock equalises after {p2_round_count} rounds");
    println!("P3: Flock equalises after {p3_round_count} rounds");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let mut input = [9, 1, 1, 4, 9, 6];
        phase_one(&mut input);
        assert_eq!(input, [8, 1, 2, 4, 8, 7]);
        phase_one(&mut input);
        phase_one(&mut input);
        phase_one(&mut input);
        phase_one(&mut input);
        phase_one(&mut input);
        assert_eq!(input, [3, 4, 4, 4, 7, 8]);
        assert_eq!(flock_checksum(&input), 122);
    }

    #[test]
    fn ex1b() {
        let mut input = [9, 1, 1, 4, 9, 6];
        play_round(&mut input, 10);
        assert_eq!(flock_checksum(&input), 109);
    }

    #[test]
    fn ex2() {
        let mut input = [805, 706, 179, 48, 158, 150, 232, 885, 598, 524, 423];
        let round_count = play_round(&mut input, usize::MAX);
        assert_eq!(round_count, 1579);
    }
}
