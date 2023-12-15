use std::{
    array,
    io::{self, BufRead},
};

use anyhow::{Context, Result};

fn hash(s: &str) -> u8 {
    s.bytes()
        .fold(0_u8, |hash, c| hash.wrapping_add(c).wrapping_mul(17))
}

fn main() -> Result<()> {
    let input = io::stdin().lock().lines().next().context("empty input")??;
    let part1: u32 = input.split(',').map(|s| u32::from(hash(s))).sum();

    let mut map: [Vec<(&str, u32)>; 256] = array::from_fn(|_| Vec::new());
    for instr in input.split(',') {
        if let Some(label) = instr.strip_suffix('-') {
            let h = usize::from(hash(label));
            if let Some(idx) = map[h].iter().position(|(l, _)| l == &label) {
                map[h].remove(idx);
            }
        } else {
            let (label, focal_length) = instr.split_once('=').context("invalid instruction")?;
            let focal_length: u32 = focal_length.parse().context("invalid focal length")?;
            let h = usize::from(hash(label));
            if let Some((_, prev_focal_length)) = map[h].iter_mut().find(|(l, _)| l == &label) {
                *prev_focal_length = focal_length;
            } else {
                map[h].push((label, focal_length));
            }
        }
    }
    let part2: u32 = map
        .iter()
        .enumerate()
        .map(|(i, entries)| {
            (i as u32 + 1)
                * entries
                    .iter()
                    .enumerate()
                    .map(|(j, (_, focal_length))| (j as u32 + 1) * focal_length)
                    .sum::<u32>()
        })
        .sum();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
