use std::char;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

fn main() {
    let input_signal: Vec<_> = BufReader::new(
        File::open(
            &env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .next()
    .unwrap()
    .unwrap()
    .chars()
    .map(|c| c.to_digit(10).unwrap() as i32)
    .collect();

    let phase_100 = (0..100).fold(input_signal.clone(), |signal, _| next_phase(&signal));
    let phase_100_first8: String = phase_100
        .iter()
        .take(8)
        .map(|d| char::from_digit(*d as u32, 10).unwrap())
        .collect();
    println!(
        "After 100 iterations of FFT, first 8 digits: {}",
        phase_100_first8
    );

    let message_offset = input_signal.iter().take(7).fold(0, |acc, x| acc * 10 + x);
    let repeated_signal: Vec<i32> = iter::repeat(input_signal)
        .take(10000)
        .flat_map(|v| v.into_iter())
        .collect();
    let phase_100 = (0..100).fold(repeated_signal.clone(), |signal, _| next_phase(&signal));
    let phase_100_first8: String = phase_100
        .iter()
        .skip(message_offset as usize)
        .take(8)
        .map(|d| char::from_digit(*d as u32, 10).unwrap())
        .collect();

    println!("Message of repeated signal: {}", phase_100_first8);
}

fn next_phase(signal: &Vec<i32>) -> Vec<i32> {
    (0..signal.len())
        .map(|i| {
            signal
                .iter()
                .zip(get_sequence(i))
                .map(|(e, c)| e * c)
                .sum::<i32>()
                .abs()
                % 10
        })
        .collect()
}

fn get_sequence(idx: usize) -> impl Iterator<Item = i32> {
    let base_pattern = [0, 1, 0, -1];

    iter::repeat(base_pattern[0])
        .take(idx + 1)
        .chain(iter::repeat(base_pattern[1]).take(idx + 1))
        .chain(iter::repeat(base_pattern[2]).take(idx + 1))
        .chain(iter::repeat(base_pattern[3]).take(idx + 1))
        .cycle()
        .skip(1)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sequence_base() {
        let seq: Vec<_> = get_sequence(0).take(8).collect();
        assert_eq!(seq, &[1, 0, -1, 0, 1, 0, -1, 0]);
    }

    #[test]
    fn sequence_repeat() {
        let seq: Vec<_> = get_sequence(1).take(8).collect();
        assert_eq!(seq, &[0, 1, 1, 0, 0, -1, -1, 0]);
    }

    #[test]
    fn next_phase_base() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(next_phase(&input), &[4, 8, 2, 2, 6, 1, 5, 8]);
    }
}
