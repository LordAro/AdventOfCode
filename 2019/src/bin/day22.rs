use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input_str: String =
        fs::read_to_string(env::args().nth(1).expect("Incorrect number of arguments"))?;

    let deck_size: isize = 10007;
    let mut deck: Vec<_> = (0..deck_size).collect();

    input_str.lines().for_each(|instr| {
        if instr.starts_with("deal with") {
            let incr_n = instr["deal with increment ".len()..]
                .parse::<usize>()
                .unwrap();
            let mut new_deck = vec![0; deck_size as usize];
            deck.iter().enumerate().for_each(|(i, &n)| {
                new_deck[(i * incr_n) % deck_size as usize] = n;
            });
            deck = new_deck;
        } else if instr.starts_with("cut") {
            let cut_n = ((instr[4..].parse::<isize>().unwrap() + deck_size) % deck_size) as usize;
            deck = [&deck[cut_n..], &deck[..cut_n]].concat();
        } else if instr.starts_with("deal into") {
            deck.reverse();
        } else {
            panic!("Unknown instruction: {}", instr);
        }
    });

    let relevant_index = deck
        .iter()
        .enumerate()
        .find(|&(_, &n)| n == 2019)
        .unwrap()
        .0;

    println!("Position of 2019th card: {}", relevant_index);

    Ok(())
}
