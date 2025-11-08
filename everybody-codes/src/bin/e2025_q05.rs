use std::fs;
use std::io;

#[derive(Debug)]
struct Node {
    num: usize,
    left: Option<usize>,
    right: Option<usize>,
}

fn build_fishbone(nums: &[usize]) -> Vec<Node> {
    let mut bone: Vec<Node> = vec![];
    'outer: for &n in nums {
        for spine in &mut bone {
            if n < spine.num && spine.left.is_none() {
                spine.left = Some(n);
                continue 'outer;
            } else if n > spine.num && spine.right.is_none() {
                spine.right = Some(n);
                continue 'outer;
            }
        }
        // Got to the end of the spine, add the number here
        bone.push(Node {
            num: n,
            left: None,
            right: None,
        });
    }
    bone
}

fn parse_fishbone(input: &str) -> (usize, Vec<Node>) {
    let (id, bone_str) = input.split_once(':').unwrap();
    let nums: Vec<_> = bone_str.split(',').map(|n| n.parse().unwrap()).collect();

    let bone = build_fishbone(&nums);
    (id.parse().unwrap(), bone)
}

fn num_digits(n: usize) -> u32 {
    if n > 0 { n.ilog10() + 1 } else { 1 }
}

// real inputs don't appear to use values > 9, but the example input does, so ilog it is
fn get_quality(bone: &[Node]) -> usize {
    bone.iter()
        .fold(0, |acc, s| acc * 10usize.pow(num_digits(s.num)) + s.num)
}

fn get_node_num(node: &Node) -> usize {
    [node.left, Some(node.num), node.right]
        .into_iter()
        .flatten()
        .fold(0, |acc, s| acc * 10usize.pow(num_digits(s)) + s)
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    let p1_input_str = fs::read_to_string(p1_input_filename)?;
    let p1_bone = parse_fishbone(&p1_input_str).1;
    let p1_quality = get_quality(&p1_bone);

    let p2_input = fs::read_to_string(p2_input_filename)?;
    let p2_bone_qualities: Vec<_> = p2_input
        .lines()
        .map(|l| get_quality(&parse_fishbone(l).1))
        .collect();
    let p2_worst_bone = p2_bone_qualities.iter().min().unwrap();
    let p2_best_bone = p2_bone_qualities.iter().max().unwrap();
    let p2_bone_diff = p2_best_bone - p2_worst_bone;

    let p3_input = fs::read_to_string(p3_input_filename)?;
    let mut p3_bones: Vec<_> = p3_input.lines().map(parse_fishbone).collect();
    // Great big sort function! Should probably be defined on a type, but...
    // Note the reverse order
    p3_bones.sort_by(|(a_id, a_bone), (b_id, b_bone)| {
        get_quality(b_bone)
            .cmp(&get_quality(a_bone))
            .then_with(|| {
                b_bone
                    .iter()
                    .map(get_node_num)
                    .cmp(a_bone.iter().map(get_node_num))
            })
            .then(b_id.cmp(a_id))
    });
    let p3_bone_checksum: usize = p3_bones
        .iter()
        .enumerate()
        .map(|(pos, (id, _))| (pos + 1) * id)
        .sum();

    println!("P1: Quality of fishbone: {p1_quality}");
    println!("P2: Bone quality difference: {p2_bone_diff}");
    println!("P3: Bone checksum: {p3_bone_checksum}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input = vec![5, 3, 7, 8, 9, 10, 4, 5, 7, 8, 8];
        let bone = build_fishbone(&input);
        assert_eq!(get_quality(&bone), 581078);
    }
}
