extern crate regex;

use regex::Regex;
use std::collections::hash_map::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum BotOutput {
    Bot(i32),
    Output(i32),
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments provided\n");
    }

    let input = BufReader::new(File::open(&env::args().nth(1).unwrap()).unwrap());

    let initial_re = Regex::new(r"value ([0-9]+) goes to bot ([0-9]+)").unwrap();
    let give_re = Regex::new(
        r"bot ([0-9]+) gives low to (bot|output) ([0-9]+) and high to (bot|output) ([0-9]+)",
    )
    .unwrap();

    // Uses a Vec to store only 2 items, for simplicity
    let mut bots: HashMap<i32, Vec<i32>> = HashMap::new();
    // A move uses the bot's number as the key, then a pair
    // of the bot/output (lower followed by higher)
    let mut bot_moves: HashMap<i32, (BotOutput, BotOutput)> = HashMap::new();
    // outputs only ever hold one item
    let mut outputs: HashMap<i32, i32> = HashMap::new();

    // Parse input into something meaningful
    for l in input.lines() {
        let line = l.unwrap();
        if let Some(caps) = initial_re.captures(&line) {
            let v: i32 = caps.at(1).unwrap().parse().unwrap();
            let b: i32 = caps.at(2).unwrap().parse().unwrap();
            bots.entry(b).or_default().push(v);
        } else if let Some(caps) = give_re.captures(&line) {
            let b: i32 = caps.at(1).unwrap().parse().unwrap();
            let l_num = caps.at(3).unwrap().parse().unwrap();
            let l = if caps.at(2).unwrap() == "output" {
                BotOutput::Output(l_num)
            } else {
                BotOutput::Bot(l_num)
            };

            let h_num = caps.at(5).unwrap().parse().unwrap();
            let h = if caps.at(4).unwrap() == "output" {
                BotOutput::Output(h_num)
            } else {
                BotOutput::Bot(h_num)
            };
            bot_moves.insert(b, (l, h));
        }
    }

    while let Some(&botnum) = bots.iter().find(|&(_, v)| v.len() == 2).map(|t| t.0) {
        let bot_move = bot_moves.get(&botnum).unwrap();
        let botitems = bots[&botnum].clone();
        let l_item = botitems.iter().min().unwrap();
        let h_item = botitems.iter().max().unwrap();
        if *h_item == 61 && *l_item == 17 {
            println!("Bot responsible: {}", botnum);
        }
        match bot_move.0 {
            // low
            BotOutput::Bot(b) => bots.entry(b).or_default().push(*l_item),
            BotOutput::Output(o) => {
                let _ = outputs.insert(o, *l_item);
            }
        }
        match bot_move.1 {
            // high
            BotOutput::Bot(b) => bots.entry(b).or_default().push(*h_item),
            BotOutput::Output(o) => {
                let _ = outputs.insert(o, *h_item);
            }
        }
        bots.get_mut(&botnum).unwrap().clear();
    }

    let product = outputs.get(&0).unwrap() * outputs.get(&1).unwrap() * outputs.get(&2).unwrap();
    println!("Output product total: {}", product);
}
