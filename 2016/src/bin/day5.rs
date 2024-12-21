extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;
use std::char;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    let mut input = BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap());

    let mut keystr = String::new();
    let _ = input.read_line(&mut keystr);
    let key = keystr.trim().as_bytes();

    let mut password = String::new();
    let mut password2 = String::from("........"); // "NaN"
    let mut hasher = Md5::new();
    for i in 0..u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);
        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
        if first_five == 0 {
            let sixth = output[2];
            // 1st password
            if password.len() < 8 {
                password.push(char::from_digit(sixth as u32, 16).unwrap());
            }

            // 2nd password
            let seventh = output[3] >> 4;
            if sixth < 8 {
                // Surprisingly difficult to replace a character in a string
                password2 = password2
                    .chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if i == sixth as usize && c == '.' {
                            char::from_digit(seventh as u32, 16).unwrap()
                        } else {
                            c
                        }
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
