use std::io::{self, BufRead};

use anyhow::{Context, Ok, Result};

fn count_valid_arrangements(record: &[u8], blocks: &[usize]) -> usize {
    let num_records = record.len();
    let num_blocks = blocks.len();
    let mut max_block = 0;
    let mut dp = vec![vec![0; num_blocks + 1]; num_records + 1];
    dp[0][0] = 1;

    for i in 0..num_records {
        let can_place_empty = record[i] != b'#';
        let can_place_spring = record[i] != b'.';
        max_block = if can_place_spring { max_block + 1 } else { 0 };
        dp[i + 1].fill(0);

        dp[i + 1][0] = if can_place_empty { dp[i][0] } else { 0 };
        for j in 0..num_blocks {
            if can_place_empty {
                dp[i + 1][j + 1] += dp[i][j + 1];
            }
            if max_block >= blocks[j] && (i + 1 == blocks[j] || record[i - blocks[j]] != b'#') {
                dp[i + 1][j + 1] += if i + 1 == blocks[j] {
                    dp[0][j]
                } else {
                    dp[i - blocks[j]][j]
                };
            }
        }
    }

    dp[num_records][num_blocks]
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
