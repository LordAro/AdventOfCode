use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_input: Vec<_> = fs::read_to_string(p1_input_filename)?
        .lines()
        .map(|l| l.parse::<f64>().unwrap())
        .collect();
    let mut num_turns = 2025.;
    for win in p1_input.windows(2) {
        num_turns *= win[0] / win[1];
    }

    let p2_input: Vec<_> = fs::read_to_string(p2_input_filename)?
        .lines()
        .map(|l| l.parse::<f64>().unwrap())
        .collect();
    let mut target: f64 = 10_000_000_000_000.;
    for win in p2_input.windows(2) {
        target *= win[1] / win[0];
    }

    let p3_input: Vec<_> = fs::read_to_string(p3_input_filename)?
        .lines()
        .map(|l| {
            let ns: Vec<_> = l.split('|').map(|n| n.parse::<f64>().unwrap()).collect();
            if ns.len() == 1 {
                (ns[0], ns[0])
            } else {
                (ns[0], ns[1])
            }
        })
        .collect();

    let mut p3_num_turns: f64 = 100.;
    for win in p3_input.windows(2) {
        p3_num_turns *= win[0].1 / win[1].0;
    }

    println!("P1: Number of final gear turns: {}", num_turns.floor());
    println!("P2: Number of first gear turns: {}", target.ceil());
    println!("P3: Number of final gear turns: {}", p3_num_turns.floor());
    Ok(())
}
