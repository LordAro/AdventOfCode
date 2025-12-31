use itertools::Itertools;
use std::env;
use std::fs;
use std::io;
use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Sub for Coord {
    type Output = Self;
    fn sub(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Coord {
    fn to_movement_str(self) -> &'static str {
        // unrolled for speed
        //let x_char = if self.x < 0 { "<" } else { ">" };
        //let y_char = if self.y < 0 { "v" } else { "^" };
        //x_char.repeat(self.x.unsigned_abs()) + &y_char.repeat(self.y.unsigned_abs())
        match self {
            Coord { x: -3, y: -3 } => "<<<vvv",
            Coord { x: -3, y: -2 } => "<<<vv",
            Coord { x: -3, y: -1 } => "<<<v",
            Coord { x: -3, y: 0 } => "<<<",
            Coord { x: -3, y: 1 } => "<<<^",
            Coord { x: -3, y: 2 } => "<<<^^",
            Coord { x: -3, y: 3 } => "<<<^^^",
            Coord { x: -2, y: -3 } => "<<vvv",
            Coord { x: -2, y: -2 } => "<<vv",
            Coord { x: -2, y: -1 } => "<<v",
            Coord { x: -2, y: 0 } => "<<",
            Coord { x: -2, y: 1 } => "<<^",
            Coord { x: -2, y: 2 } => "<<^^",
            Coord { x: -2, y: 3 } => "<<^^^",
            Coord { x: -1, y: -3 } => "<vvv",
            Coord { x: -1, y: -2 } => "<vv",
            Coord { x: -1, y: -1 } => "<v",
            Coord { x: -1, y: 0 } => "<",
            Coord { x: -1, y: 1 } => "<^",
            Coord { x: -1, y: 2 } => "<^^",
            Coord { x: -1, y: 3 } => "<^^^",
            Coord { x: 0, y: -3 } => "vvv",
            Coord { x: 0, y: -2 } => "vv",
            Coord { x: 0, y: -1 } => "v",
            Coord { x: 0, y: 0 } => "",
            Coord { x: 0, y: 1 } => "^",
            Coord { x: 0, y: 2 } => "^^",
            Coord { x: 0, y: 3 } => "^^^",
            Coord { x: 1, y: -3 } => ">vvv",
            Coord { x: 1, y: -2 } => ">vv",
            Coord { x: 1, y: -1 } => ">v",
            Coord { x: 1, y: 0 } => ">",
            Coord { x: 1, y: 1 } => ">^",
            Coord { x: 1, y: 2 } => ">^^",
            Coord { x: 1, y: 3 } => ">^^^",
            Coord { x: 2, y: -3 } => ">>vvv",
            Coord { x: 2, y: -2 } => ">>vv",
            Coord { x: 2, y: -1 } => ">>v",
            Coord { x: 2, y: 0 } => ">>",
            Coord { x: 2, y: 1 } => ">>^",
            Coord { x: 2, y: 2 } => ">>^^",
            Coord { x: 2, y: 3 } => ">>^^^",
            Coord { x: 3, y: -3 } => ">>>vvv",
            Coord { x: 3, y: -2 } => ">>>vv",
            Coord { x: 3, y: -1 } => ">>>v",
            Coord { x: 3, y: 0 } => ">>>",
            Coord { x: 3, y: 1 } => ">>>^",
            Coord { x: 3, y: 2 } => ">>>^^",
            Coord { x: 3, y: 3 } => ">>>^^^",
            _ => unreachable!(),
        }
    }
}

fn keypad_coords(c: char) -> Coord {
    // 789
    // 456
    // 123
    //  0A
    match c {
        '7' => Coord { x: 0, y: 3 },
        '8' => Coord { x: 1, y: 3 },
        '9' => Coord { x: 2, y: 3 },
        '4' => Coord { x: 0, y: 2 },
        '5' => Coord { x: 1, y: 2 },
        '6' => Coord { x: 2, y: 2 },
        '1' => Coord { x: 0, y: 1 },
        '2' => Coord { x: 1, y: 1 },
        '3' => Coord { x: 2, y: 1 },
        // blank
        '0' => Coord { x: 1, y: 0 },
        'A' => Coord { x: 2, y: 0 },
        _ => unreachable!(),
    }
}

fn get_keypad_vectors(code: &[char]) -> Vec<&str> {
    code.iter()
        .scan('A', |cur_pos, c| {
            let movement = keypad_coords(*c) - keypad_coords(*cur_pos);
            *cur_pos = *c;

            let move_str = movement.to_movement_str();
            if move_str.is_empty() {
                Some(vec!["A"])
            } else {
                Some(vec![move_str, "A"])
            }
        })
        .flatten()
        .collect()
}

fn directional_coords(c: char) -> Coord {
    //  ^A
    // <v>
    match c {
        // blank
        '^' => Coord { x: 1, y: 1 },
        'A' => Coord { x: 2, y: 1 },
        '<' => Coord { x: 0, y: 0 },
        'v' => Coord { x: 1, y: 0 },
        '>' => Coord { x: 2, y: 0 },
        _ => unreachable!(),
    }
}

fn get_directional_vectors(code: &[char]) -> Vec<&'static str> {
    let mut cur_pos = 'A';
    let mut overall_movement = vec![];
    for c in code {
        let movement = directional_coords(*c) - directional_coords(cur_pos);
        let move_str = movement.to_movement_str();
        if !move_str.is_empty() {
            overall_movement.push(move_str);
        }
        overall_movement.push("A");
        cur_pos = *c;
    }
    overall_movement
}

