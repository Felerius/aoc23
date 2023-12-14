use std::io::{self, BufRead};

use anyhow::Result;
use itertools::Itertools;

fn count_reflections(grid: &[Vec<u8>], diff: usize) -> usize {
    (0..(grid.len() - 1))
        .filter(|&i| {
            let left = (0..=i).rev();
            let right = (i + 1)..grid.len();
            let differences = left
                .zip(right)
                .flat_map(|(l, r)| grid[l].iter().zip(&grid[r]))
                .filter(|(a, b)| a != b)
                .count();
            differences == diff
        })
        .map(|i| i + 1)
        .sum()
}

fn main() -> Result<()> {
    let mut patterns = Vec::new();
    let mut lines = io::stdin().lock().lines().peekable();
    while lines.peek().is_some() {
        let pattern = lines
            .by_ref()
            .take_while(|line| line.as_ref().is_ok_and(|line| line != ""))
            .map_ok(|line| line.into_bytes())
            .collect::<Result<Vec<_>, _>>()?;
        patterns.push(pattern);
    }

    let (part1, part2) = patterns
        .iter()
        .map(|pattern| {
            let mut pattern_transposed = vec![vec![b' '; pattern.len()]; pattern[0].len()];
            for (i, row) in pattern.iter().enumerate() {
                for (j, &c) in row.iter().enumerate() {
                    pattern_transposed[j][i] = c;
                }
            }

            let part1 =
                count_reflections(&pattern_transposed, 0) + 100 * count_reflections(&pattern, 0);
            let part2 =
                count_reflections(&pattern_transposed, 1) + 100 * count_reflections(&pattern, 1);
            (part1, part2)
        })
        .fold((0, 0), |(a1, a2), (b1, b2)| (a1 + b1, a2 + b2));

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
