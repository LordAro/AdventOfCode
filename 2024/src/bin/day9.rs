use std::env;
use std::fs;
use std::io;
use std::iter;

fn expand_disk_map(disk_map: &str) -> Vec<usize> {
    let mut disk_layout: Vec<usize> = Vec::new();
    let mut id = 0;
    for (n, d) in disk_map
        .chars()
        .enumerate()
        .filter_map(|(n, c)| c.to_digit(10).map(|d| (n, d as usize)))
    {
        if n % 2 == 0 {
            disk_layout.extend(iter::repeat(id).take(d));
            id += 1;
        } else {
            disk_layout.extend(iter::repeat(usize::MAX).take(d));
        }
    }
    disk_layout
}

fn compress_disk(disk: &mut Vec<usize>) {
    let mut i = 0;
    while i < disk.len() {
        if disk[i] == usize::MAX {
            disk.swap_remove(i); // replaces with element from the end
        } else {
            i += 1;
        }
    }
}

fn compress_disk_no_fragment(disk: &mut Vec<usize>) {
    let mut partitioned_disk: Vec<_> = disk.chunk_by(|a, b| a == b).collect();

    let mut partition_to_place = partitioned_disk.len() - 1;
    while partition_to_place > 0 {
        let partition_len = partitioned_disk[partition_to_place].len();
        if partitioned_disk[partition_to_place][0] == usize::MAX {
            partition_to_place -= 1;
            continue;
        }

        let mut possible_placement_idx = 0;
        while possible_placement_idx < partition_to_place {
            let possible_placement_len = partitioned_disk[possible_placement_idx].len();
            // This can create multiple contiguous empty partitions but because it's only a
            // single pass this is ok and doesn't cause any issues
            if partitioned_disk[possible_placement_idx][0] == usize::MAX
                && possible_placement_len >= partition_len
            {
                partitioned_disk.swap(possible_placement_idx, partition_to_place);
                if possible_placement_len > partition_len {
                    partitioned_disk.insert(
                        possible_placement_idx + 1,
                        &partitioned_disk[partition_to_place][partition_len..],
                    );
                    // adjust index
                    partition_to_place += 1;
                    // shorten the space we just swapped out
                    partitioned_disk[partition_to_place] =
                        &partitioned_disk[partition_to_place][0..partition_len];
                }
                break;
            }
            possible_placement_idx += 1;
        }
        partition_to_place -= 1;
    }
    *disk = partitioned_disk
        .into_iter()
        .flatten()
        .copied()
        .collect::<Vec<_>>();
}

fn disk_checksum(disk: &[usize]) -> usize {
    disk.iter()
        .enumerate()
        .filter(|(_, id)| **id != usize::MAX)
        .map(|(n, id)| n * *id)
        .sum()
}

fn main() -> io::Result<()> {
    let disk_map = fs::read_to_string(env::args().nth(1).expect("missing cli argument"))?;

    let mut p1_disk_layout = expand_disk_map(&disk_map);
    let mut p2_disk_layout = p1_disk_layout.clone();

    compress_disk(&mut p1_disk_layout);
    println!("P1: Disk checksum: {}", disk_checksum(&p1_disk_layout));
    compress_disk_no_fragment(&mut p2_disk_layout);
    println!("P2: Disk checksum: {}", disk_checksum(&p2_disk_layout));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input = "12345";
        let mut layout = expand_disk_map(input);
        assert_eq!(
            layout,
            [
                0,
                usize::MAX,
                usize::MAX,
                1,
                1,
                1,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                usize::MAX,
                2,
                2,
                2,
                2,
                2
            ]
        );
        compress_disk(&mut layout);
        assert_eq!(layout, [0, 2, 2, 1, 1, 1, 2, 2, 2]);
    }

    #[test]
    fn ex1b() {
        let input = "2333133121414131402";
        let mut layout = expand_disk_map(input);
        compress_disk(&mut layout);
        assert_eq!(disk_checksum(&layout), 1928);
    }

    #[test]
    fn ex2() {
        let input = "2333133121414131402";
        let mut layout = expand_disk_map(input);
        compress_disk_no_fragment(&mut layout);
        assert_eq!(disk_checksum(&layout), 2858);
    }
}
