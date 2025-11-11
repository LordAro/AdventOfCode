use std::fs;
use std::io;

fn count_available_mentors(l: &str) -> (usize, usize, usize) {
    let mut seen_sword = 0;
    let mut seen_archery = 0;
    let mut seen_magic = 0;

    let mut total_sword = 0;
    let mut total_archery = 0;
    let mut total_magic = 0;

    for c in l.chars() {
        match c {
            'A' => seen_sword += 1,
            'B' => seen_archery += 1,
            'C' => seen_magic += 1,
            'a' => total_sword += seen_sword,
            'b' => total_archery += seen_archery,
            'c' => total_magic += seen_magic,
            _ => unreachable!(),
        }
    }
    (total_sword, total_archery, total_magic)
}

fn count_visible_mentors<const DIST_LIMIT: usize>(l: &str) -> usize {
    let layout = l.as_bytes();

    let mut visible_sword = 0;
    let mut visible_archery = 0;
    let mut visible_magic = 0;

    let mut leading_idx = 0;
    let mut trailing_idx = 0;

    while leading_idx < DIST_LIMIT {
        match layout[leading_idx] {
            b'A' => visible_sword += 1,
            b'B' => visible_archery += 1,
            b'C' => visible_magic += 1,
            _ => {}
        }

        leading_idx += 1;
    }

    let mut total = 0;

    for (idx, c) in layout.iter().enumerate() {
        if idx > DIST_LIMIT {
            match layout[trailing_idx] {
                b'A' => visible_sword -= 1,
                b'B' => visible_archery -= 1,
                b'C' => visible_magic -= 1,
                _ => {}
            }
            trailing_idx += 1;
        }

        if idx < layout.len() - DIST_LIMIT {
            match layout[leading_idx] {
                b'A' => visible_sword += 1,
                b'B' => visible_archery += 1,
                b'C' => visible_magic += 1,
                _ => {}
            }
            leading_idx += 1;
        }

        match c {
            b'a' => total += visible_sword,
            b'b' => total += visible_archery,
            b'c' => total += visible_magic,
            _ => {}
        }
    }
    total
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_input = fs::read_to_string(p1_input_filename)?;
    let (p1_total_sword, _, _) = count_available_mentors(&p1_input);

    let p2_input = fs::read_to_string(p2_input_filename)?;
    let (p2_total_sword, p2_total_archery, p2_total_magic) = count_available_mentors(&p2_input);
    let p2_total = p2_total_sword + p2_total_archery + p2_total_magic;

    let p3_input = fs::read_to_string(p3_input_filename)?.repeat(1000);
    let p3_total = count_visible_mentors::<1000>(&p3_input);

    println!("P1: Total number of sword fighting pairs: {p1_total_sword}");
    println!("P2: Total number of avilable pairs: {p2_total}");
    println!("P3: Total number of visible pairs: {p3_total}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex3a() {
        let input = "AABCBABCABCabcabcABCCBAACBCa";
        let total = count_visible_mentors::<10>(input);
        assert_eq!(total, 34);
    }

    #[test]
    fn ex3b() {
        let input = "AABCBABCABCabcabcABCCBAACBCa".repeat(2);
        let total = count_visible_mentors::<10>(&input);
        assert_eq!(total, 72);
    }

    #[test]
    fn ex3c() {
        let input = "AABCBABCABCabcabcABCCBAACBCa".repeat(1000);
        let total = count_visible_mentors::<1000>(&input);
        assert_eq!(total, 3442321);
    }
}
