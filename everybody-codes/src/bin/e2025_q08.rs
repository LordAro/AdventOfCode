use std::collections::HashSet;
use std::fs;
use std::io;

fn count_crossings_for_point<const NUM_NAILS: usize>(
    existing_crossings: &[(usize, usize)],
    new_line: (usize, usize),
) -> usize {
    let (a, b) = new_line;
    // work out which direction to look for crossings
    // (there's got to be a better way of doing this?)
    let side1: HashSet<_> = (a + 1..b).collect();
    let side2: HashSet<_> = (b + 1..a + NUM_NAILS).map(|x| x % NUM_NAILS).collect();

    // For every line that we've already placed
    existing_crossings
        .iter()
        // Remove lines that start/end at one of our points - can't cross
        .filter(|(c, d)| a != *c && b != *c && a != *d && b != *d)
        // Remove lines that are exclusive on one side or the other and don't cross over
        .filter(|(c, d)| !(side1.contains(c) && side1.contains(d)))
        .filter(|(c, d)| !(side2.contains(c) && side2.contains(d)))
        .count()
}

fn generate_crossings<const NUM_NAILS: usize>(input: &[usize]) -> (usize, Vec<(usize, usize)>) {
    let mut total_crossings = 0;
    let mut seen_crossings: Vec<(usize, usize)> = vec![];
    for ab in input.windows(2) {
        // 1-based to 0-based
        let a = (ab[0] - 1).min(ab[1] - 1);
        let b = (ab[1] - 1).max(ab[0] - 1);

        let new_crossings = count_crossings_for_point::<NUM_NAILS>(&seen_crossings, (a, b));
        total_crossings += new_crossings;

        seen_crossings.push((a, b));
    }
    (total_crossings, seen_crossings)
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
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
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
    let (p2_num_knots, _) = generate_crossings::<256>(&p2_input);

    let p3_input: Vec<_> = fs::read_to_string(p3_input_filename)?
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let (_, p3_crossings) = generate_crossings::<256>(&p3_input);
    let p3_max_num_cuts = (0..254)
        // skip +1 as that can't create cuts
        .flat_map(|a| (a + 2..256).map(move |b| (a, b)))
        .map(|(a, b)| {
            // number of lines we cross, and those that are the same point
            count_crossings_for_point::<256>(&p3_crossings, (a, b))
                + p3_crossings
                    .iter()
                    .filter(|(c, d)| a == *c && b == *d)
                    .count()
        })
        .max()
        .unwrap();

    println!("P1: Number of centre passes: {p1_num_centre_passes}");
    println!("P2: Number of knots required: {p2_num_knots}");
    println!("P3: Maximum number of threads that can be cut: {p3_max_num_cuts}");
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
        let (num_crossings, _) = generate_crossings::<8>(&input);
        assert_eq!(num_crossings, 21);
    }

    #[test]
    fn ex3() {
        let input = [1, 5, 2, 6, 8, 4, 1, 7, 3, 6];
        let (_, crossings) = generate_crossings::<8>(&input);
        let max_num_cuts = (0..6)
            .flat_map(|a| (a + 2..8).map(move |b| (a, b)))
            .map(|(a, b)| {
                count_crossings_for_point::<8>(&crossings, (a, b))
                    + crossings.iter().filter(|(c, d)| a == *c && b == *d).count()
            })
            .max()
            .unwrap();
        assert_eq!(max_num_cuts, 7);
    }
}
