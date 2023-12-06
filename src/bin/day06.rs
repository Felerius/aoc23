use std::io::{self, BufRead};

use anyhow::{Context, Result};
use itertools::Itertools;

fn read_line(line: &str) -> Result<Vec<u64>> {
    line.split_once(":")
        .context("invalid input")?
        .1
        .split_ascii_whitespace()
        .map(|s| Ok(s.parse()?))
        .collect()
}

fn binary_search(mut low: u64, mut high: u64, mut pred: impl FnMut(u64) -> bool) -> u64 {
    while high - low > 1 {
        let mid = low + (high - low) / 2;
        if pred(mid) {
            high = mid;
        } else {
            low = mid;
        }
    }

    low
}

fn count_ways_to_beat(time: u64, distance: u64) -> u64 {
    let mid = time / 2;
    if mid * (time - mid) <= distance {
        return 0;
    }

    let low = binary_search(0, mid, |x| x * (time - x) > distance);
    let high = binary_search(mid, time + 1, |x| x * (time - x) <= distance);
    high - low
}

fn main() -> Result<()> {
    let mut lines = io::stdin().lock().lines();
    let times = read_line(&lines.next().context("invalid input")??)?;
    let distances = read_line(&lines.next().context("invalid input")??)?;
    let part1: u64 = times
        .iter()
        .zip(&distances)
        .map(|(&time, &distance)| count_ways_to_beat(time, distance))
        .product();

    let joined_time = times.iter().join("").parse().unwrap();
    let joined_distance = distances.iter().join("").parse().unwrap();
    let part2 = count_ways_to_beat(joined_time, joined_distance);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
