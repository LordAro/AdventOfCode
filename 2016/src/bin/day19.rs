use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn josephus2(n: usize) -> usize {
    if n == 1 {
        1
    } else if n % 2 == 0 {
        2 * josephus2(n / 2) - 1
    } else {
        2 * josephus2(n / 2) + 1 // f(2j+1), but we're using integer division
    }
}

fn josephus_opposite(n: usize) -> usize {
    let mut pow3 = 1;
    // get biggest 3**m less than n
    while pow3 * 3 < n {
        pow3 *= 3;
    }

    if n == pow3 * 3 {
        pow3 * 3
    } else if n <= pow3 * 2 {
        n - pow3
    } else {
        // 2(n - pow3) - pow3
        2 * n - 3 * pow3
    }
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    let num_elves = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    println!(
        "Elf with all the presents (stealing next): {}",
        josephus2(num_elves),
    );

    println!(
        "Elf with all the presents (stealing opposite): {}",
        josephus_opposite(num_elves)
    );
}
