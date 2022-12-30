extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;
use std::char;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_triple(arr: &[u8; 32]) -> Option<char> {
    arr.windows(3)
        .find(|t| t[0] == t[1] && t[1] == t[2])
        .map(|t| char::from_digit(t[0] as u32, 16).unwrap())
}

fn get_quintuple(arr: &[u8; 32]) -> Option<char> {
    arr.windows(5)
        .find(|t| t[0] == t[1] && t[1] == t[2] && t[2] == t[3] && t[3] == t[4])
        .map(|t| char::from_digit(t[0] as u32, 16).unwrap())
}

fn get_new_keys(idx: u64, arr: &[u8; 32], existing_triples: &mut Vec<(u64, char)>) -> Vec<u64> {
    if let Some(c) = get_quintuple(arr) {
        let matches = existing_triples
            .iter()
            .filter(|&&(ti, tc)| tc == c && ti < idx && idx < ti + 1000)
            .map(|&(ti, _)| ti)
            .collect();
        // Invert the above condition so as to stop keys from being picked more than once
        existing_triples.retain(|&(ti, tc)| !(tc == c && ti < idx && idx < ti + 1000));
        return matches;
    }
    vec![]
}

fn split_arr(arr: &[u8; 16]) -> [u8; 32] {
    let mut out = [0; 32];
    for i in 0..16 {
        // There's definitely a better way of doing this
        out[2 * i] = arr[i] >> 4;
        out[2 * i + 1] = arr[i] & 0xF;
    }
    out
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    let input = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap();
    let key = input.as_bytes();

    // Lots of doubled stuff because of both parts
    let mut triples: Vec<(u64, char)> = vec![];
    let mut triples2016: Vec<(u64, char)> = vec![];
    let mut valid_indexes = vec![];
    let mut valid_indexes2016 = vec![];

    let mut hasher = Md5::new();
    for i in 0..std::u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);
        for _ in 0..2016 {
            let out_str = hasher.result_str();
            hasher.reset();
            hasher.input(out_str.as_bytes());
        }
        let mut output2016 = [0; 16];
        hasher.result(&mut output2016);

        let output_split = split_arr(&output);
        let output_split2016 = split_arr(&output2016);

        if let Some(c) = get_triple(&output_split) {
            triples.push((i, c));
        }
        if let Some(c) = get_triple(&output_split2016) {
            triples2016.push((i, c));
        }

        valid_indexes.extend(get_new_keys(i, &output_split, &mut triples));
        valid_indexes2016.extend(get_new_keys(i, &output_split2016, &mut triples2016));

        hasher.reset();
        if valid_indexes.len() >= 64 && valid_indexes2016.len() >= 64 {
            break;
        }
    }
    println!("64th pad key index: {}", valid_indexes[63]);
    println!("Actual 64th pad key index: {}", valid_indexes2016[63]);
}
