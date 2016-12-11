use std::fs::File;
use std::env;
use std::io::{BufReader, Read};

fn decompress_len(input: &String, recurse: bool) -> usize {
    let mut length = 0;
    let mut it = input.chars();
    while let Some(c) = it.next() {
        if c == '(' {
            let ss_length: usize =
                it.by_ref().take_while(|&c| c != 'x').collect::<String>().parse().unwrap();
            let repeat_count: usize =
                it.by_ref().take_while(|&c| c != ')').collect::<String>().parse().unwrap();
            let substr: String = it.by_ref().take(ss_length).collect();
            length += if recurse {
                decompress_len(&substr, recurse) * repeat_count
            } else {
                ss_length * repeat_count
            }
        } else if !c.is_whitespace() {
            length += 1;
        }
    }
    return length;
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }
    let mut input = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap());

    let mut initial = "".to_string();
    let _ = input.read_to_string(&mut initial);

    println!("Initial decompressed length: {}",
             decompress_len(&initial, false));

    println!("Fully decompressed length: {}",
             decompress_len(&initial, true));
}
