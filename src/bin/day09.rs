use std::io::{self, BufRead};

use anyhow::Result;

fn extrapolate(mut history: Vec<i64>) -> i64 {
    for i in 0..history.len() {
        if history[i..].iter().all(|&x| x == history[i]) {
            history.push(history[i]);
            for j in (0..i).rev() {
                for k in (j + 1)..history.len() {
                    history[k] += history[k - 1];
                }
            }

            return history[history.len() - 1];
        }

        for j in ((i + 1)..history.len()).rev() {
            history[j] -= history[j - 1];
        }
    }

    unreachable!()
}

fn main() -> Result<()> {
    let histories = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line?
                .split_ascii_whitespace()
                .map(|s| Ok(s.parse::<i64>()?))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;

    let part1: i64 = histories.iter().cloned().map(extrapolate).sum();
    let part2: i64 = histories
        .into_iter()
        .map(|mut history| {
            history.reverse();
            history
        })
        .map(extrapolate)
        .sum();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
