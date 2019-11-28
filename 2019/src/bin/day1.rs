use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided");
    }
    let mut input = File::open(&env::args().nth(1).unwrap()).expect("Could not open input file");

    let mut contents = String::new();
    input.read_to_string(&mut contents)?;
    println!("Input str: {}", contents);
    Ok(())
}
