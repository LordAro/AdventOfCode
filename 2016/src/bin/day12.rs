use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_reg(word: &str) -> Option<char> {
    word.chars().nth(0)
}

fn run_prog(registers: &mut HashMap<char, i32>, prog: &Vec<Vec<&str>>) {
    let mut pc = 0;
    while pc < prog.len() {
        let ins = &prog[pc];
        match ins[0] {
            "cpy" => {
                let reg = parse_reg(ins[2]).unwrap();
                if let Ok(val) = ins[1].parse() {
                    registers.insert(reg, val);
                } else {
                    let val = *registers.get(&parse_reg(ins[1]).unwrap()).unwrap();
                    registers.insert(reg, val);
                }
            }
            "inc" | "dec" => {
                let reg = parse_reg(ins[1]).unwrap();
                *registers.get_mut(&reg).unwrap() += if ins[0] == "inc" { 1 } else { -1 };
            }
            "jnz" => {
                let val: i32 = ins[2].parse().unwrap();
                if let Ok(reg) = ins[1].parse::<i32>() {
                    if reg != 0 {
                        pc = (pc as i32 + (val - 1)) as usize; // -1 because of increment at end of loop
                    }
                } else {
                    let reg = parse_reg(ins[1]).unwrap();
                    if *registers.get(&reg).unwrap() != 0 {
                        pc = (pc as i32 + (val - 1)) as usize; // -1 because of increment at end of loop
                    }
                }
            }
            _ => panic!("Unrecognised instruction: {}", ins[0]),
        }
        pc += 1;
    }
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }
    let input: Vec<_> = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let prog: Vec<Vec<_>> = input
        .iter()
        .map(|l| l.split_whitespace().collect())
        .collect();

    let mut registers: HashMap<char, i32> = HashMap::new();
    registers.insert('a', 0);
    registers.insert('b', 0);
    registers.insert('c', 0);
    registers.insert('d', 0);
    run_prog(&mut registers, &prog);
    println!("Value of register a: {}", *registers.get(&'a').unwrap());

    registers.insert('a', 0);
    registers.insert('b', 0);
    registers.insert('c', 1);
    registers.insert('d', 0);
    run_prog(&mut registers, &prog);
    println!("Value of register a: {}", *registers.get(&'a').unwrap());
}
