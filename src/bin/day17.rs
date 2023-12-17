use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{self, BufRead},
};

use anyhow::Result;
use itertools::Itertools;

// Directions: east, south, west, north
const TURNS: [[(usize, isize, isize); 2]; 4] = [
    [(1, 1, 0), (3, -1, 0)],
    [(0, 0, 1), (2, 0, -1)],
    [(1, 1, 0), (3, -1, 0)],
    [(0, 0, 1), (2, 0, -1)],
];

fn dijkstra<const MIN: usize, const MAX: usize>(weights: &[Vec<u8>]) -> u32 {
    let height = weights.len();
    let width = weights[0].len();

    let mut dist = vec![vec![[u32::MAX; 4]; width]; height];
    let mut queue = BinaryHeap::new();
    for dir in [2, 3] {
        dist[0][0][dir] = 0;
        queue.push((Reverse(0), 0, 0, dir));
    }

    while let Some((d, y, x, in_dir)) = queue.pop() {
        let Reverse(d) = d;
        if y == height - 1 && x == width - 1 {
            return d;
        }
        if d > dist[y][x][in_dir] {
            continue;
        }

        for (out_dir, dy, dx) in TURNS[in_dir] {
            (1..=MAX)
                .scan((y, x, d), |(y2, x2, d2), _| {
                    *y2 = y2.checked_add_signed(dy).filter(|&y2| y2 < height)?;
                    *x2 = x2.checked_add_signed(dx).filter(|&x2| x2 < width)?;
                    *d2 += u32::from(weights[*y2][*x2]);
                    Some((*y2, *x2, *d2))
                })
                .skip(MIN - 1)
                .for_each(|(y2, x2, d2)| {
                    if d2 < dist[y2][x2][out_dir] {
                        dist[y2][x2][out_dir] = d2;
                        queue.push((Reverse(d2), y2, x2, out_dir));
                    }
                });
        }
    }

    unreachable!("target should always be reachable");
}

fn main() -> Result<()> {
    let weights: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map_ok(|line| {
            let mut row = line.into_bytes();
            for weight in &mut row {
                *weight -= b'0';
            }
            row
        })
        .try_collect()?;

    let part1 = dijkstra::<1, 3>(&weights);
    let part2 = dijkstra::<4, 10>(&weights);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
