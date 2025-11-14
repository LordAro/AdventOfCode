use std::collections::HashMap;
use std::fs;
use std::io;

fn parse_input(input: &str) -> (Vec<&str>, HashMap<u8, Vec<u8>>) {
    let mut it = input.lines();
    let names = it.next().unwrap().split(',').collect();
    let letter_mapping = it
        .skip(1)
        .map(|l| {
            let (k, val_str) = l.split_once(" > ").unwrap();
            (
                k.bytes().next().unwrap(),
                val_str.bytes().filter(|c| *c != b',').collect::<Vec<_>>(),
            )
        })
        .collect();

    (names, letter_mapping)
}

fn does_name_match(name: &str, mapping: &HashMap<u8, Vec<u8>>) -> bool {
    for c in name.as_bytes().windows(2) {
        let a = c[0];
        let b = c[1];
        if !mapping.get(&a).is_some_and(|v| v.contains(&b)) {
            return false;
        }
    }
    true
}

fn count_unique_names(prefix: &str, mapping: &HashMap<u8, Vec<u8>>) -> usize {
    // Only need to keep track of the last byte
    // (and hope that there aren't any converging mappings at any point in the input,
    // as the solutions will then not be unique)
    //   A
    //  / \
    //  B C
    //  \ /
    //   D
    let mut to_search = vec![(prefix.as_bytes().last().unwrap(), prefix.len())];
    let mut count = 0;
    while let Some((last_b, len)) = to_search.pop() {
        if (7..=11).contains(&len) {
            count += 1;
        }
        if len < 11
            && let Some(v) = mapping.get(last_b)
        {
            for next_b in v {
                to_search.push((next_b, len + 1));
            }
        }
    }
    count
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_input = fs::read_to_string(p1_input_filename)?;
    let (p1_names, p1_mapping) = parse_input(&p1_input);
    let p1_name = p1_names
        .iter()
        .find(|name| does_name_match(name, &p1_mapping))
        .unwrap();

    let p2_input = fs::read_to_string(p2_input_filename)?;
    let (p2_names, p2_mapping) = parse_input(&p2_input);
    let p2_name_sum: usize = p2_names
        .iter()
        .enumerate()
        .filter(|(_, n)| does_name_match(n, &p2_mapping))
        .map(|(i, _)| i + 1)
        .sum();

    let p3_input = fs::read_to_string(p3_input_filename)?;
    let (p3_names, p3_mapping) = parse_input(&p3_input);
    let p3_unique_name_count: usize = p3_names
        .iter()
        .filter(|prefix| does_name_match(prefix, &p3_mapping))
        // Exclude any (valid) prefixes that are covered by shorter ones
        .filter(|prefix| {
            !p3_names
                .iter()
                .any(|n| prefix.len() > n.len() && prefix.starts_with(*n))
        })
        .map(|prefix| count_unique_names(prefix, &p3_mapping))
        .sum();

    println!("P1: Can create this name: {p1_name}");
    println!("P2: Name index sum: {p2_name_sum}");
    println!("P3: Number of unique names: {p3_unique_name_count}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex3a() {
        let input = "Xaryt

X > a,o
a > r,t
r > y,e,a
h > a,e,v
t > h
v > e
y > p,t";
        let (names, mapping) = parse_input(input);
        assert_eq!(count_unique_names(names[0], &mapping), 25);
    }

    #[test]
    fn ex3b() {
        let input = "Khara,Xaryt,Noxer,Kharax

r > v,e,a,g,y
a > e,v,x,r,g
e > r,x,v,t
h > a,e,v
g > r,y
y > p,t
i > v,r
K > h
v > e
B > r
t > h
N > e
p > h
H > e
l > t
z > e
X > a
n > v
x > z
T > i";
        let (names, mapping) = parse_input(input);
        let unique_name_count: usize = names
            .iter()
            .filter(|prefix| does_name_match(prefix, &mapping))
            .filter(|prefix| {
                !names
                    .iter()
                    .any(|n| prefix.len() > n.len() && prefix.starts_with(*n))
            })
            .map(|prefix| count_unique_names(prefix, &mapping))
            .sum();
        assert_eq!(unique_name_count, 1154);
    }
}
