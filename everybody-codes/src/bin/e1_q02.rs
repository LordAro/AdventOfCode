use std::fs;
use std::io;

#[derive(Debug, Clone)]
struct Symbol {
    rank: usize,
    id: usize,
    symbol: char,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

#[derive(Debug)]
enum Cmd {
    Add(Symbol, Symbol),
    Swap(usize),
}

fn parse_input(input: &str) -> Vec<Cmd> {
    input
        .lines()
        .map(|l| {
            let mut it = l.split(' ');
            if it.next().unwrap() == "ADD" {
                let id = it.next().unwrap()[3..].parse().unwrap();
                let left_elts: Vec<_> = it.next().unwrap()[5..].split(',').collect();
                let left_rank = left_elts[0][1..].parse().unwrap();
                let left_sym = left_elts[1].chars().next().unwrap();
                let left = Symbol {
                    rank: left_rank,
                    symbol: left_sym,
                    id,
                    left: None,
                    right: None,
                };
                let right_elts: Vec<_> = it.next().unwrap()[6..].split(',').collect();
                let right_rank = right_elts[0][1..].parse().unwrap();
                let right_sym = right_elts[1].chars().next().unwrap();
                let right = Symbol {
                    rank: right_rank,
                    symbol: right_sym,
                    id,
                    left: None,
                    right: None,
                };
                Cmd::Add(left, right)
            } else {
                let id = it.next().unwrap().parse().unwrap();
                Cmd::Swap(id)
            }
        })
        .collect()
}

fn insert_elt(tree: &mut Box<Symbol>, new_node: Box<Symbol>) {
    if tree.rank < new_node.rank {
        if let Some(left) = &mut tree.left {
            insert_elt(left, new_node);
        } else {
            tree.left = Some(new_node);
        }
    } else if tree.rank > new_node.rank {
        if let Some(right) = &mut tree.right {
            insert_elt(right, new_node);
        } else {
            tree.right = Some(new_node);
        }
    } else {
        // input strictly ordered, no need to check for equality
        unreachable!();
    }
}

fn find_elts(tree: &mut Symbol, id: usize) -> Vec<&mut Symbol> {
    let mut res = vec![];
    let mut to_search = vec![tree];
    while let Some(elt) = to_search.pop() {
        // Prevent horrible explosions from nodes with the same ids being part of the same subtree
        if elt.id == id {
            res.push(elt);
        } else {
            if let Some(left) = &mut elt.left {
                to_search.push(left);
            }
            if let Some(right) = &mut elt.right {
                to_search.push(right);
            }
        }
    }
    res
}

fn swap_elt(left_tree: &mut Box<Symbol>, right_tree: &mut Box<Symbol>, id: usize) {
    let l = &mut find_elts(left_tree, id)[0];
    let r = &mut find_elts(right_tree, id)[0];
    // Swap the values
    let l_sym = l.symbol;
    let l_rank = l.rank;
    let r_sym = r.symbol;
    let r_rank = r.rank;
    l.symbol = r_sym;
    l.rank = r_rank;
    r.symbol = l_sym;
    r.rank = l_rank;
}

fn swap_tree(left_tree: &mut Box<Symbol>, right_tree: &mut Box<Symbol>, id: usize) {
    let mut ls = find_elts(left_tree, id);
    let mut rs = find_elts(right_tree, id);
    if ls.len() == 1 && rs.len() == 1 {
        std::mem::swap(ls[0], rs[0]);
    } else if ls.len() == 2 {
        let mut it = ls.into_iter();
        let a = it.next().unwrap();
        let b = it.next().unwrap();
        std::mem::swap(a, b);
    } else if rs.len() == 2 {
        let mut it = rs.into_iter();
        let a = it.next().unwrap();
        let b = it.next().unwrap();
        std::mem::swap(a, b);
    } else {
        unreachable!();
    }
}

fn build_trees<const RECURSIVE_SWAP: bool>(input_syms: &[Cmd]) -> (Box<Symbol>, Box<Symbol>) {
    // roots
    let (mut left_tree, mut right_tree) = match &input_syms[0] {
        Cmd::Add(left, right) => (Box::new(left.clone()), Box::new(right.clone())),
        _ => unreachable!(),
    };

    for cmd in &input_syms[1..] {
        match cmd {
            Cmd::Add(left, right) => {
                insert_elt(&mut left_tree, Box::new(left.clone()));
                insert_elt(&mut right_tree, Box::new(right.clone()));
            }
            Cmd::Swap(id) => {
                if RECURSIVE_SWAP {
                    swap_tree(&mut left_tree, &mut right_tree, *id);
                } else {
                    swap_elt(&mut left_tree, &mut right_tree, *id);
                }
            }
        }
    }

    (left_tree, right_tree)
}

fn get_widest_level(tree: &Symbol) -> Vec<&Symbol> {
    let mut levels = vec![];
    // DFS to maintain order (though I don't think it matters?)
    let mut to_search = vec![(tree, 0)];
    while let Some((elt, lvl)) = to_search.pop() {
        if levels.len() <= lvl {
            levels.push(vec![]); // can be at most 1 out
        }
        levels[lvl].push(elt);
        if let Some(left) = &elt.left {
            to_search.push((left, lvl + 1));
        }
        if let Some(right) = &elt.right {
            to_search.push((right, lvl + 1));
        }
    }
    // cheap and dirty way of getting the first element (closest to the root)
    levels
        .into_iter()
        .min_by_key(|ns| -(ns.len() as isize))
        .unwrap()
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let input = parse_input(&fs::read_to_string(p1_input_filename)?);
    let (left_tree, right_tree) = build_trees::<false>(&input);

    let largest_left_level = get_widest_level(&left_tree);
    let largest_right_level = get_widest_level(&right_tree);
    let p1_lvl_str: String = largest_left_level
        .iter()
        .map(|s| s.symbol)
        .chain(largest_right_level.iter().map(|s| s.symbol))
        .collect();

    let input = parse_input(&fs::read_to_string(p2_input_filename)?);
    let (left_tree, right_tree) = build_trees::<false>(&input);

    let largest_left_level = get_widest_level(&left_tree);
    let largest_right_level = get_widest_level(&right_tree);
    let p2_lvl_str: String = largest_left_level
        .iter()
        .map(|s| s.symbol)
        .chain(largest_right_level.iter().map(|s| s.symbol))
        .collect();

    let input = parse_input(&fs::read_to_string(p3_input_filename)?);
    let (left_tree, right_tree) = build_trees::<true>(&input);

    let largest_left_level = get_widest_level(&left_tree);
    let largest_right_level = get_widest_level(&right_tree);
    let p3_lvl_str: String = largest_left_level
        .iter()
        .map(|s| s.symbol)
        .chain(largest_right_level.iter().map(|s| s.symbol))
        .collect();

    println!("P1: Message: {p1_lvl_str}");
    println!("P2: Message: {p2_lvl_str}");
    println!("P3: Message: {p3_lvl_str}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input_str = "ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]";
        let input = parse_input(input_str);
        let (left_tree, right_tree) = build_trees::<false>(&input);

        let largest_left_level = get_widest_level(&left_tree);
        let largest_right_level = get_widest_level(&right_tree);
        let lvl_str: String = largest_left_level
            .iter()
            .map(|s| s.symbol)
            .chain(largest_right_level.iter().map(|s| s.symbol))
            .collect();
        assert_eq!(lvl_str, "CFGNLK");
    }

