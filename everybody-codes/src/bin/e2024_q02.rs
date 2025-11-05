use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::iter;

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    {
        let p1_input: String = fs::read_to_string(p1_input_filename)?;
        let words: Vec<_> = p1_input.lines().next().expect("Unexpected input format")[6..]
            .split(',')
            .collect();

        let sentence = p1_input.lines().nth(2).expect("Unexpected input format");

        let num_runic_words: usize = words
            .iter()
            .map(|word| sentence.matches(word).count())
            .sum();
        println!("P1: Number of runic words: {num_runic_words}");
    }

    {
        let p2_input: String = fs::read_to_string(p2_input_filename)?;

        // strip off 'WORDS:' prefix
        let words: Vec<_> = p2_input.lines().next().expect("Unexpected input format")[6..]
            .split(',')
            .collect();

        let all_words: HashSet<_> = words
            .clone()
            .into_iter()
            .map(String::from)
            .chain(words.iter().map(|s| s.chars().rev().collect()))
            .collect();

        let inscription = p2_input.lines().skip(2).join("\n");

        let matched_indices =
            all_words
                .iter()
                .fold(HashSet::<usize>::new(), |mut indices, word| {
                    // Can't use match_indices due to words that overlap with themselves (e.g. OWO & OWOWO)
                    for i in 0..inscription.len() {
                        if i + word.len() <= inscription.len()
                            && inscription[i..i + word.len()] == *word
                        {
                            indices.extend(i..i + word.len());
                        }
                    }
                    indices
                });

        let num_runic_symbols = matched_indices.len();
        println!("P2: Number of runic symbols: {num_runic_symbols}");
    }

    {
        let p3_input: String = fs::read_to_string(p3_input_filename)?;
        //        let p3_input: String = "WORDS:THE,OWE,MES,ROD,RODEO
        //
        //HELWORLT
        //ENIGWDXL
        //TRODEOAL
        //"
        //        .to_string();

        // strip off 'WORDS:' prefix
        let words: Vec<_> = p3_input.lines().next().expect("Unexpected input format")[6..]
            .split(',')
            .collect();

        let all_words: HashSet<_> = words
            .clone()
            .into_iter()
            .map(String::from)
            .chain(words.iter().map(|s| s.chars().rev().collect()))
            .collect();

        let inscription: Vec<Vec<char>> = p3_input
            .lines()
            .skip(2)
            .map(|l| l.chars().collect())
            .collect();

        let mut matched_scales = HashSet::<(usize, usize)>::new();
        for y in 0..inscription.len() {
            for x in 0..inscription[y].len() {
                for word in &all_words {
                    if inscription[y][x] != word.chars().next().expect("No empty strings") {
                        // minor short circuit
                        continue;
                    }
                    let ltr_coords: Vec<_> = iter::repeat_n(y, word.len())
                        .zip((x..x + word.len()).map(|x2| x2 % inscription[y].len()))
                        .collect();

                    let ltr_word: String = ltr_coords
                        .iter()
                        .map(|(y, x)| inscription[*y][*x])
                        .collect();
                    if ltr_word == *word {
                        matched_scales.extend(ltr_coords.iter());
                    }

                    // top and bottom don't wrap round
                    if y + word.len() <= inscription.len() {
                        let ttb_coords: Vec<_> = (y..y + word.len())
                            .zip(iter::repeat_n(x, word.len()))
                            .collect();
                        let ttb_word: String = ttb_coords
                            .iter()
                            .map(|(y, x)| inscription[*y][*x])
                            .collect();
                        if ttb_word == *word {
                            matched_scales.extend(ttb_coords.iter());
                        }
                    }
                }
            }
        }

        println!("P3: Total scales: {}", matched_scales.len());
    }
    Ok(())
}
