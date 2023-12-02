use std::{
    array,
    io::{self, BufRead},
};

use anyhow::{bail, ensure, Context, Ok, Result};

fn parse_line(line: &str) -> Result<impl Iterator<Item = Result<[u32; 3]>> + '_> {
    let sets = line.split_once(": ").context("invalid input line")?.1;
    let sets = sets.split("; ").map(|set| {
        set.split(", ")
            .map(|entry| {
                let (count, color) = entry.split_once(' ').context("invalid set entry")?;
                let count = count.parse()?;
                let color = match color {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => bail!("invalid color {color:?}"),
                };
                Ok((count, color))
            })
            .try_fold([0; 3], |mut counts, entry| {
                let (count, color) = entry?;
                ensure!(counts[color] == 0, "duplicate color {color:?}");
                counts[color] = count;
                Ok(counts)
            })
    });
    Ok(sets)
}

fn main() -> Result<()> {
    let stdin = io::stdin().lock();
    let mut part1 = 0;
    let mut part2 = 0;
    for (i, line) in stdin.lines().enumerate() {
        let max = parse_line(&line?)?.try_fold([0; 3], |max, set| {
            let set = set?;
            Ok(array::from_fn(|i| max[i].max(set[i])))
        })?;
        if max[0] <= 12 && max[1] <= 13 && max[2] <= 14 {
            part1 += i + 1;
        }
        part2 += max[0] * max[1] * max[2];
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
