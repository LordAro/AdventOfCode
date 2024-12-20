use itertools::Itertools;
use std::env;
use std::fs;
use std::io;

fn run_program(init_a: i64, init_b: i64, init_c: i64, program: &[i64]) -> Vec<i64> {
    let mut a = init_a;
    let mut b = init_b;
    let mut c = init_c;
    let mut ip = 0;

    let mut output = vec![];

    while ip < program.len() {
        let op = program[ip];

        let literal = program[ip + 1];
        let combo = match literal {
            0..=3 => literal,
            4 => a,
            5 => b,
            6 => c,
            7 => unreachable!("Reserved"),
            _ => unreachable!("Unrecognised operand {literal}"),
        };
        // println!("ip={ip} a={a} b={b} c={c} op={op} opand={literal}/{combo}");
        match op {
            // adv
            0 => a /= 2_i64.pow(combo as u32),
            // bxl
            1 => b ^= literal,
            // bst
            2 => b = combo % 8,
            // jnz
            3 => {
                if a != 0 {
                    ip = literal as usize;
                    continue; // don't increment ip
                }
            }
            // bxc
            4 => b ^= c,
            // out
            5 => output.push(combo % 8),
            // bdv
            6 => b = a / 2_i64.pow(combo as u32),
            // cdv
            7 => c = a / 2_i64.pow(combo as u32),
            _ => unreachable!("Unknown opcode {op}"),
        }
        ip += 2;
    }
    output
}

fn parse_registers(s: &str) -> (i64, i64, i64) {
    let mut l = s.lines();
    (
        l.next().unwrap()[12..].parse().unwrap(),
        l.next().unwrap()[12..].parse().unwrap(),
        l.next().unwrap()[12..].parse().unwrap(),
    )
}

fn parse_program(s: &str) -> Vec<i64> {
    s[9..]
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}

fn find_a(program: &[i64]) -> i64 {
    let mut to_search: Vec<_> = vec![0];
    while let Some(p) = to_search.pop() {
        for a in 0..8 {
            let new_a = p * 8 + a;
            let out = run_program(new_a, 0, 0, program);
            if out == program {
                return new_a;
            }
            if program[program.len() - out.len()..] == out {
                to_search.push(new_a);
            }
        }
    }
    unreachable!()
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?;
    let Some((register_str, program_str)) = input.split_once("\n\n") else {
        unreachable!()
    };

    let (init_a, init_b, init_c) = parse_registers(register_str);
    let program = parse_program(program_str);

    let program_output = run_program(init_a, init_b, init_c, &program);

    println!("P1: Program output: {}", program_output.iter().join(","));

    // My program is:
    // while a != 0:
    //     print((((a % 8) ^ 3) ^ (a // (2 ** ((a % 8) ^ 3)))) % 8)
    //     print((a ^ (a >> ((a & 7) ^ 3))) % 8) # simplified
    //     a //= 8
    let reversed_a = find_a(&program);
    println!("P2: Value of A necessary to output a quine: {reversed_a}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1a() {
        let program_output = run_program(10, 0, 0, &[5, 0, 5, 1, 5, 4]);
        assert_eq!(program_output, [0, 1, 2]);
    }

    #[test]
    fn ex1b() {
        let program_output = run_program(2024, 0, 0, &[0, 1, 5, 4, 3, 0]);
        assert_eq!(program_output, [4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    }

    #[test]
    fn ex1c() {
        let program_output = run_program(729, 0, 0, &[0, 1, 5, 4, 3, 0]);
        assert_eq!(program_output, [4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }
}
