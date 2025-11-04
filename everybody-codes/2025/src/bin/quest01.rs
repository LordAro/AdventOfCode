use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let (p1_input_filename, _p2_input_filename, _p3_input_filename) =
        everybody_codes::get_input_files()?;

    let _p1_input: Vec<_> = fs::read_to_string(p1_input_filename)?
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect();

    println!("P1: ");
    Ok(())
}
