use std::cell;
use std::fs;
use std::io;

fn flock_checksum(ducks: &[usize]) -> usize {
    ducks.iter().enumerate().map(|(i, n)| (i + 1) * n).sum()
}

fn swap_ducks_lt(ducks: &mut [usize]) -> bool {
    let mut has_modified = false;
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

fn swap_ducks_gt(ducks: &mut [usize]) -> bool {
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
    while round_num < round_limit && swap_ducks_lt(ducks) {
        round_num += 1;
    }

    while round_num < round_limit && swap_ducks_gt(ducks) {
        round_num += 1;
    }
    round_num
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

    let mut p3_input: Vec<_> = fs::read_to_string(p3_input_filename)?
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();
    let p3_round_count = play_round(&mut p3_input, usize::MAX);

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
        swap_ducks_lt(&mut input);
        assert_eq!(input, [8, 1, 2, 4, 8, 7]);
        swap_ducks_lt(&mut input);
        swap_ducks_lt(&mut input);
        swap_ducks_lt(&mut input);
        swap_ducks_lt(&mut input);
        swap_ducks_lt(&mut input);
        assert_eq!(input, [3, 4, 4, 4, 7, 8]);
        assert_eq!(flock_checksum(&input), 122);
    }

    #[test]
    fn ex1b() {
        let mut input = [9, 1, 1, 4, 9, 6];
        play_round(&mut input, 10);
        assert_eq!(flock_checksum(&input), 109);
    }
}
