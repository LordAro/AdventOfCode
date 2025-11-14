use std::collections::HashSet;
use std::fs;
use std::io;

fn count_crossings<const NUM_NAILS: usize>(input: &[usize]) -> usize {
    let mut total_crossings = 0;
    let mut seen_crossings: Vec<(usize, usize)> = vec![];
    for ab in input.windows(2) {
        // 1-based to 0-based
        let a = (ab[0] - 1).min(ab[1] - 1);
        let b = (ab[1] - 1).max(ab[0] - 1);

        // work out which direction to look for crossings
        // (there's got to be a better way of doing this?)
        let side1: HashSet<_> = (a + 1..b).collect();
        let side2: HashSet<_> = (b + 1..a + NUM_NAILS).map(|x| x % NUM_NAILS).collect();

        // For every line that we've already placed
        let new_crossings = seen_crossings
            .iter()
            // Remove lines that start/end at one of our points - can't cross
            .filter(|(c, d)| a != *c && b != *c && a != *d && b != *d)
            // Remove lines that are exclusive on one side or the other and don't cross over
            .filter(|(c, d)| !(side1.contains(c) && side1.contains(d)))
            .filter(|(c, d)| !(side2.contains(c) && side2.contains(d)))
            .count();
        total_crossings += new_crossings;

        seen_crossings.push((a, b));
    }
    total_crossings
}

fn count_centre_crossings<const NUM_NAILS: usize>(input: &[usize]) -> usize {
    input
        .windows(2)
        .filter(|ab| {
            let a = ab[0];
            let b = ab[1];
            (a + NUM_NAILS / 2) % NUM_NAILS == b % NUM_NAILS
        })
        .count()
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, _p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_input: Vec<_> = fs::read_to_string(p1_input_filename)?
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let p1_num_centre_passes = count_centre_crossings::<32>(&p1_input);

    let p2_input: Vec<_> = fs::read_to_string(p2_input_filename)?
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let p2_num_crossings = count_crossings::<256>(&p2_input);

    println!("P1: Number of centre passes: {p1_num_centre_passes}");
    println!("P2: Number of knots required: {p2_num_crossings}");
    println!("P3: ");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input = [1, 5, 2, 6, 8, 4, 1, 7, 3];
        assert_eq!(count_centre_crossings::<8>(&input), 4);
    }

    #[test]
    fn ex2() {
        let input = [1, 5, 2, 6, 8, 4, 1, 7, 3, 5, 7, 8, 2];
        let num_crossings = count_crossings::<8>(&input);
        assert_eq!(num_crossings, 21);
    }
}
