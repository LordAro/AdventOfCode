use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs;
use std::io;
use std::str::FromStr;

#[derive(Clone, PartialEq)]
enum Action {
    Increase,
    Decrease,
    Maintain,
    Start,
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Action::Increase => '+',
                Action::Decrease => '-',
                Action::Maintain => '=',
                Action::Start => 'S',
            }
        )
    }
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Action, ()> {
        match s {
            "+" => Ok(Action::Increase),
            "-" => Ok(Action::Decrease),
            "=" => Ok(Action::Maintain),
            "S" => Ok(Action::Start),
            _ => Err(()),
        }
    }
}

impl Action {
    pub fn from(c: char) -> Result<Action, ()> {
        match c {
            '+' => Ok(Action::Increase),
            '-' => Ok(Action::Decrease),
            '=' => Ok(Action::Maintain),
            'S' => Ok(Action::Start),
            _ => Err(()),
        }
    }
}

fn parse_device_actions(input: &str) -> HashMap<char, Vec<Action>> {
    input
        .lines()
        .map(|l| {
            let key = l.chars().next().unwrap();
            let actions: Vec<_> = l[2..].split(',').map(|c| c.parse().unwrap()).collect();
            (key, actions)
        })
        .collect()
}

// Extremely hacky path following. Convert to byte array for indexing purposes
fn parse_track_fancy(input: &[&[u8]]) -> Vec<Action> {
    let mut seen_coords = HashSet::<(usize, usize)>::new();
    let mut coord = (0, 1); // y, x
    let mut parsed_track = vec![Action::from(input[coord.0][coord.1] as char).unwrap()];
    while input[coord.0][coord.1] != b'S' {
        if coord.1 < input[coord.0].len() - 1
            && input[coord.0][coord.1 + 1] != b' '
            && !seen_coords.contains(&(coord.0, coord.1 + 1))
        {
            // E
            coord = (coord.0, coord.1 + 1);
        } else if coord.0 < input.len() - 1
            && input[coord.0 + 1][coord.1] != b' '
            && !seen_coords.contains(&(coord.0 + 1, coord.1))
        {
            // S
            coord = (coord.0 + 1, coord.1);
        } else if coord.1 > 0
            && input[coord.0][coord.1 - 1] != b' '
            && !seen_coords.contains(&(coord.0, coord.1 - 1))
        {
            // W
            coord = (coord.0, coord.1 - 1);
        } else if coord.0 > 0
            && input[coord.0 - 1][coord.1] != b' '
            && !seen_coords.contains(&(coord.0 - 1, coord.1))
        {
            // N
            coord = (coord.0 - 1, coord.1);
        } else {
            unreachable!();
        }
        seen_coords.insert(coord);
        parsed_track.push(Action::from(input[coord.0][coord.1] as char).unwrap());
    }
    parsed_track
}

fn run_race_individual(actions: &[Action], total_rounds: usize) -> usize {
    let mut accumulated_power = (0, 10);
    for round_no in 0..total_rounds {
        let cur_power = accumulated_power.1;
        let action = &actions[round_no % actions.len()];
        let new_power = match action {
            Action::Increase => cur_power + 1,
            Action::Decrease => cur_power - 1,
            Action::Maintain => cur_power,
            Action::Start => unreachable!(),
        };
        accumulated_power = (accumulated_power.0 + cur_power, new_power);
    }
    accumulated_power.0 + accumulated_power.1 - 10 // remove initial default
}

fn run_race(
    device_actions: &HashMap<char, Vec<Action>>,
    total_rounds: usize,
) -> HashMap<char, usize> {
    device_actions
        .iter()
        .map(|(&key, action_plan)| (key, run_race_individual(action_plan, total_rounds)))
        .collect()
}

fn run_race_track_individual(track: &[Action], actions: &[Action], lap_count: usize) -> usize {
    //let mut accumulated_power = Vec::with_capacity(lap_count * track.len());
    let mut accumulated_power = (0, 10);
    for round_no in 0..track.len() * lap_count {
        let cur_power = accumulated_power.1;
        let action = &actions[round_no % actions.len()];
        let track_action = &track[round_no % track.len()];

        let new_power = match (track_action, action) {
            (Action::Increase, _) => cur_power + 1,
            (Action::Decrease, _) => cur_power - 1,
            (Action::Start | Action::Maintain, Action::Increase) => cur_power + 1,
            (Action::Start | Action::Maintain, Action::Decrease) => cur_power - 1,
            _ => cur_power,
        };
        accumulated_power = (accumulated_power.0 + cur_power, new_power);
    }
    accumulated_power.0 + accumulated_power.1 - 10 // remove initial default
}

fn run_on_track(
    track: &[Action],
    device_actions: &HashMap<char, Vec<Action>>,
    lap_count: usize,
) -> HashMap<char, usize> {
    device_actions
        .iter()
        .map(|(&key, action_plan)| {
            (
                key,
                run_race_track_individual(track, action_plan, lap_count),
            )
        })
        .collect()
}

