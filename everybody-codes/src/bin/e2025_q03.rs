use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let mut p1_input: Vec<_> = fs::read_to_string(p1_input_filename)?
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    p1_input.sort();
    p1_input.dedup();
    let p1_total: usize = p1_input.iter().sum();

    let mut p2_input: Vec<_> = fs::read_to_string(p2_input_filename)?
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    p2_input.sort();
    p2_input.dedup();
    let p2_total: usize = p2_input.iter().take(20).sum();

    let mut p3_input: Vec<_> = fs::read_to_string(p3_input_filename)?
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    p3_input.sort();
    let mut num_sets = 0;
    while !p3_input.is_empty() {
        let mut prev = 0;
        // don't actually need the result
        let _: Vec<_> = p3_input
            .extract_if(.., |x| {
                if *x > prev {
                    prev = *x;
                    return true;
                }
                false
            })
            .collect();
        num_sets += 1;
    }

    println!("P1: Maximum packing size: {p1_total}");
    println!("P2: Minimum packing size of 20: {p2_total}");
    println!("P3: Minimum number of sets: {num_sets}");
    Ok(())
}