    #[test]
    fn ex2() {
        let input_str = "ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
SWAP 1
SWAP 5
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]";
        let input = parse_input(input_str);
        let (left_tree, right_tree) = build_trees::<false>(&input);

        let largest_left_level = get_widest_level(&left_tree);
        let largest_right_level = get_widest_level(&right_tree);
        let lvl_str: String = largest_left_level
            .iter()
            .map(|s| s.symbol)
            .chain(largest_right_level.iter().map(|s| s.symbol))
            .collect();
        assert_eq!(lvl_str, "MGFLNK");
    }

    #[test]
    fn ex3a() {
        let input_str = "ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
SWAP 1
SWAP 5
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]
SWAP 2";
        let input = parse_input(input_str);
        let (left_tree, right_tree) = build_trees::<true>(&input);

        let largest_left_level = get_widest_level(&left_tree);
        let largest_right_level = get_widest_level(&right_tree);
        let lvl_str: String = largest_left_level
            .iter()
            .map(|s| s.symbol)
            .chain(largest_right_level.iter().map(|s| s.symbol))
            .collect();
        assert_eq!(lvl_str, "DJMGL");
    }

    #[test]
    fn ex3b() {
        let input_str = "ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
SWAP 1
SWAP 5
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]
SWAP 2
SWAP 5";
        let input = parse_input(input_str);
        let (left_tree, right_tree) = build_trees::<true>(&input);

        let largest_left_level = get_widest_level(&left_tree);
        let largest_right_level = get_widest_level(&right_tree);
        let lvl_str: String = largest_left_level
            .iter()
            .map(|s| s.symbol)
            .chain(largest_right_level.iter().map(|s| s.symbol))
            .collect();
        assert_eq!(lvl_str, "DJCGL");
    }
}
