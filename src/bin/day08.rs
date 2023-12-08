use std::{
    collections::HashMap,
    io::{self, BufRead},
    iter,
};

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;

fn parse_node(s: &str) -> usize {
    let s = s.as_bytes();
    (usize::from(s[0] - b'A') * 26 + usize::from(s[1] - b'A')) * 26 + usize::from(s[2] - b'A')
}

const START_NODE: usize = 0;
const END_NODE: usize = 26 * 26 * 26 - 1;

fn ext_gcd(mut a: i64, mut b: i64) -> (i64, i64, i64) {
    let mut x = 1;
    let mut y = 0;
    let mut x1 = 0;
    let mut y1 = 1;
    while b != 0 {
        let t = a / b;
        (x, x1) = (x1, x - t * x1);
        (y, y1) = (y1, y - t * y1);
        (a, b) = (b, a - t * b);
    }
    (a, x, y)
}

fn solve_crt(mut a: i64, m: i64, mut b: i64, n: i64) -> Option<(i64, i64)> {
    if n > m {
        return solve_crt(b, n, a, m);
    }

    a %= m;
    b %= n;
    let (g, x, _) = ext_gcd(m, n);
    ((b - a) % g == 0).then(|| {
        let l = m / g * n;
        let z = (b - a) % n * x % n / g * m + a;
        let c = if z < 0 { z + l } else { z };
        (c, l)
    })
}

fn main() -> Result<()> {
    let mut lines = io::stdin().lock().lines();
    let instructions = lines.next().context("unexpected eof")??;
    lines.next();

    let mut adj = vec![None; 26 * 26 * 26];
    for line in lines {
        let line = line?;
        let (front, back) = line.split_once(" = ").context("invalid input")?;
        let node = parse_node(front);
        let (left, right) = back[1..]
            .trim_end_matches(')')
            .split_once(", ")
            .context("invalid input")?;
        adj[node] = Some((parse_node(left), parse_node(right)));
    }

    let mut node = START_NODE;
    let mut part1 = 0;
    for &direction in iter::repeat(instructions.as_bytes()).flatten() {
        part1 += 1;
        let (left, right) = adj[node].context("trail has left the graph!?")?;
        node = if direction == b'L' { left } else { right };
        if node == END_NODE {
            break;
        }
    }

    let cycles = (0..(26 * 26))
        .map(|v| 26 * v)
        .filter(|&v| adj[v].is_some())
        .map(|v| {
            let mut path = vec![v];
            let mut seen = HashMap::new();
            seen.insert((v, instructions.len() - 1), 0);
            for (instr_idx, direction) in iter::repeat(instructions.bytes().enumerate()).flatten() {
                let node = *path.last().unwrap();
                let (left, right) = adj[node].expect("trail has left the graph!?");
                let next_node = if direction == b'L' { left } else { right };

                if let Some(offset) = seen.insert((next_node, instr_idx), path.len()) {
                    let end_pos = path[offset..]
                        .iter()
                        .copied()
                        .enumerate()
                        .filter(|&(_, v)| v % 26 == 25)
                        .map(|(i, _)| i)
                        .exactly_one()
                        .map_err(|err| {
                            let msg = if err.count() == 0 {
                                "no end positions in cycle"
                            } else {
                                "multiple end positions in cycle"
                            };
                            anyhow!(msg)
                        })?;
                    let cyc_len = path.len() - offset;
                    return Ok((cyc_len, end_pos, offset));
                }

                path.push(next_node);
            }

            unreachable!();
        })
        .collect::<Result<Vec<_>>>()?;

    let (mut part2, lcm, mn) =
        cycles
            .into_iter()
            .fold((0, 1, 0), |(x, l, mn), (cyc_len, end_pos, offset)| {
                let cyc_len = cyc_len as i64;
                let end_pos2 = (end_pos + offset) as i64 % cyc_len;
                let (x_new, l_new) = solve_crt(x, l, end_pos2, cyc_len).expect("no solution");
                (x_new, l_new, mn.max(offset as i64))
            });
    while part2 < mn {
        part2 += lcm;
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
