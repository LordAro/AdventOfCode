use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

/* Shiny iterator function */
fn fuel_reqs(mass: i32) -> impl std::iter::Iterator<Item = i32> {
    let mut m = mass;
    std::iter::from_fn(move || {
        m = m / 3 - 2;
        if m > 0 {
            Some(m)
        } else {
            None
        }
    })
}

fn main() -> io::Result<()> {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided");
    }
    let modules: Vec<_> =
        BufReader::new(File::open(env::args().nth(1).unwrap()).expect("Could not open input file"))
            .lines()
            .map(|l| l.unwrap().parse().unwrap())
            .collect();

    let fuel_sum: i32 = modules.iter().map(|n| fuel_reqs(*n).next().unwrap()).sum();
    let rec_fuel_sum: i32 = modules.iter().map(|n| fuel_reqs(*n).sum::<i32>()).sum();
    println!("Fuel requirements: {}", fuel_sum);
    println!("Fuel requirements (inc fuel): {}", rec_fuel_sum);
    Ok(())
}
