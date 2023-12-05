use std::{
    io::{self, BufRead},
    ops::Range,
    str::FromStr,
};

use anyhow::{Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let mut lines = io::stdin().lock().lines().fuse();
    let seeds: Vec<_> = lines
        .next()
        .context("invalid input")??
        .split_once(": ")
        .context("invalid input")?
        .1
        .split_ascii_whitespace()
        .map(u64::from_str)
        .collect::<Result<_, _>>()?;
    lines.next();

    let mut mappings = Vec::new();
    while lines.next().is_some() {
        let mut ranges = Vec::new();
        while let Some(line) = lines.next() {
            let line = line?;
            if line.is_empty() {
                break;
            }

            let (dest_start, src_start, width) = line
                .split_ascii_whitespace()
                .map(u64::from_str)
                .collect_tuple()
                .context("invalid input")?;
            ranges.push((src_start?, dest_start?, width?));
        }

        ranges.sort_unstable_by_key(|(src_start, _, _)| *src_start);
        let mut low = 0;
        let mut ranges: Vec<_> = ranges
            .into_iter()
            .flat_map(|(src_start, dest_start, width)| {
                let range1 = (low < src_start).then_some((low, low));
                let range2 = (src_start, dest_start);
                low = src_start + width;
                [range1, Some(range2)]
            })
            .flatten()
            .collect();
        ranges.push((low, low));
        ranges.push((u64::MAX, u64::MAX));

        mappings.push(ranges);
    }

    let part1 = seeds
        .iter()
        .copied()
        .map(|mut seed| {
            for mapping in &mappings {
                let index = mapping.partition_point(|(src_start, _)| *src_start <= seed) - 1;
                let (src_start, dest_start) = mapping[index];
                seed = seed - src_start + dest_start;
            }

            seed
        })
        .min()
        .context("empty input")?;

    let mut ranges: Vec<_> = seeds
        .iter()
        .copied()
        .tuples()
        .map(|(start, width)| start..(start + width))
        .collect();
    ranges.sort_unstable_by_key(|range| range.start);
    for mapping in &mappings {
        ranges = ranges
            .into_iter()
            .flat_map(|range| {
                let Range { start, end } = range;
                mapping.iter().copied().tuple_windows().flat_map(
                    move |((src_start, dest_start), (src_end, _))| {
                        let low = start.max(src_start);
                        let high = end.min(src_end);
                        (low < high).then(|| {
                            (low - src_start + dest_start)..(high - src_start + dest_start)
                        })
                    },
                )
            })
            .collect();
    }
    dbg!(&ranges);
    let part2 = ranges
        .into_iter()
        .map(|range| range.start)
        .min()
        .context("empty input")?;

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
