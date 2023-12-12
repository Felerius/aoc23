use std::{
    io::{self, BufRead},
    mem,
};

use anyhow::{Context, Ok, Result};

fn count_valid_arrangements(record: &[u8], blocks: &[usize]) -> usize {
    let num_records = record.len();
    let num_blocks = blocks.len();
    let max_blocks = blocks.iter().copied().max().unwrap_or(0);
    let mut dp = vec![vec![0; max_blocks + 1]; num_blocks + 1];
    let mut dp_prev = dp.clone();
    dp_prev[0][0] = 1;

    for i in 0..num_records {
        let can_place_empty = record[i] != b'#';
        let can_place_spring = record[i] != b'.';
        for row in &mut dp {
            row.fill(0);
        }

        for j in 0..=num_blocks {
            for k in 0..=blocks.get(j).copied().unwrap_or(0) {
                if can_place_empty {
                    if j != num_blocks && k == blocks[j] {
                        dp[j + 1][0] += dp_prev[j][k];
                    } else if k == 0 {
                        dp[j][0] += dp_prev[j][k];
                    }
                }

                if can_place_spring {
                    if j != num_blocks && k < blocks[j] {
                        dp[j][k + 1] += dp_prev[j][k];
                    }
                }
            }
        }

        mem::swap(&mut dp, &mut dp_prev);
    }

    dp_prev[num_blocks][0] + dp_prev[num_blocks - 1][blocks[num_blocks - 1]]
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
            let part1 = count_valid_arrangements(&record, &blocks);

            let n = record.len();
            let m = blocks.len();
            for _ in 0..4 {
                record.push(b'?');
                record.extend_from_within(0..n);
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
