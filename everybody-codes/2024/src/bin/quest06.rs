use std::collections::HashMap;
use std::fs;
use std::io;

type Tree = HashMap<String, Vec<String>>;

fn parse_tree(input: &str) -> Tree {
    input
        .lines()
        .map(|line| {
            let mut root_branch = line.split(':');
            let root = root_branch.next().unwrap().to_string();
            let branches: Vec<_> = root_branch
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.to_string())
                .collect();
            (root, branches)
        })
        .collect()
}

fn get_unique_fruit_path(tree: &Tree) -> Vec<&str> {
    let mut complete_paths = vec![];
    let mut paths = vec![vec!["RR"]];
    while !paths.is_empty() {
        let path = paths.remove(0);
        let last_node = path.last().unwrap();
        if *last_node == "@" {
            complete_paths.push(path);
            continue;
        }

        if !tree.contains_key(*last_node) {
            // path ends without '@', throw it away
            continue;
        }
        for branch in tree.get(*last_node).unwrap() {
            if path.contains(&branch.as_str()) {
                // loop, continue
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(branch);
            paths.push(new_path);
        }
    }
    complete_paths.sort_by_key(|path| path.len());
    complete_paths
        .chunk_by(|a, b| a.len() == b.len())
        .find(|chunk| chunk.len() == 1)
        .unwrap()[0]
        .clone()
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_tree = parse_tree(&fs::read_to_string(p1_input_filename)?);
    println!(
        "P1: Path to most powerful fruit: {}",
        get_unique_fruit_path(&p1_tree).concat()
    );

    let p2_tree = parse_tree(&fs::read_to_string(p2_input_filename)?);
    println!(
        "P2: Path to most powerful fruit: {}",
        get_unique_fruit_path(&p2_tree)
            .iter()
            .map(|n| n.chars().next().unwrap())
            .collect::<String>()
    );

    let p3_tree = parse_tree(&fs::read_to_string(p3_input_filename)?);
    println!(
        "P3: Path to most powerful fruit: {}",
        get_unique_fruit_path(&p3_tree)
            .iter()
            .map(|n| n.chars().next().unwrap())
            .collect::<String>()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input_str = "RR:A,B,C
A:D,E
B:F,@
C:G,H
D:@
E:@
F:@
G:@
H:@";
        let tree = parse_tree(input_str);
        assert_eq!(get_unique_fruit_path(&tree).concat(), "RRB@");
    }
}
