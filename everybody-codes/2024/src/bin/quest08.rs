use memoise::memoise_map;
use std::fs;
use std::io;

fn get_missing_and_width(block_supply: usize) -> (usize, usize) {
    let missing_blocks = (1..).map(|s| s * s).find(|s| *s >= block_supply).unwrap() - block_supply;
    let final_width = (1..)
        .map(|s| s * s)
        .take_while(|s| *s < block_supply)
        .count()
        * 2
        + 1;
    (missing_blocks, final_width)
}

fn get_fancy_missing_and_width(
    block_supply: usize,
    priests: usize,
    acolytes: usize,
) -> (usize, usize) {
    let mut block_sum = 1;
    let mut prev_layer = 1;
    let mut final_layer_width = 0;
    for layer_n in 2.. {
        let layer_width = (layer_n - 1) * 2 + 1;
        let next_layer = (prev_layer * priests) % acolytes;
        prev_layer = next_layer;
        block_sum += next_layer * layer_width;
        if block_sum > block_supply {
            final_layer_width = layer_width;
            break;
        }
    }
    (block_sum - block_supply, final_layer_width)
}

#[memoise_map(layer_n, priests, acolytes)]
fn pyramid_outer_columns(layer_n: usize, priests: usize, acolytes: usize) -> usize {
    if layer_n == 1 {
        return 1;
    }
    ((pyramid_outer_columns(layer_n - 1, priests, acolytes) * priests) % acolytes) + acolytes
}

#[memoise_map(n, k, priests, acolytes)]
fn get_pyramid_layer(n: usize, k: usize, priests: usize, acolytes: usize) -> usize {
    let layer_width = (n - 1) * 2 + 1;
    if k == 1 || k == layer_width {
        return pyramid_outer_columns(n, priests, acolytes);
    }
    pyramid_outer_columns(n, priests, acolytes) + get_pyramid_layer(n - 1, k - 1, priests, acolytes)
}

fn get_fancier_total(block_supply: usize, priests: usize, acolytes: usize) -> usize {
    (1..)
        .map(|layer_n| {
            let layer_width = (layer_n - 1) * 2 + 1;

            // Get shrine totals
            (0..layer_width)
                .map(|k| {
                    let column_total = get_pyramid_layer(layer_n, k + 1, priests, acolytes);

                    // Now work out how many blocks to remove
                    if k == 0 || k == layer_width - 1 {
                        return column_total;
                    }
                    let blocks_to_remove = (priests * layer_width * column_total) % acolytes;
                    //println!("{priests} * {layer_width} * {column_total} mod {acolytes} -> {blocks_to_remove}");
                    column_total - blocks_to_remove
                })
                .sum()
        })
        .find(|&spaced_layer_total| spaced_layer_total > block_supply)
        .unwrap()
}

fn main() -> io::Result<()> {
    let (p1_input_filename, p2_input_filename, p3_input_filename) =
        everybody_codes::get_input_files()?;

    {
        let p1_num: usize = fs::read_to_string(p1_input_filename)?.parse().unwrap();
        let (missing_blocks, final_width) = get_missing_and_width(p1_num);
        println!(
            "P1: Missing blocks: {missing_blocks}, Final width: {final_width} ({})",
            missing_blocks * final_width
        );
    }

    {
        let p2_priests: usize = fs::read_to_string(p2_input_filename)?.parse().unwrap();
        let (missing_blocks, final_width) = get_fancy_missing_and_width(20240000, p2_priests, 1111);
        println!(
            "P2: Missing blocks: {missing_blocks}, Final width: {final_width} ({})",
            missing_blocks * final_width
        );
    }

    {
        let p3_priests: usize = fs::read_to_string(p3_input_filename)?.parse().unwrap();
        let plat_supply = 202400000;
        let total_required_blocks = get_fancier_total(plat_supply, p3_priests, 10);
        println!(
            "P3: King needs to buy: {} blocks",
            total_required_blocks - plat_supply
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ex1() {
        let input = 13;
        let (missing_blocks, final_width) = get_missing_and_width(input);
        assert_eq!(missing_blocks, 3);
        assert_eq!(final_width, 7);
    }

    #[test]
    fn ex2() {
        let supply = 50;
        let priests = 3;
        let acolytes = 5;
        assert_eq!(
            (3, 9),
            get_fancy_missing_and_width(supply, priests, acolytes)
        );
    }

    #[test]
    fn ex3() {
        let block_supply = 160;
        let priests = 2;
        let acolytes = 5;
        let total = get_fancier_total(block_supply, priests, acolytes);
        assert_eq!(total, 162);
    }

    #[test]
    fn ex3b() {
        let priests = 2;
        let acolytes = 5;
        for supply in [
            19, 67, 115, 162, 239, 353, 491, 569, 690, 1885, 7601, 30655, 123131, 491005, 1964801,
            7863295, 31461371, 125820925,
        ] {
            let total = get_fancier_total(supply - 1, priests, acolytes);
            assert_eq!(total, supply);
        }
    }
}
