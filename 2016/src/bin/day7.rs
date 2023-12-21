extern crate itertools;

use itertools::{Either, Itertools};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

fn contains_abba_pair(s: &str) -> bool {
    s.chars()
        .tuple_windows()
        .filter(|&(a, b, c, d)| a == d && b == c && a != b)
        .count()
        != 0
}

fn find_aba(v: &[String]) -> Vec<String> {
    v.iter()
        .flat_map(|s| s.chars().tuple_windows())
        .filter(|&(a, _, c)| a == c)
        .map(|(a, b, c)| [a, b, c].iter().cloned().collect())
        .collect()
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }
    let input = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap());

    let splitted_lines: Vec<(Vec<_>, Vec<_>)> = input
        .lines()
        .map(|l| {
            l.unwrap()
                .split(|c| c == '[' || c == ']')
                .map(String::from)
                .collect::<Vec<_>>()
        })
        .map(|v| {
            v.into_iter().enumerate().partition_map(|(i, s)| {
                if i % 2 == 0 {
                    Either::Left(s) // supernets
                } else {
                    Either::Right(s) // hypernets
                }
            })
        })
        .collect();

    let valid_tls = splitted_lines
        .iter()
        .filter(|&(sn, hn)| {
            sn.iter().any(|s| contains_abba_pair(s)) && hn.iter().all(|s| !contains_abba_pair(s))
        })
        .count();
    let valid_ssl = splitted_lines
        .iter()
        .filter(|&(sn, hn)| {
            for aba in find_aba(sn).iter() {
                let a = aba.chars().next().unwrap();
                let b = aba.chars().nth(1).unwrap();
                let bab = String::from_iter(vec![b, a, b]);

                if hn.iter().any(|s| s.contains(&bab)) {
                    return true;
                }
            }
            false
        })
        .count();
    println!("IPs supporting TLS: {}", valid_tls);
    println!("IPs supporting SSL: {}", valid_ssl);
}
