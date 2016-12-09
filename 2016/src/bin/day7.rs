extern crate itertools;

use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead};
use std::iter::FromIterator;
use itertools::{Itertools, Either};

fn contains_abba_pair(s: &str) -> bool {
    for i in 0..s.len() - 3 {
        let abba: (_, _, _, _) = s.chars().skip(i).take(4).next_tuple().unwrap();
        if abba.0 == abba.3 && abba.1 == abba.2 && abba.0 != abba.1 {
            return true;
        }
    }
    return false;
}

fn find_aba(v: &Vec<String>) -> Vec<String> {
    // ridiculous oneliner, but no mutable state!
    // gets length 3 "windows" of each String, filters the ones that are aba format, then converts
    // the remaining ones back into Strings
    v.iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .iter()
        .flat_map(|v| {
            v.windows(3)
                .filter(|v| v[0] == v[2])
                .map(|v| v.iter().cloned().collect::<String>())
        })
        .collect::<Vec<_>>()
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }
    let input = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap());

    let splitted_lines: Vec<(Vec<_>, Vec<_>)> = input.lines()
        .map(|l| l.unwrap().split(|c| c == '[' || c == ']').map(String::from).collect::<Vec<_>>())
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

    let valid_tls = splitted_lines.iter()
        .filter(|&&(ref sn, ref hn)| {
            sn.into_iter().any(|s| contains_abba_pair(&s)) &&
            hn.into_iter().all(|s| !contains_abba_pair(&s))
        })
        .count();
    let valid_ssl = splitted_lines.iter()
        .filter(|&&(ref sn, ref hn)| {
            for aba in find_aba(sn).iter() {
                let a = aba.chars().nth(0).unwrap();
                let b = aba.chars().nth(1).unwrap();
                let bab = String::from_iter(vec![b, a, b]);

                if hn.into_iter().any(|s| s.contains(&bab)) {
                    return true;
                }
            }
            return false;
        })
        .count();
    println!("IPs supporting TLS: {}", valid_tls);
    println!("IPs supporting SSL: {}", valid_ssl);
}
