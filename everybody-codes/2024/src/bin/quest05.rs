use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;

fn get_starting_positions(input: &str) -> Vec<Vec<usize>> {
    let v: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.split(" ").map(|c| c.parse::<usize>().unwrap()).collect())
        .collect();

    // Convert to columnular data
    let mut rot = vec![vec![0; v.len()]; v[0].len()];
    for (y, row) in v.iter().enumerate() {
        for x in 0..row.len() {
            rot[x][y] = row[x];
        }
    }
    rot
}

fn do_move(positions: &mut [Vec<usize>], move_no: usize) {
    // In theory slightly more efficient to use a VecDeque, but this is good enough
    let cur_col = move_no % positions.len();
    let clapper = positions[cur_col].remove(0);

    let next_col = (cur_col + 1) % positions.len();
    let next_col_len = positions[next_col].len();

    let mut is_left_side = ((clapper / next_col_len) % 2) == 0;
    let mut new_clapper_pos = clapper;
    if clapper % next_col_len == 0 {
        new_clapper_pos = next_col_len;
        is_left_side = !is_left_side;
    } else {
        new_clapper_pos %= next_col_len;
    }

    if is_left_side {
        positions[next_col].insert(new_clapper_pos - 1, clapper);
    } else {
        positions[next_col].insert(next_col_len - new_clapper_pos + 1, clapper);
    }
}

fn get_shout_number(positions: &[Vec<usize>]) -> usize {
    positions.iter().fold(0, |acc, col| {
        acc * 10_usize.pow(col[0].ilog10() + 1) + col[0]
    })
}

fn do_2024_dance(positions: &mut [Vec<usize>]) -> (usize, usize) {
    let mut shout_map = HashMap::<usize, usize>::new();

    let mut move_no = 0;
    loop {
        do_move(positions, move_no);
        let shout_no = get_shout_number(positions);

        let shout_total = *shout_map
            .entry(shout_no)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
        if shout_total == 2024 {
            break;
        }
        move_no += 1;
    }
    let big_shout = get_shout_number(positions);
    (move_no + 1, big_shout) // convert to 1-based
}

fn do_forever_dance(positions: &mut [Vec<usize>]) -> usize {
    let mut state_set = HashSet::<Vec<Vec<usize>>>::new();
    let mut shout_set = HashSet::<usize>::new();

    for move_no in 0.. {
        state_set.insert(positions.to_vec());
        let shout_no = get_shout_number(positions);
        shout_set.insert(shout_no);

        do_move(positions, move_no);
        if state_set.contains(positions) {
            break;
        }
    }
    *shout_set.iter().max().unwrap()
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    {
        let mut p1_state = get_starting_positions(&fs::read_to_string(p1_input_filename)?);
        for move_no in 0..10 {
            do_move(&mut p1_state, move_no);
        }
        println!(
            "P1: Shouted number after 10 rounds: {}",
            get_shout_number(&p1_state)
        );
    }
    {
        let mut p2_state = get_starting_positions(&fs::read_to_string(p2_input_filename)?);
        let (p2_total_moves, p2_final_shout) = do_2024_dance(&mut p2_state);

        println!(
            "P2: Shouted number after {} rounds: {} ({})",
            p2_total_moves,
            p2_final_shout,
            p2_total_moves * p2_final_shout
        );
    }
    {
        let mut p3_state = get_starting_positions(&fs::read_to_string(p3_input_filename)?);
        let p3_largest_shout = do_forever_dance(&mut p3_state);

        println!(
            "P3: Largest shout number for forever dance: {}",
            p3_largest_shout,
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input_str = "2 3 4 5
3 4 5 2
4 5 2 3
5 2 3 4";
        let mut state = get_starting_positions(input_str);
        assert_eq!(
            state,
            [[2, 3, 4, 5], [3, 4, 5, 2], [4, 5, 2, 3], [5, 2, 3, 4]]
        );

        do_move(&mut state, 0);
        assert_eq!(
            state,
            vec![
                vec![3, 4, 5],
                vec![3, 2, 4, 5, 2],
                vec![4, 5, 2, 3],
                vec![5, 2, 3, 4]
            ]
        );
        assert_eq!(get_shout_number(&state), 3345);

        do_move(&mut state, 1);
        assert_eq!(
            state,
            vec![
                vec![3, 4, 5],
                vec![2, 4, 5, 2],
                vec![4, 5, 3, 2, 3],
                vec![5, 2, 3, 4]
            ]
        );
        assert_eq!(get_shout_number(&state), 3245);

        do_move(&mut state, 2);
        assert_eq!(
            state,
            vec![
                vec![3, 4, 5],
                vec![2, 4, 5, 2],
                vec![5, 3, 2, 3],
                vec![5, 2, 3, 4, 4]
            ]
        );
        assert_eq!(get_shout_number(&state), 3255);

        do_move(&mut state, 3);
        assert_eq!(
            state,
            vec![
                vec![3, 4, 5, 5],
                vec![2, 4, 5, 2],
                vec![5, 3, 2, 3],
                vec![2, 3, 4, 4]
            ]
        );
        assert_eq!(get_shout_number(&state), 3252);
    }

    #[test]
    fn ex1b() {
        let input_str = "2 3 4 5
3 4 5 2
4 5 2 3
5 2 3 4";
        let mut state = get_starting_positions(input_str);
        let shout_numbers: Vec<_> = (0..10)
            .map(|move_no| {
                do_move(&mut state, move_no);
                get_shout_number(&state)
            })
            .collect();
        assert_eq!(
            shout_numbers,
            [3345, 3245, 3255, 3252, 4252, 4452, 4422, 4423, 2423, 2323]
        );
    }

    #[test]
    fn ex2() {
        let input_str = "2 3 4 5\n6 7 8 9";
        let mut state = get_starting_positions(input_str);
        let (total_moves, final_shout) = do_2024_dance(&mut state);
        assert_eq!(total_moves, 8095);
        assert_eq!(final_shout, 6285);
    }

    #[test]
    fn ex3() {
        let input_str = "2 3 4 5\n6 7 8 9";
        let mut state = get_starting_positions(input_str);
        let largest_shout = do_forever_dance(&mut state);
        assert_eq!(largest_shout, 6584);
    }
}
