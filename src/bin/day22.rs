use std::{
    collections::VecDeque,
    io::{self, BufRead},
    str::FromStr,
};

use anyhow::{ensure, Context, Result};
use itertools::{iproduct, Itertools};

const X_Y_BOUND: usize = 10;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (x, y, z) = s
            .split(',')
            .map(usize::from_str)
            .collect_tuple()
            .context("invalid number of coordinates")?;
        let x = x?;
        let y = y?;
        ensure!((0..X_Y_BOUND).contains(&x), "x out of range");
        ensure!((0..X_Y_BOUND).contains(&y), "y out of range");
        Ok(Self { x, y, z: z? })
    }
}

fn main() -> Result<()> {
    let mut bricks: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line?;
            let (from, to) = line.split_once('~').context("invalid line")?;
            let from = Point::from_str(from)?;
            let to = Point::from_str(to)?;
            ensure!(
                from.x <= to.x && from.y <= to.y && from.z <= to.z,
                "coordinates out of order"
            );
            Ok((from, to))
        })
        .try_collect()?;
    bricks.sort_unstable_by_key(|(from, _)| from.z);

    let mut height_map = [[(0, usize::MAX); X_Y_BOUND]; X_Y_BOUND];
    let mut adj: Vec<Vec<_>> = Vec::new();
    let mut in_deg = Vec::new();
    for &(from, to) in &bricks {
        let blocks = iproduct!(from.x..=to.x, from.y..=to.y, from.z..=to.z)
            .map(|(x, y, z)| Point { x, y, z });

        let base_height = blocks
            .clone()
            .map(|p| height_map[p.x][p.y].0 + 1)
            .max()
            .unwrap_or(0);

        let v = adj.len();
        let mut in_deg_v = 0;
        for p in blocks {
            let final_z = p.z - from.z + base_height;
            // blocks are ordered by ascending z
            debug_assert!(height_map[p.x][p.y].0 < final_z);

            if height_map[p.x][p.y].0 == final_z - 1 {
                let below = height_map[p.x][p.y].1;
                if below != v && below != usize::MAX && adj[below].last() != Some(&v) {
                    adj[below].push(v);
                    in_deg_v += 1;
                }
            }

            height_map[p.x][p.y] = (final_z, v);
        }

        adj.push(Vec::new());
        in_deg.push(in_deg_v);
    }

    let mut rem_in_deg = vec![(0, 0); adj.len()];
    let mut queue = VecDeque::new();
    let (part1, part2) = (0..adj.len())
        .map(|v0| {
            queue.push_back(v0);
            let mut count = 0;
            while let Some(v) = queue.pop_front() {
                count += 1;
                for &v2 in &adj[v] {
                    if rem_in_deg[v2].0 <= v0 {
                        rem_in_deg[v2] = (v0 + 1, in_deg[v2]);
                    }

                    rem_in_deg[v2].1 -= 1;
                    if rem_in_deg[v2].1 == 0 {
                        queue.push_back(v2);
                    }
                }
            }

            count - 1
        })
        .fold((0, 0), |(part1, part2), cnt| {
            (part1 + usize::from(cnt == 0), part2 + cnt)
        });

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
