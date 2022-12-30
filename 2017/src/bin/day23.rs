use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Machine<'a> {
    pc: i64,
    program: &'a Vec<(String, char, String)>,
    regs: HashMap<char, i64>,
}

impl<'a> Machine<'a> {
    fn new(program: &'a Vec<(String, char, String)>) -> Machine<'a> {
        Machine {
            pc: 0,
            program,
            regs: HashMap::new(),
        }
    }

    fn get_val(&mut self, val: &str) -> i64 {
        val.parse()
            .unwrap_or(*self.regs.entry(val.chars().next().unwrap()).or_insert(0))
    }

    fn get_val_c(&mut self, val: char) -> i64 {
        if val.is_ascii_digit() {
            val.to_digit(10).unwrap() as i64
        } else {
            *self.regs.entry(val).or_insert(0)
        }
    }

    fn run(&mut self) -> usize {
        let mut mul_count = 0;
        while self.pc >= 0 && (self.pc as usize) < self.program.len() {
            let ins = &self.program[self.pc as usize];
            let val = match ins.0.as_ref() {
                "jnz" => self.get_val_c(ins.1),
                _ => self.get_val(&ins.2),
            };
            match ins.0.as_ref() {
                "set" => {
                    self.regs.insert(ins.1, val);
                }
                "sub" => *self.regs.entry(ins.1).or_insert(0) -= val,
                "mul" => {
                    *self.regs.entry(ins.1).or_insert(0) *= val;
                    mul_count += 1;
                }
                "jnz" => {
                    if val != 0 {
                        self.pc += self.get_val(&ins.2) - 1;
                    }
                }
                _ => unreachable!(),
            }
            self.pc += 1;
        }
        mul_count
    }
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    let input: Vec<(String, char, String)> =
        BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
            .lines()
            .map(|l| {
                let v: Vec<_> = l
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                (
                    v[0].clone(),
                    v[1].chars().next().unwrap(),
                    if v.len() == 3 {
                        v[2].clone()
                    } else {
                        "".to_string()
                    },
                )
            })
            .collect();

    let mut machine1 = Machine::new(&input);
    let mul_count = machine1.run();
    println!("Number of mul instructions: {}", mul_count);

    // Program finds primes of the form b*100+10000 + kP,
    // with b=79, k=[0..1000] & P = 17
    let mut h = 0;

    let mut b = input[0].2.parse::<usize>().unwrap();
    b = (b * 100) + 100_000;
    let c = b + 17_000;
    while b <= c {
        for d in 2..b {
            if b % d == 0 {
                h += 1;
                break;
            }
        }
        b += 17;
    }

    println!("Value in h: {}", h);
}