fn get_race_results(results: &HashMap<char, usize>) -> String {
    results
        .iter()
        .sorted_by_key(|(_, v)| *v)
        .map(|(k, _)| k)
        .rev()
        .collect()
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_device_actions = parse_device_actions(&fs::read_to_string(p1_input_filename)?);
    let p1_race_results = get_race_results(&run_race(&p1_device_actions, 10));
    println!("P1: Race results: {p1_race_results}");

    let p2_race_track = parse_track_fancy(&[
        "S-=++=-==++=++=-=+=-=+=+=--=-=++=-==++=-+=-=+=-=+=+=++=-+==++=++=-=-=--".as_bytes(),
        "-                                                                     -".as_bytes(),
        "=                                                                     =".as_bytes(),
        "+                                                                     +".as_bytes(),
        "=                                                                     +".as_bytes(),
        "+                                                                     =".as_bytes(),
        "=                                                                     =".as_bytes(),
        "-                                                                     -".as_bytes(),
        "--==++++==+=+++-=+=-=+=-+-=+-=+-=+=-=+=--=+++=++=+++==++==--=+=++==+++-".as_bytes(),
    ]);

    let p2_device_actions = parse_device_actions(&fs::read_to_string(p2_input_filename)?);
    let p2_race_results = get_race_results(&run_on_track(&p2_race_track, &p2_device_actions, 10));
    println!("P2: Race results: {p2_race_results}");

    let p3_race_track = parse_track_fancy(&[
        "S+= +=-== +=++=     =+=+=--=    =-= ++=     +=-  =+=++=-+==+ =++=-=-=--".as_bytes(),
        "- + +   + =   =     =      =   == = - -     - =  =         =-=        -".as_bytes(),
        "= + + +-- =-= ==-==-= --++ +  == == = +     - =  =    ==++=    =++=-=++".as_bytes(),
        "+ + + =     +         =  + + == == ++ =     = =  ==   =   = =++=       ".as_bytes(),
        "= = + + +== +==     =++ == =+=  =  +  +==-=++ =   =++ --= + =          ".as_bytes(),
        "+ ==- = + =   = =+= =   =       ++--          +     =   = = =--= ==++==".as_bytes(),
        "=     ==- ==+-- = = = ++= +=--      ==+ ==--= +--+=-= ==- ==   =+=    =".as_bytes(),
        "-               = = = =   +  +  ==+ = = +   =        ++    =          -".as_bytes(),
        "-               = + + =   +  -  = + = = +   =        +     =          -".as_bytes(),
        "--==++++==+=+++-= =-= =-+-=  =+-= =-= =--   +=++=+++==     -=+=++==+++-".as_bytes(),
    ]);
    let p3_device_actions = parse_device_actions(&fs::read_to_string(p3_input_filename)?);
    let p3_opposition_score: usize =
        run_race_track_individual(&p3_race_track, p3_device_actions.get(&'A').unwrap(), 2024);

    let initial_action_plan: Vec<_> = "+++++---==="
        .chars()
        .map(|c| Action::from(c).unwrap())
        .collect();
    let perms = initial_action_plan.into_iter().permutations(11);
    let num_winning_plans = perms
        .filter(|action_plan| {
            let score: usize = run_race_track_individual(&p3_race_track, action_plan, 2024);
            score > p3_opposition_score
        })
        .count();
    println!("P3: Number of winning action plans: {num_winning_plans} (beating {p3_opposition_score})");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn parse_track_hardcoded(input: &str) -> Vec<Action> {
        input.chars().map(|c| Action::from(c).unwrap()).collect()
    }

    #[test]
    fn ex1() {
        let input_str = "A:+,-,=,=\nB:+,=,-,+\nC:=,-,+,+\nD:=,=,=,+";

        let actions = parse_device_actions(input_str);
        let race_scores = run_race(&actions, 10);
        let expected_scores = HashMap::from([
            ('A', 103),
            ('B', 116),
            ('C', 107),
            ('D', 110),
        ]);
        assert_eq!(race_scores, expected_scores);
        let race_result = get_race_results(&race_scores);
        assert_eq!(race_result, "BDCA");
    }

    #[test]
    fn ex2() {
        let input_str = "A:+,-,=,=\nB:+,=,-,+\nC:=,-,+,+\nD:=,=,=,+";
        let race_track = parse_track_hardcoded("+===++-=+=-S");

        let actions = parse_device_actions(input_str);
        let race_result = get_race_results(&run_on_track(&race_track, &actions, 10));
        assert_eq!(race_result, "DCBA");
    }

    #[test]
    fn ex2_track() {
        let expected_track = parse_track_hardcoded("+===++-=+=-S");
        let input_track = ["S+===".as_bytes(), "-   +".as_bytes(), "=+=-+".as_bytes()];
        assert_eq!(parse_track_fancy(&input_track), expected_track);
    }
}
