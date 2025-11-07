use std::fs;
use std::io;
use std::mem;

#[derive(Debug, Copy, Clone)]
struct Symbol {
    rank: usize,
    id: usize,
    symbol: char,
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
                };
                let right_elts: Vec<_> = it.next().unwrap()[6..].split(',').collect();
                let right_rank = right_elts[0][1..].parse().unwrap();
                let right_sym = right_elts[1].chars().next().unwrap();
                let right = Symbol {
                    rank: right_rank,
                    symbol: right_sym,
                    id,
                };
                Cmd::Add(left, right)
            } else {
                let id = it.next().unwrap().parse().unwrap();
                Cmd::Swap(id)
            }
        })
        .collect()
}

fn overwrite_elt(tree: &mut Vec<Option<Symbol>>, idx: usize, new_node: Option<Symbol>) {
    tree.resize(tree.len().max(idx + 1), None);
    tree[idx] = new_node;
}

fn insert_elt(tree: &mut Vec<Option<Symbol>>, parent_idx: usize, new_node: Symbol) {
    // input strictly ordered, no need to check for equality
    let child_idx = if new_node.rank < tree[parent_idx].unwrap().rank {
        2 * parent_idx + 1
    } else {
        2 * parent_idx + 2
    };

    if tree.len() > child_idx && tree[child_idx].is_some() {
        insert_elt(tree, child_idx, new_node);
    } else {
        overwrite_elt(tree, child_idx, Some(new_node));
    }
}

fn swap_elt(l_tree: &mut [Option<Symbol>], r_tree: &mut [Option<Symbol>], id: usize) {
    let l_node_idx = l_tree
        .iter()
        .position(|n| n.is_some_and(|v| v.id == id))
        .unwrap();
    let r_node_idx = r_tree
        .iter()
        .position(|n| n.is_some_and(|v| v.id == id))
        .unwrap();
    mem::swap(&mut l_tree[l_node_idx], &mut r_tree[r_node_idx]);
}

fn extract_subtree(tree: &mut [Option<Symbol>], idx: usize) -> Vec<Option<Symbol>> {
    let mut subtree = Vec::new();
    let mut to_search = vec![(idx, 0)];
    while let Some((cur_ix, new_ix)) = to_search.pop() {
        if cur_ix < tree.len() && tree[cur_ix].is_some() {
            overwrite_elt(&mut subtree, new_ix, tree[cur_ix]);
            tree[cur_ix] = None;
            to_search.push((2 * cur_ix + 1, 2 * new_ix + 1));
            to_search.push((2 * cur_ix + 2, 2 * new_ix + 2));
        }
    }
    subtree
}

fn enplace_subtree(
    tree: &mut Vec<Option<Symbol>>,
    subtree: &[Option<Symbol>],
    tree_ix: usize,
    sub_ix: usize,
) {
    if sub_ix < subtree.len() && subtree[sub_ix].is_some() {
        overwrite_elt(tree, tree_ix, subtree[sub_ix]);
        enplace_subtree(tree, subtree, 2 * tree_ix + 1, 2 * sub_ix + 1);
        enplace_subtree(tree, subtree, 2 * tree_ix + 2, 2 * sub_ix + 2);
    }
}

fn swap_elt_recursive(
    l_tree: &mut Vec<Option<Symbol>>,
    r_tree: &mut Vec<Option<Symbol>>,
    id: usize,
) {
    let l_node_idxs: Vec<_> = l_tree
        .iter()
        .enumerate()
        .filter(|(_, n)| n.is_some_and(|v| v.id == id))
        .map(|(i, _)| i)
        .collect();
    let r_node_idxs: Vec<_> = r_tree
        .iter()
        .enumerate()
        .filter(|(_, n)| n.is_some_and(|v| v.id == id))
        .map(|(i, _)| i)
        .collect();

    if l_node_idxs.len() > 1 {
        // Swaps have meant both nodes with same id have ended up on same subtree.
        // Needs some thought (what if one is now an ancestor of the other??)
        let l1_extracted_subtree = extract_subtree(l_tree, l_node_idxs[0]);
        assert!(l_tree[l_node_idxs[1]].is_some()); // shouldn't have been removed by extract
        let l2_extracted_subtree = extract_subtree(l_tree, l_node_idxs[1]);
        enplace_subtree(l_tree, &l1_extracted_subtree, l_node_idxs[1], 0);
        enplace_subtree(l_tree, &l2_extracted_subtree, l_node_idxs[0], 0);
    } else if r_node_idxs.len() > 1 {
        let r1_extracted_subtree = extract_subtree(r_tree, r_node_idxs[0]);
        assert!(r_tree[r_node_idxs[1]].is_some()); // shouldn't have been removed by extract
        let r2_extracted_subtree = extract_subtree(r_tree, r_node_idxs[1]);
        enplace_subtree(r_tree, &r1_extracted_subtree, r_node_idxs[1], 0);
        enplace_subtree(r_tree, &r2_extracted_subtree, r_node_idxs[0], 0);
    } else if l_node_idxs.len() == 1 && r_node_idxs.len() == 1 {
        let l_extracted_subtree = extract_subtree(l_tree, l_node_idxs[0]);
        let r_extracted_subtree = extract_subtree(r_tree, r_node_idxs[0]);
        enplace_subtree(r_tree, &l_extracted_subtree, r_node_idxs[0], 0);
        enplace_subtree(l_tree, &r_extracted_subtree, l_node_idxs[0], 0);
    } else {
        unreachable!();
    }
}

