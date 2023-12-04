use std::io::{self, BufRead};

use anyhow::Result;
use itertools::Itertools;

fn is_symbol(c: u8) -> bool {
    !c.is_ascii_digit() && c != b'.'
}

fn main() -> Result<()> {
    let lines: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map_ok(|line| line.into_bytes())
        .collect::<Result<_, _>>()?;
    let height = lines.len();
    let width = lines[0].len();

    let mut part1 = 0;
    let mut gear_adj = vec![vec![vec![]; width]; height];
    for y in 0..height {
        let mut x = 0;
        while x < width {
            if !lines[y][x].is_ascii_digit() {
                x += 1;
                continue;
            }

            let mut xr = x;
            let mut num = 0;
            while xr < width && lines[y][xr].is_ascii_digit() {
                num = num * 10 + u32::from(lines[y][xr] - b'0');
                xr += 1;
            }

            let y_adj = y.saturating_sub(1)..=(y + 1).min(height - 1);
            let x_adj = x.saturating_sub(1)..=xr.min(width - 1);
            let mut any_symbol = false;
            for y2 in y_adj {
                for x2 in x_adj.clone() {
                    any_symbol |= is_symbol(lines[y2][x2]);
                    if lines[y2][x2] == b'*' {
                        gear_adj[y2][x2].push(num);
                    }
                }
            }
            if any_symbol {
                part1 += num;
            }

            x = xr;
        }
    }

    let part2 = gear_adj
        .into_iter()
        .flatten()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum::<u32>();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
