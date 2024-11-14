use std::fs;
use std::io;

fn potion_requirement(c: char, count: usize) -> usize {
    (match c {
        'A' => 0,
        'B' => 1,
        'C' => 3,
        'D' => 5,
        'x' => 0,
        _ => unreachable!(),
    }) + if c != 'x' { count.saturating_sub(1) } else { 0 }
}

fn required_potions<const GROUP_SIZE: usize>(creatures: &[char]) -> usize {
    creatures
        .chunks_exact(GROUP_SIZE)
        .map(|creature_team| {
            let count_creatures = creature_team.iter().filter(|c| **c != 'x').count();
            creature_team
                .iter()
                .map(|c| potion_requirement(*c, count_creatures))
                .sum::<usize>()
        })
        .sum()
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) = everybody_codes::get_input_files()?;

    let p1_input: Vec<_> = fs::read_to_string(p1_input_filename)?
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect();

    let p2_input: Vec<_> = fs::read_to_string(p2_input_filename)?
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect();

    let p3_input: Vec<_> = fs::read_to_string(p3_input_filename)?
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect();

    let potion_count_p1 = required_potions::<1>(&p1_input);
    println!("P1: Required potions: {potion_count_p1}");
    let potion_count_p2 = required_potions::<2>(&p2_input);
    println!("P2: Required potions: {potion_count_p2}");
    let potion_count_p3 = required_potions::<3>(&p3_input);
    println!("P3: Required potions: {potion_count_p3}");
    Ok(())
}
