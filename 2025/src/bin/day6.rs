use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input: String = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?;
    let input_lines: Vec<_> = input.lines().collect();

    let rows: Vec<Vec<_>> = input_lines[0..input_lines.len() - 1]
        .iter()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let operators: Vec<_> = input_lines[input_lines.len() - 1]
        .split_ascii_whitespace()
        .collect();

    let p1_sum: usize = (0..operators.len())
        .map(|i| {
            let col_sum: usize = match operators[i] {
                "*" => rows.iter().map(|r| r[i]).product(),
                "+" => rows.iter().map(|r| r[i]).sum(),
                _ => unreachable!(),
            };
            col_sum
        })
        .sum();

    // rtl column indexes
    let mut p2_sum: usize = 0;
    let mut cur_nums = vec![];
    for i in (0..input_lines[0].len()).rev() {
        let mut cur_num = 0;
        for cur_char in input_lines.iter().map(|l| l.as_bytes()[i]) {
            match cur_char {
                b'*' => {
                    cur_nums.push(cur_num);
                    p2_sum += cur_nums.iter().product::<usize>();
                    cur_nums.clear();
                    cur_num = 0;
                }
                b'+' => {
                    cur_nums.push(cur_num);
                    p2_sum += cur_nums.iter().sum::<usize>();
                    cur_nums.clear();
                    cur_num = 0;
                }
                b' ' => {}
                b'0'..=b'9' => {
                    let n = cur_char - b'0';
                    cur_num = cur_num * 10 + n as usize;
                }
                _ => unreachable!(),
            }
        }
        if cur_num > 0 {
            cur_nums.push(cur_num);
            cur_num = 0;
        }
    }

    println!("P1: Grand total: {p1_sum}");
    println!("P2: Grand total with cephalopod maths: {p2_sum}");
    Ok(())
}
