use std::env;
use std::fs;
use std::io;
use memoise::memoise_map;

// todo: something that does not require vecs
fn blink(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }
    let num_digits = stone.ilog10() + 1;
    if num_digits % 2 == 0 {
        return vec![
            stone / 10_u32.pow(num_digits / 2) as u64,
            stone % 10_u32.pow(num_digits / 2) as u64,
        ];
    }
    vec![stone * 2024]
}

#[memoise_map(stone, remaining_blinks)]
fn stone_count_after_n_blinks(stone: u64, remaining_blinks: u64) -> usize {
    if remaining_blinks == 0 {
        return 1;
    }
    blink(stone).iter().map(|s| stone_count_after_n_blinks(*s, remaining_blinks - 1)).sum()
}

fn blink_stones(stones: &[u64], blinks: u64) -> usize {
    stones.iter().map(|s| stone_count_after_n_blinks(*s, blinks)).sum()
}

fn main() -> io::Result<()> {
    let input: Vec<u64> = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?
        .split(" ")
        .map(|n| n.trim().parse().unwrap())
        .collect();

    println!("P1: Number of stones after blinking 25 times: {}", blink_stones(&input, 25));
    println!("P2: Number of stones after blinking 75 times: {}", blink_stones(&input, 75));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        assert_eq!(blink(0), [1]);
        assert_eq!(blink(1), [2024]);
        assert_eq!(blink(10), [1, 0]);
        assert_eq!(blink(99), [9, 9]);
        assert_eq!(blink(999), [2021976]);
    }

    #[test]
    fn ex1b() {
        assert_eq!(blink_stones(&[0, 1, 10, 99, 999], 1), 7);
        assert_eq!(blink_stones(&[125, 17], 6), 22);
    }
}
