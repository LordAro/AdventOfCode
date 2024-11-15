use std::fs;
use std::io;

fn get_required_hammer_hits(input_str: &str) -> usize {
    let nails: Vec<usize> = input_str
        .lines()
        .map(|l| l.parse().expect("Should be numeric"))
        .collect();
    let shortest_nail = nails.iter().min().expect("List not empty");
    nails.iter().map(|n| n - shortest_nail).sum()
}

fn get_required_hammer_hits_avg(input_str: &str) -> isize {
    let mut nails: Vec<usize> = input_str
        .lines()
        .map(|l| l.parse().expect("Should be numeric"))
        .collect();
    nails.sort();
    let median_nail = nails[nails.len() / 2];

    nails
        .iter()
        .map(|n| (*n as isize - median_nail as isize).abs())
        .sum()
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_required_hammer_hits = get_required_hammer_hits(&fs::read_to_string(p1_input_filename)?);
    println!("P1: Required hammer hits: {p1_required_hammer_hits}");
    let p2_required_hammer_hits = get_required_hammer_hits(&fs::read_to_string(p2_input_filename)?);
    println!("P2: Required hammer hits: {p2_required_hammer_hits}");
    let p3_required_hammer_hits =
        get_required_hammer_hits_avg(&fs::read_to_string(p3_input_filename)?);
    println!("P3: Required hammer hits: {p3_required_hammer_hits}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input_str = "3\n4\n7\n8";
        assert_eq!(get_required_hammer_hits(input_str), 10);
    }

    #[test]
    fn ex3() {
        let input_str = "2\n4\n5\n6\n8";
        assert_eq!(get_required_hammer_hits_avg(input_str), 8);
    }
}
