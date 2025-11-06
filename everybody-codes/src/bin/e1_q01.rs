use std::cmp;
use std::fs;
use std::io;

type InputTuple = (usize, usize, usize, usize, usize, usize, usize);

fn parse_input(input_str: &str) -> Vec<InputTuple> {
    input_str
        .lines()
        .map(|l| {
            // A=4 B=4 C=6 X=3 Y=4 Z=5 M=11
            let mut it = l.split(' ').map(|s| s[2..].parse::<usize>().unwrap());
            (
                it.next().unwrap(), // a
                it.next().unwrap(), // b
                it.next().unwrap(), // c
                it.next().unwrap(), // x
                it.next().unwrap(), // y
                it.next().unwrap(), // z
                it.next().unwrap(), // m
            )
        })
        .collect()
}


fn gcd(a: usize, b: usize) -> usize {
    // Use Stein's algorithm
    let mut m = a;
    let mut n = b;
    if m == 0 || n == 0 {
        return (m | n);
    }

    // find common factors of 2
    let shift = (m | n).trailing_zeros();

    // divide n and m by 2 until odd
    m >>= m.trailing_zeros();
    n >>= n.trailing_zeros();

    while m != n {
        if m > n {
            m -= n;
            m >>= m.trailing_zeros();
        } else {
            n -= m;
            n >>= n.trailing_zeros();
        }
    }
    m << shift
}

fn eni(n: usize, exp: usize, mod_: usize) -> usize {
    let mut res = 0;
    let mut score = 1;
    let mut it = 0;
    for _ in 0..exp {
        score = (score * n).rem_euclid(mod_);
        res += score * 10_usize.pow(it);
        it += if score > 0 { score.ilog10() } else { 0 } + 1;
    }
    res
}

fn eni_lim(n: usize, exp: usize, mod_: usize) -> usize {
    let mut res = vec![];
    let mut score = 1;
    for _ in 0..exp {
        score = (score * n).rem_euclid(mod_);
        if res.contains(&score) {
            break;
        }
        res.push(score);
    }
    res.iter()
        .rev()
        .cycle()
        .skip(res.len() - exp % res.len())
        .take(cmp::min(exp, 5))
        .fold(0, |acc, n| {
            let num_digits = if *n > 0 { n.ilog10() } else { 0 } + 1;
            acc * 10_usize.pow(num_digits) + n
        })
}

fn eni_sum(n: usize, exp: usize, mod_: usize) -> usize {
    println!("eni_sum({n}, {exp}, {mod_})");
    let mut res = vec![];
    let mut score = 1;
    let mut repeat_idx = 0;
    for _ in 0..exp {
        score = (score * n).rem_euclid(mod_);
        if let Some(n) = res.iter().position(|x| *x == score) {
            repeat_idx = n;
            break;
        }
        res.push(score);
    }
    let rem_start: usize = res.iter().take(repeat_idx).sum(); // always at most 1?
    let repeat_sum: usize = res.iter().skip(repeat_idx).sum();
    let repeated_total = exp - 1) / (res.len() - repeat_idx) * repeat_sum;
    let rem_end: usize = res.iter().skip(exp % (res.len() - repeat_idx)).sum();
    println!("{exp} == {repeat_idx} + {} + {}", exp / (res.len() - repeat_idx), exp % (res.len() - repeat_idx));
    assert_eq!(repeat_idx + (exp / (res.len() - repeat_idx)) + (exp % (res.len() - repeat_idx)), exp);
    rem_start + repeated_total + rem_end
}

