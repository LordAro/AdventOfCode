use std::env;
use std::fs;
use std::io;

#[derive(Debug)]
struct Prize {
    a: (i64, i64),
    b: (i64, i64),
    p: (i64, i64),
}

fn parse_prize_instr(prize_str: &str) -> Prize {
    let mut l = prize_str.lines();
    let Some(button_a_str) = l.next() else {
        unreachable!()
    };
    let Some(button_b_str) = l.next() else {
        unreachable!()
    };
    let Some(prize_str) = l.next() else {
        unreachable!()
    };

    let (a_x_str, a_y_str) = button_a_str
        .split_once(": ")
        .unwrap()
        .1
        .split_once(", ")
        .unwrap();
    let a = (
        a_x_str[2..].parse::<i64>().unwrap(),
        a_y_str[2..].parse::<i64>().unwrap(),
    );

    let (b_x_str, b_y_str) = button_b_str
        .split_once(": ")
        .unwrap()
        .1
        .split_once(", ")
        .unwrap();
    let b = (
        b_x_str[2..].parse::<i64>().unwrap(),
        b_y_str[2..].parse::<i64>().unwrap(),
    );

    let (p_x_str, p_y_str) = prize_str
        .split_once(": ")
        .unwrap()
        .1
        .split_once(", ")
        .unwrap();
    let p = (
        p_x_str[2..].parse::<i64>().unwrap(),
        p_y_str[2..].parse::<i64>().unwrap(),
    );
    Prize { a, b, p }
}

// Solves
// a*A0 + b*B0 = P0
// a*A1 + b*B1 = P1
// where Ax, Bx, Ay, By, Px, Py are constants
// and a & b are minimised
fn get_button_presses<const OFFSET: i64>(prize: &Prize) -> Option<(i64, i64)> {
    let Prize { a, b, p } = prize;
    let p = (p.0 + OFFSET, p.1 + OFFSET);
    //println!("{}*{}-{}*{} / {}*{}-{}*{}", a.1, p.0, a.0, p.1, a.1, b.0, a.0, b.1);
    //println!("{} / {}", a.1*p.0-a.0*p.1, a.1*b.0-a.0*b.1);
    // be nice if i had some divmod operation about now
    let has_b_soln = (a.1 * p.0 - a.0 * p.1) % (a.1 * b.0 - a.0 * b.1) == 0;
    let b_presses = (a.1 * p.0 - a.0 * p.1) / (a.1 * b.0 - a.0 * b.1);
    let has_a_soln = (p.0 - b.0 * b_presses) % a.0 == 0;
    let a_presses = (p.0 - b.0 * b_presses) / a.0;
    (has_b_soln && has_a_soln).then_some((a_presses, b_presses))
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?;

    let prize_instrs: Vec<_> = input
        .split("\n\n")
        .map(|prize_str| parse_prize_instr(prize_str))
        .collect();

    let p1_token_total: i64 = prize_instrs.iter()
        .flat_map(|prize| get_button_presses::<0>(prize))
        .map(|(a, b)| a * 3 + b)
        .sum();

    let p2_token_total: i64 = prize_instrs.iter()
        .flat_map(|prize| get_button_presses::<10_000_000_000_000>(prize))
        .map(|(a, b)| a * 3 + b)
        .sum();

    println!("P1: Total number of tokens required: {p1_token_total}");
    println!("P2: Total number of (really big) tokens required: {p2_token_total}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let p = Prize {
            a: (94, 34),
            b: (22, 67),
            p: (8400, 5400),
        };
        assert_eq!(get_button_presses::<0>(&p), Some((80, 40)));
    }

    #[test]
    fn ex1b() {
        let p = Prize {
            a: (26, 66),
            b: (67, 21),
            p: (12748, 12176),
        };
        assert_eq!(get_button_presses::<0>(&p), None);
    }
}
