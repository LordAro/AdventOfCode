use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            let (_, scale) = l.split_once(':').unwrap();
            scale.bytes().collect()
        })
        .collect()
}

fn get_similarity(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b.iter()).filter(|(a, b)| a == b).count()
}

fn is_child_of(child: &[u8], parent1: &[u8], parent2: &[u8]) -> bool {
    itertools::izip!(child, parent1, parent2).all(|(c, p1, p2)| c == p1 || c == p2)
}

fn group_families(
    child_to_parent: &HashMap<usize, (usize, usize)>,
    parent_to_child: &HashMap<usize, Vec<usize>>,
) -> Vec<HashSet<usize>> {
    let mut families: Vec<HashSet<usize>> = vec![];
    for child in child_to_parent.keys() {
        if families.iter().any(|fam| fam.contains(child)) {
            // already placed
            continue;
        }

        let mut family = HashSet::new();
        let mut to_search: Vec<usize> = vec![*child];
        while let Some(idx) = to_search.pop() {
            if family.contains(&idx) {
                // avoid loops
                continue;
            }
            if let Some((p1, p2)) = child_to_parent.get(&idx) {
                to_search.push(*p1);
                to_search.push(*p2);
            }
            if let Some(children) = parent_to_child.get(&idx) {
                to_search.extend(children);
            }
            family.insert(idx);
        }
        families.push(family);
    }

    families
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_input = parse_input(&fs::read_to_string(p1_input_filename)?);

    let p1_p1 = get_similarity(&p1_input[0], &p1_input[2]);
    let p1_p2 = get_similarity(&p1_input[1], &p1_input[2]);

    let p2_input = parse_input(&fs::read_to_string(p2_input_filename)?);
    let mut p2_total_similarity = 0;
    for ci in 0..p2_input.len() {
        for p1 in 0..p2_input.len() - 1 {
            if p1 == ci {
                continue;
            }
            for p2 in p1 + 1..p2_input.len() {
                if p2 == ci {
                    continue;
                }
                if is_child_of(&p2_input[ci], &p2_input[p1], &p2_input[p2]) {
                    p2_total_similarity += get_similarity(&p2_input[ci], &p2_input[p1])
                        * get_similarity(&p2_input[ci], &p2_input[p2]);
                }
            }
        }
    }

    // create family trees
    let p3_input = parse_input(&fs::read_to_string(p3_input_filename)?);
    let mut p3_child_to_parent = HashMap::<usize, (usize, usize)>::new();
    let mut p3_parent_to_child = HashMap::<usize, Vec<usize>>::new();
    for ci in 0..p3_input.len() {
        for p1 in 0..p3_input.len() - 1 {
            if p1 == ci {
                continue;
            }
            for p2 in p1 + 1..p3_input.len() {
                if p2 == ci {
                    continue;
                }
                if is_child_of(&p3_input[ci], &p3_input[p1], &p3_input[p2]) {
                    p3_child_to_parent.insert(ci, (p1, p2));
                    p3_parent_to_child
                        .entry(p1)
                        .and_modify(|e| e.push(ci))
                        .or_insert(vec![ci]);
                    p3_parent_to_child
                        .entry(p2)
                        .and_modify(|e| e.push(ci))
                        .or_insert(vec![ci]);
                }
            }
        }
    }

    let p3_families = group_families(&p3_child_to_parent, &p3_parent_to_child);
    let p3_largest_family_scale_num: usize = p3_families
        .iter()
        .max_by_key(|fam| fam.len())
        .map(|fam| fam.iter().map(|f| f + 1).sum())
        .unwrap();

    println!("P1: Degree of similarity: {}", p1_p1 * p1_p2);
    println!("P2: Total degrees of similarity: {p2_total_similarity}");
    println!("P3: Scale number of largest family: {p3_largest_family_scale_num}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input1 = "CAAGCGCTAAGTTCGCTGGATGTGTGCCCGCG".as_bytes();
        let input2 = "CTTGAATTGGGCCGTTTACCTGGTTTAACCAT".as_bytes();
        let input3 = "CTAGCGCTGAGCTGGCTGCCTGGTTGACCGCG".as_bytes();

        assert_eq!(get_similarity(input1, input3), 23);
        assert_eq!(get_similarity(input2, input3), 18);
    }
}
