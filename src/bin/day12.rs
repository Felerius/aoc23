use std::{
    io::{self, BufRead},
    mem,
};

use anyhow::{Context, Ok, Result};

fn count_valid_arrangements(records: &[u8], blocks: &[usize]) -> u64 {
    // We modify the input in `main` to avoid handling the case where the first
    // block starts at the very beginning
    debug_assert_eq!(records[0], b'.');

    let mut dp = vec![0; records.len() + 1];
    let mut dp_prev = dp.clone();
    let max_empty_prefix = records.iter().take_while(|&&record| record != b'#').count();
    dp_prev[..=max_empty_prefix].fill(1);

    for block in blocks {
        dp.fill(0);
        let mut max_block = 0;
        for (j, &record) in records.iter().enumerate() {
            let can_place_empty = record != b'#';
            let can_place_spring = record != b'.';
            max_block = if can_place_spring { max_block + 1 } else { 0 };

            if can_place_empty {
                dp[j + 1] += dp[j];
            }

            if max_block >= *block && j + 1 > *block && records[j - *block] != b'#' {
                dp[j + 1] += dp_prev[j - *block];
            }
        }

        mem::swap(&mut dp, &mut dp_prev);
    }

    dp_prev[records.len()]
}

fn main() -> Result<()> {
    let lines: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let mut line = line?;
            let (front, back) = line.split_once(' ').context("invalid input")?;
            let blocks = back
                .split(',')
                .map(|num| num.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()?;
            line.truncate(front.len());
            Ok((line.into_bytes(), blocks))
        })
        .collect::<Result<_, _>>()?;

    let (part1, part2) = lines
        .into_iter()
        .map(|(mut record, mut blocks)| {
            // To avoid handling the first block starting at the very beginning
            record.insert(0, b'.');

            let part1 = count_valid_arrangements(&record, &blocks);

            let n = record.len();
            let m = blocks.len();
            for _ in 0..4 {
                record.push(b'?');
                record.extend_from_within(1..n);
                blocks.extend_from_within(0..m);
            }
            let part2 = count_valid_arrangements(&record, &blocks);

            (part1, part2)
        })
        .fold((0, 0), |(a1, a2), (b1, b2)| (a1 + b1, a2 + b2));

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