fn route_goes_over_blank(code: &[char], blank: Coord) -> bool {
    let mut cur_pos = keypad_coords('A');
    for c in code.iter() {
        match c {
            '^' => cur_pos.y += 1,
            'v' => cur_pos.y -= 1,
            '>' => cur_pos.x += 1,
            '<' => cur_pos.x -= 1,
            _ => (),
        }
        if cur_pos == blank {
            return true;
        }
    }
    false
}

// fun with iterators.
// all permutations of inner elements while maintaining existing order of outer elements
fn all_inner_combinations<'a>(input: &[&'a str]) -> impl Iterator<Item = Vec<char>> + use<'a> {
    input
        .iter()
        // dedup is ok here because number of permutations per string is relatively small
        .map(|c| c.chars().permutations(c.len()).dedup())
        .multi_cartesian_product()
        .map(|v| v.into_iter().flatten().collect())
}

fn get_shortest_derivative_directional_moves(code: &str, robots: usize) -> String {
    let keypad_vec: Vec<_> = code.chars().collect();
    // inner-most robot
    let mut possible_movement_vecs: Vec<_> =
        all_inner_combinations(&get_keypad_vectors(&keypad_vec))
            .filter(|kv| !route_goes_over_blank(kv, Coord { x: 0, y: 0 }))
            .collect();
    for _ in 0..robots {
        //println!("robot: {n}");
        //println!("foo1 {}", possible_movement_vecs.len());
        possible_movement_vecs = possible_movement_vecs
            .iter()
            .flat_map(|movement_vec| {
                let init_dv = get_directional_vectors(movement_vec);
                //println!(
                //    "foo mv {} -> {} / {}",
                //    movement_vec.len(),
                //    init_dv.len(),
                //    init_dv.iter().map(|d| d.len()).sum::<usize>()
                //);
                all_inner_combinations(&init_dv)
                    .find(|kv| !route_goes_over_blank(kv, Coord { x: 0, y: 1 }))
            })
            .collect();
        //println!("foo2 {}", possible_movement_vecs.len());
        let min_len = possible_movement_vecs
            .iter()
            .map(|movement_vec| movement_vec.len())
            .min()
            .unwrap();
        //println!("foo min {}", min_len);
        possible_movement_vecs.retain(|movement_vec| movement_vec.len() == min_len);
    }
    possible_movement_vecs[0].iter().collect()
}

fn main() -> io::Result<()> {
    let codes: Vec<String> = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?
        .lines()
        .map(|s| s.to_string())
        .collect();

    let code_complexity: usize = codes
        .iter()
        .map(|code| {
            let moves = get_shortest_derivative_directional_moves(code, 2);
            let num: usize = code[0..3].parse().unwrap();
            moves.len() * num
        })
        .sum();
    println!("P1: Code complexity score: {code_complexity}");

    let code_complexity: usize = codes
        .iter()
        .map(|code| {
            let moves = get_shortest_derivative_directional_moves(code, 25);
            let num: usize = code[0..3].parse().unwrap();
            moves.len() * num
        })
        .sum();
    println!("P2: Code complexity score: {code_complexity}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1a() {
        let code = "029A";
        let moves = get_shortest_derivative_directional_moves(code, 2);
        assert_eq!(
            moves.len(),
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn ex1b() {
        let code = "980A";
        let moves = get_shortest_derivative_directional_moves(code, 2);
        assert_eq!(
            moves.len(),
            "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len()
        );
    }

    #[test]
    fn ex1c() {
        let code = "179A";
        let moves = get_shortest_derivative_directional_moves(code, 2);
        assert_eq!(
            moves.len(),
            "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn ex1d() {
        let code = "456A";
        let moves = get_shortest_derivative_directional_moves(code, 2);
        assert_eq!(
            moves.len(),
            "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".len()
        );
    }

    #[test]
    fn ex1e() {
        let code = "379A";
        let moves = get_shortest_derivative_directional_moves(code, 2);
        assert_eq!(
            moves.len(),
            "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
        );
    }
}
