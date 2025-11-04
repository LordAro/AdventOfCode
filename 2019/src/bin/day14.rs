use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type ReactionMap<'a> = HashMap<&'a str, (u64, Vec<(&'a str, u64)>)>;

fn parse_reactions<T: AsRef<str>>(input_str: &[T]) -> ReactionMap<'_> {
    input_str
        .iter()
        .map(|s| {
            let mut in_out = s.as_ref().split(" => ");
            let inputs = in_out
                .next()
                .unwrap()
                .split(", ")
                .map(|s| {
                    let mut ins = s.split(' ');
                    let in_n = ins.next().unwrap().parse::<u64>().unwrap();
                    let in_name = ins.next().unwrap();
                    (in_name, in_n)
                })
                .collect::<Vec<_>>();

            let mut out = in_out.next().unwrap().split(' ');
            let output_n = out.next().unwrap().parse::<u64>().unwrap();
            let output_name = out.next().unwrap();
            (output_name, (output_n, inputs))
        })
        .collect()
}

fn get_ore_count<'a>(
    reactions: &'a ReactionMap,
    target: &'a str,
    quantity_required: u64,
    waste: &mut HashMap<&'a str, u64>,
) -> u64 {
    let mut quantity_required = quantity_required;
    if target == "ORE" {
        return quantity_required;
    }
    let wasted_count = waste.entry(target).or_default();
    //println!("Got {} spare", wasted_count);
    if *wasted_count >= quantity_required {
        *wasted_count -= quantity_required;
        return 0; // No extra required
    } else {
        quantity_required -= *wasted_count;
        *wasted_count = 0;
    }
    let (reaction_output_count, inputs_required) = reactions.get(target).unwrap();
    let reaction_count = quantity_required.div_ceil(*reaction_output_count);
    *waste.entry(target).or_default() += reaction_output_count * reaction_count - quantity_required;
    inputs_required
        .iter()
        .map(|(resource, amount)| {
            get_ore_count(reactions, resource, reaction_count * amount, waste)
        })
        .sum()
}

fn required_ore_count(reactions: &ReactionMap) -> u64 {
    let mut waste = HashMap::new();
    get_ore_count(reactions, "FUEL", 1, &mut waste)
}

fn get_fuel_count(reactions: &ReactionMap) -> u64 {
    let mut lower_bound = 1;
    let mut upper_bound = 2;
    while get_ore_count(reactions, "FUEL", upper_bound, &mut HashMap::new()) < 1_000_000_000_000 {
        upper_bound *= 2;
    }

    while lower_bound < upper_bound {
        let mid = (upper_bound + lower_bound) / 2;
        let ore_count = get_ore_count(reactions, "FUEL", mid, &mut HashMap::new());
        if ore_count < 1_000_000_000_000 {
            lower_bound = mid + 1;
        } else {
            upper_bound = mid;
        }
    }
    lower_bound - 1 // lower_bound is first count > 1tril
}

fn main() {
    let input_str: Vec<_> = BufReader::new(
        File::open(
            env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .map(|l| l.unwrap())
    .collect();

    let reactions = parse_reactions(&input_str);

    println!(
        "Total ore needed for 1 FUEL: {}",
        required_ore_count(&reactions)
    );

    println!(
        "Total FUEL able to be produced: {}",
        get_fuel_count(&reactions)
    );
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EX3_INPUT_STR: &str = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    const EX4_INPUT_STR: &str = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";

    const EX5_INPUT_STR: &str = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";

    #[test]
    fn ex1() {
        let input_str: Vec<_> = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"
            .lines()
            .collect();

        let reactions = parse_reactions(&input_str);
        assert_eq!(required_ore_count(&reactions), 31);
    }

    #[test]
    fn ex2() {
        let input_str: Vec<_> = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"
            .lines()
            .collect();

        let reactions = parse_reactions(&input_str);
        assert_eq!(required_ore_count(&reactions), 165);
    }

    #[test]
    fn ex3() {
        let input_str: Vec<_> = EX3_INPUT_STR.lines().collect();

        let reactions = parse_reactions(&input_str);
        assert_eq!(required_ore_count(&reactions), 13312);
    }

    #[test]
    fn ex4() {
        let input_str: Vec<_> = EX4_INPUT_STR.lines().collect();

        let reactions = parse_reactions(&input_str);
        assert_eq!(required_ore_count(&reactions), 180697);
    }

    #[test]
    fn ex5() {
        let input_str: Vec<_> = EX5_INPUT_STR.lines().collect();

        let reactions = parse_reactions(&input_str);
        assert_eq!(required_ore_count(&reactions), 2210736);
    }

    #[test]
    fn p2_ex3() {
        let input_str: Vec<_> = EX3_INPUT_STR.lines().collect();

        let reactions = parse_reactions(&input_str);
        assert_eq!(get_fuel_count(&reactions), 82892753);
    }

    #[test]
    fn p2_ex4() {
        let input_str: Vec<_> = EX4_INPUT_STR.lines().collect();

        let reactions = parse_reactions(&input_str);
        assert_eq!(get_fuel_count(&reactions), 5586022);
    }

    #[test]
    fn p2_ex5() {
        let input_str: Vec<_> = EX5_INPUT_STR.lines().collect();

        let reactions = parse_reactions(&input_str);
        assert_eq!(get_fuel_count(&reactions), 460664);
    }
}
