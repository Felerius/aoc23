use std::io::{self, BufRead};

use anyhow::Result;
use itertools::Itertools;

const COORD_EXPANSION_FACTORS: [usize; 2] = [2, 1_000_000];

fn solve(galaxies: &mut [[usize; 2]]) -> [usize; 2] {
    (0..2)
        .map(|dim| {
            galaxies.sort_unstable_by_key(|coord| coord[dim]);

            let mut prev = 0;
            let mut num_expanded = 0;
            let mut coord_sum_before = [0; 2];
            let mut ans = [0; 2];
            for (num_before, &coord) in galaxies.iter().enumerate() {
                if let Some(expanded) = (coord[dim] - prev).checked_sub(1) {
                    num_expanded += expanded;
                }
                prev = coord[dim];

                for ((expansion, ans), sum_before) in COORD_EXPANSION_FACTORS
                    .into_iter()
                    .zip(&mut ans)
                    .zip(&mut coord_sum_before)
                {
                    let actual_coord = coord[dim] + num_expanded * (expansion - 1);
                    *ans += actual_coord * num_before - *sum_before;
                    *sum_before += actual_coord;
                }
            }

            ans
        })
        .fold([0; 2], |[a1, b1], [a2, b2]| [a1 + a2, b1 + b2])
}

fn main() -> Result<()> {
    let mut galaxies: Vec<_> = io::stdin()
        .lock()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            Ok(line?
                .into_bytes()
                .into_iter()
                .enumerate()
                .filter(|(_, c)| *c == b'#')
                .map(move |(x, _)| [x, y]))
        })
        .flatten_ok()
        .collect::<Result<_>>()?;
    let [part1, part2] = solve(&mut galaxies);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