fn build_trees<const RECURSIVE_SWAP: bool>(
    input_syms: &[Cmd],
) -> (Vec<Option<Symbol>>, Vec<Option<Symbol>>) {
    let mut left_tree = Vec::new();
    let mut right_tree = Vec::new();
    match input_syms[0] {
        Cmd::Add(left, right) => {
            left_tree.push(Some(left));
            right_tree.push(Some(right));
        }
        _ => unreachable!(),
    }

    for cmd in &input_syms[1..] {
        match cmd {
            Cmd::Add(left, right) => {
                insert_elt(&mut left_tree, 0, *left);
                insert_elt(&mut right_tree, 0, *right);
            }
            Cmd::Swap(id) => {
                if RECURSIVE_SWAP {
                    swap_elt_recursive(&mut left_tree, &mut right_tree, *id);
                } else {
                    swap_elt(&mut left_tree, &mut right_tree, *id);
                }
            }
        }
    }

    (left_tree, right_tree)
}

fn get_nodes_at_level(tree: &[Option<Symbol>], level: u32) -> Vec<Symbol> {
    let start_index = 2_usize.pow(level) - 1;
    if start_index > tree.len() {
        return vec![];
    }
    let end_index = 2_usize.pow(level + 1) - 2;

    tree[start_index..tree.len().min(end_index + 1)]
        .iter()
        .flatten()
        .copied()
        .collect()
}

fn get_largest_level(tree: &[Option<Symbol>]) -> Vec<Symbol> {
    (0..)
        .map(|lvl| get_nodes_at_level(tree, lvl))
        .take_while(|ns| !ns.is_empty())
        // cheap and dirty way of getting the first element (closest to the root)
        .min_by_key(|ns| -(ns.len() as isize))
        .unwrap()
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let input = parse_input(&fs::read_to_string(p1_input_filename)?);
    let (left_tree, right_tree) = build_trees::<false>(&input);

    let largest_left_level = get_largest_level(&left_tree);
    let largest_right_level = get_largest_level(&right_tree);
    let p1_lvl_str: String = largest_left_level
        .iter()
        .map(|s| s.symbol)
        .chain(largest_right_level.iter().map(|s| s.symbol))
        .collect();

    let input = parse_input(&fs::read_to_string(p2_input_filename)?);
    let (left_tree, right_tree) = build_trees::<false>(&input);

    let largest_left_level = get_largest_level(&left_tree);
    let largest_right_level = get_largest_level(&right_tree);
    let p2_lvl_str: String = largest_left_level
        .iter()
        .map(|s| s.symbol)
        .chain(largest_right_level.iter().map(|s| s.symbol))
        .collect();

    let input = parse_input(&fs::read_to_string(p3_input_filename)?);
    let (left_tree, right_tree) = build_trees::<true>(&input);

    let largest_left_level = get_largest_level(&left_tree);
    let largest_right_level = get_largest_level(&right_tree);
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

        let largest_left_level = get_largest_level(&left_tree);
        let largest_right_level = get_largest_level(&right_tree);
        let lvl_str: String = largest_left_level
            .iter()
            .map(|s| s.symbol)
            .chain(largest_right_level.iter().map(|s| s.symbol))
            .collect();
        assert_eq!(lvl_str, "CFGNLK");
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

        let largest_left_level = get_largest_level(&left_tree);
        let largest_right_level = get_largest_level(&right_tree);
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

        let largest_left_level = get_largest_level(&left_tree);
        let largest_right_level = get_largest_level(&right_tree);
        let lvl_str: String = largest_left_level
            .iter()
            .map(|s| s.symbol)
            .chain(largest_right_level.iter().map(|s| s.symbol))
            .collect();
        assert_eq!(lvl_str, "DJCGL");
    }
}
