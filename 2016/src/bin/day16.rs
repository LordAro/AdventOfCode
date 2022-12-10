use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn data_to_string(a: &[bool]) -> String {
    return a.iter().map(|&b| if b { '1' } else { '0' }).collect();
}

fn toggled(a: &[bool]) -> Vec<bool> {
    a.iter().map(|b| !b).collect()
}

fn extend_dragon_curve(a: &mut Vec<bool>) {
    let toggled = toggled(a);
    a.push(false); // 0
    a.extend(toggled.iter().rev());
}

fn calculate_checksum(a: &[bool], len: usize) -> Vec<bool> {
    let checksum = a
        .chunks(2)
        .take(len / 2)
        .map(|c| c[0] == c[1])
        .collect::<Vec<_>>();
    if checksum.len() % 2 == 0 {
        return calculate_checksum(&checksum, checksum.len());
    }
    checksum
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }
    let input_str = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap())
        .lines()
        .next() // isn't
        .unwrap() // rust
        .unwrap(); // fun?

    let input = input_str.chars().map(|c| c == '1').collect::<Vec<_>>();

    let disk_len1 = 272;
    let disk_len2 = 35651584;

    let mut output_data = input;
    while output_data.len() < disk_len2 {
        extend_dragon_curve(&mut output_data);
    }

    let checksum1 = calculate_checksum(&output_data, disk_len1);
    let checksum2 = calculate_checksum(&output_data, disk_len2);

    println!(
        "Data checksum with disk len {}: {}",
        disk_len1,
        data_to_string(&checksum1)
    );
    println!(
        "Data checksum with disk len {}: {}",
        disk_len2,
        data_to_string(&checksum2)
    );
}
