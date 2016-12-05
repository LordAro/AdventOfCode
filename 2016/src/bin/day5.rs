extern crate crypto;

use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead};
use crypto::md5::Md5;
use crypto::digest::Digest;

fn to_hex_digit(i: u8) -> char {
    match i {
        0...9 => (i + ('0' as u8)) as char,
        10...15 => (i - 10 + ('a' as u8)) as char,
        _ => unreachable!(),
    }
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    let mut input = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap());

    let mut keystr = String::new();
    let _ = input.read_line(&mut keystr);
    let key = keystr.trim().as_bytes();

    let mut password = String::new();
    let mut password2 = String::from("........"); // "NaN"
    let mut hasher = Md5::new();
    for i in 0..std::u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);
        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
        if first_five == 0 {
            let sixth = (output[2] & 0xFF) as u8;
            // 1st password
            if password.len() < 8 {
                password.push(to_hex_digit(sixth));
            }

            // 2nd password
            let seventh = (output[3] >> 4) as u8;
            if sixth < 8 {
                // Surprisingly difficult to replace a character in a string
                password2 = password2.chars()
                    .enumerate()
                    .map(|(i, c)| if i == sixth as usize && c == '.' {
                        to_hex_digit(seventh)
                    } else {
                        c
                    })
                    .collect();
            }

            if password.len() == 8 && !password2.contains('.') {
                break;
            }
        }
        hasher.reset();
    }
    println!("Password: {}", password);
    println!("Second password: {}", password2);
}
