use std::io::{self, BufRead};

use anyhow::{Context, Result};
use rustc_hash::{FxHashMap, FxHashSet};

fn main() -> Result<()> {
    let stdin = io::stdin().lock();
    let mut counts = FxHashMap::<usize, usize>::default();
    let mut part1 = 0;
    let mut part2 = 0;
    for (i, line) in stdin.lines().enumerate() {
        let line = line?;
        let count = counts.get(&i).copied().unwrap_or(1);
        let (winning, chosen) = line
            .split_once(": ")
            .context("invalid input")?
            .1
            .split_once(" | ")
            .context("invalid input")?;
        let winning: FxHashSet<_> = winning.split_ascii_whitespace().collect();
        let num_correct = chosen
            .split_ascii_whitespace()
            .filter(|num| winning.contains(num))
            .count();

        if let Some(exponent) = num_correct.checked_sub(1) {
            part1 += 1 << exponent;
        }

        part2 += count;
        for off in 1..=num_correct {
            *counts.entry(i + off).or_insert(1) += count;
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
