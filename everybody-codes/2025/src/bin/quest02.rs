use std::fs;
use std::io;

type Point = (isize, isize);

fn get_a(filename: &str) -> io::Result<Point> {
    let input: String = fs::read_to_string(filename)?;
    let a_com_vec: Vec<_> = input[3..input.len() - 1] // A=[...]
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect();
    Ok((a_com_vec[0], a_com_vec[1]))
}

fn add(a: Point, b: Point) -> Point {
    (a.0 + b.0, a.1 + b.1)
}

fn div(a: Point, b: Point) -> Point {
    (a.0 / b.0, a.1 / b.1)
}

// [X1,Y1] * [X2,Y2] = [X1 * X2 - Y1 * Y2, X1 * Y2 + Y1 * X2]
fn mul(a: Point, b: Point) -> Point {
    (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
}

fn engrave_point(a: Point) -> Option<Point> {
    let mut r_com = (0, 0);
    for _ in 0..100 {
        r_com = mul(r_com, r_com);
        r_com = div(r_com, (100_000, 100_000));
        r_com = add(r_com, a);
        if !(-1_000_000..=1_000_000).contains(&r_com.0)
            || !(-1_000_000..=1_000_000).contains(&r_com.1)
        {
            return None;
        }
    }
    Some(r_com)
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let a_com = get_a(&p1_input_filename)?;

    let mut r_com = (0, 0);
    for _ in 0..3 {
        // R = R * R
        r_com = mul(r_com, r_com);
        // R = R / [10,10]
        r_com = div(r_com, (10, 10));
        // R = R + A
        r_com = add(r_com, a_com);
    }

    let a_com = get_a(&p2_input_filename)?;

    let mut p2_will_engrave = 0;
    let mut p3_will_engrave = 0;
    for a_x in (a_com.0..=a_com.0 + 1000) {
        for a_y in (a_com.1..=a_com.1 + 1000) {
            let cur_a = (a_x, a_y);
            let point = engrave_point(cur_a);
            if point.is_some() {
                if (a_x - a_com.0) % 10 == 0 && (a_y - a_com.1) % 10 == 0 {
                    p2_will_engrave += 1;
                }
                p3_will_engrave += 1;
            }
        }
    }

    println!("P1: Result: [{},{}]", r_com.0, r_com.1);
    println!("P2: Number of engraved points: {p2_will_engrave}");
    println!("P2: Number of detailed engraved points: {p3_will_engrave}");
    Ok(())
}
