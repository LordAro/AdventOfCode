use std::fs;
use std::io;

// numbers go brr
#[derive(Debug)]
struct Coord {
    x: i128,
    y: i128,
}

fn parse_snails(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|l| {
            let mut it = l.split(' ');
            let x = it.next().unwrap()[2..].parse().unwrap();
            let y = it.next().unwrap()[2..].parse().unwrap();
            Coord { x, y }
        })
        .collect()
}

// Returns a list `result` of size 3 where:
// Referring to the equation ax + by = gcd(a, b)
//     result[0] is gcd(a, b)
//     result[1] is x
//     result[2] is y
// Returns r, s & t such that a*s + b*t = r (r == gcd(a, b))
fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    let mut s = (0, 1);
    let mut t = (1, 0);
    let mut r = (b, a);
    while r.0 != 0 {
        let quot = r.1 / r.0;
        r = (r.1 - quot * r.0, r.0);
        s = (s.1 - quot * s.0, s.0);
        t = (t.1 - quot * t.0, t.0);
    }
    //println!("{a}*{} + {b}*{} = gcd({a}, {b}) = {}", s.1, t.1, r.1);
    (r.1, s.1, t.1)
}

fn reduce_incongruence(a: i128, b: i128, m: i128, n: i128) -> (i128, i128) {
    let (r, u, v) = extended_gcd(m, n);
    let l = (a - b) / r;
    assert_eq!(b + (n * l * v), a - (m * u * l));
    (b + (n * l * v), (m * n / r))
}

fn get_snail_alignment(snails: &[Coord]) -> i128 {
    let (a, b) = snails
        .iter()
        .map(|snail| {
            // solve x = dist mod cycle_len
            let dist = snail.y - 1;
            let cycle_len = snail.x + snail.y - 1;
            (dist, cycle_len)
        })
        .fold((1, 1), |(a, m), (b, n)| {
            let (c, d) = reduce_incongruence(a, b, m, n);
            (c.rem_euclid(d), d)
        });
    a.rem_euclid(b)
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let mut p1_snails = parse_snails(&fs::read_to_string(p1_input_filename)?);
    for _ in 0..100 {
        for snail in &mut p1_snails {
            if snail.y == 1 {
                snail.y = snail.x;
                snail.x = 1;
            } else {
                snail.x += 1;
                snail.y -= 1;
            }
        }
    }
    let p1_snail_sum: i128 = p1_snails.iter().map(|s| s.x + 100 * s.y).sum();

    let p2_snails = parse_snails(&fs::read_to_string(p2_input_filename)?);
    let p2_day_length = get_snail_alignment(&p2_snails);

    let p3_snails = parse_snails(&fs::read_to_string(p3_input_filename)?);
    let p3_day_length = get_snail_alignment(&p3_snails);
    println!("P1: Snail sum (100 days): {p1_snail_sum}");
    println!("P2: Number of days to wait: {p2_day_length}");
    println!("P3: Number of days to wait: {p3_day_length}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    // confirm incongruence stuff works
    #[test]
    fn ex2pre() {
        let input = vec![(2, 3), (3, 5), (2, 7)];
        let ans = input
            .into_iter()
            .fold((0, 1), |(a, m), (b, n)| reduce_incongruence(a, b, m, n));
        assert_eq!((ans.0.rem_euclid(ans.1), ans.1), (23, 105));

        // non-coprime
        let input = vec![(3, 10), (5, 12)];
        let ans = input
            .into_iter()
            .fold((0, 1), |(a, m), (b, n)| reduce_incongruence(a, b, m, n));
        assert_eq!((ans.0.rem_euclid(ans.1), ans.1), (53, 60));
    }

    #[test]
    fn ex2() {
        let input = "x=12 y=2
x=8 y=4
x=7 y=1
x=1 y=5
x=1 y=3";
        let snails = parse_snails(input);
        let alignment_day = get_snail_alignment(&snails);
        assert_eq!(alignment_day, 14);
    }

    #[test]
    fn ex2b() {
        let input = "x=3 y=1
x=3 y=9
x=1 y=5
x=4 y=10
x=5 y=3";
        let snails = parse_snails(input);
        let alignment_day = get_snail_alignment(&snails);
        assert_eq!(alignment_day, 13659);
    }
}