fn eni_sum_dumb(n: usize, exp: usize, mod_: usize) -> usize {
    let mut res = vec![];
    let mut score = 1;
    for _ in 0..exp {
        score = (score * n).rem_euclid(mod_);
        res.push(score);
    }
    res.iter().sum()
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_input = parse_input(&fs::read_to_string(p1_input_filename)?);
    let p1_max_calc = p1_input
        .iter()
        .map(|&(a, b, c, x, y, z, m)| eni(a, x, m) + eni(b, y, m) + eni(c, z, m))
        .max()
        .unwrap();

    let p2_input = parse_input(&fs::read_to_string(p2_input_filename)?);
    let p2_max_calc = p2_input
        .iter()
        .map(|&(a, b, c, x, y, z, m)| eni_lim(a, x, m) + eni_lim(b, y, m) + eni_lim(c, z, m))
        .max()
        .unwrap();

    let p3_input = parse_input(&fs::read_to_string(p3_input_filename)?);
    let p3_max_calc = p3_input
        .iter()
        .map(|&(a, b, c, x, y, z, m)| eni_sum(a, x, m) + eni_sum(b, y, m) + eni_sum(c, z, m))
        .max()
        .unwrap();

    println!("P1: Maximum value: {p1_max_calc}");
    println!("P2: Maximum limited value: {p2_max_calc}");
    println!("P3: Maximum sum value: {p3_max_calc}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex2() {
        assert_eq!(eni_lim(2, 7, 5), 34_213);
        assert_eq!(eni_lim(3, 8, 16), 111_931);

        assert_eq!(eni_lim(4, 3, 11), 954);
        assert_eq!(eni_lim(5, 8, 14), 115_139);
        assert_eq!(eni_lim(5, 6, 15), 10_510_510);
        assert_eq!(eni_lim(8, 14, 13), 12_815_12);
    }

    #[test]
    fn ex3() {
        assert_eq!(eni_sum(2, 7, 5), 19);
        assert_eq!(eni_sum(3, 8, 16), 48);

        assert_eq!(eni_sum(4, 3_000, 110), 132_000);
        assert_eq!(eni_sum(4, 14_000, 110), 616_000);
        assert_eq!(eni_sum(6, 15_000, 110), 825_000);

        assert_eq!(eni_sum(8, 8_000, 120), 240_000);
        assert_eq!(eni_sum(4, 14_000, 120), 559_940);
        assert_eq!(eni_sum(7, 16_000, 120), 640_000);

        assert_eq!(eni_sum(2, 2_000, 130), 129_860);
        assert_eq!(eni_sum(8, 14_000, 130), 910_000);
        assert_eq!(eni_sum(6, 15_000, 130), 1_040_000);

        assert_eq!(eni_sum(5, 8_000, 140), 466_580);
        assert_eq!(eni_sum(9, 16_000, 140), 933_340);
        assert_eq!(eni_sum(6, 18_000, 140), 1_007_930);
    }

    #[test]
    fn ex3b() {
        let input_str = "A=4 B=4 C=6 X=3000 Y=14000 Z=15000 M=110
A=8 B=4 C=7 X=8000 Y=14000 Z=16000 M=120
A=2 B=8 C=6 X=2000 Y=14000 Z=15000 M=130
A=5 B=9 C=6 X=8000 Y=16000 Z=18000 M=140
A=5 B=9 C=7 X=6000 Y=16000 Z=18000 M=150
A=8 B=8 C=8 X=6000 Y=19000 Z=16000 M=160";
        let input = parse_input(input_str);
        let max_calc = input
            .iter()
            .map(|&(a, b, c, x, y, z, m)| eni_sum(a, x, m) + eni_sum(b, y, m) + eni_sum(c, z, m))
            .max()
            .unwrap();
        assert_eq!(max_calc, 3279640);
    }

    #[test]
    fn ex3c() {
        let input_str = "A=3657 B=3583 C=9716 X=903056852 Y=9283895500 Z=85920867478 M=188
A=6061 B=4425 C=5082 X=731145782 Y=1550090416 Z=87586428967 M=107
A=7818 B=5395 C=9975 X=122388873 Y=4093041057 Z=58606045432 M=102
A=7681 B=9603 C=5681 X=716116871 Y=6421884967 Z=66298999264 M=196
A=7334 B=9016 C=8524 X=297284338 Y=1565962337 Z=86750102612 M=145";
        let input = parse_input(input_str);
        let max_calc = input
            .iter()
            .map(|&(a, b, c, x, y, z, m)| eni_sum(a, x, m) + eni_sum(b, y, m) + eni_sum(c, z, m))
            .max()
            .unwrap();
        assert_eq!(max_calc, 7276515438396);

    }
}
