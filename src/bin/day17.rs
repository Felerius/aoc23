use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{self, BufRead},
};

use anyhow::Result;
use itertools::Itertools;

fn part1(weights: &[Vec<u8>]) -> u32 {
    let height = weights.len();
    let width = weights[0].len();

    // Directions: east, south, west, north
    let mut dist = vec![vec![[[u32::MAX; 3]; 4]; width]; height];
    let mut queue = BinaryHeap::new();
    for dir in [0, 1] {
        dist[0][0][dir][0] = 0;
        queue.push((Reverse(0), 0, 0, dir, 0));
    }

    while let Some((d, y, x, in_dir, in_streak)) = queue.pop() {
        let Reverse(d) = d;
        if y == height - 1 && x == width - 1 {
            return d;
        }
        if d > dist[y][x][in_dir][in_streak] {
            continue;
        }

        for (out_dir, (dy, dx)) in [(0, 1), (1, 0), (0, -1), (-1, 0)].into_iter().enumerate() {
            let Some(y2) = y.checked_add_signed(dy) else {
                continue;
            };
            let Some(x2) = x.checked_add_signed(dx) else {
                continue;
            };
            if y2 == height || x2 == width {
                continue;
            }

            let out_streak = if in_dir == out_dir { in_streak + 1 } else { 0 };
            if (in_dir + 2) % 4 == out_dir || out_streak > 2 {
                continue;
            }

            let d2 = d + u32::from(weights[y2][x2]);
            if d2 < dist[y2][x2][out_dir][out_streak] {
                dist[y2][x2][out_dir][out_streak] = d2;
                queue.push((Reverse(d2), y2, x2, out_dir, out_streak));
            }
        }
    }

    unreachable!("target should always be reachable");
}

fn part2(weights: &[Vec<u8>]) -> u32 {
    let height = weights.len();
    let width = weights[0].len();

    // Directions: east, south, west, north
    let mut dist = vec![vec![[[u32::MAX; 10]; 4]; width]; height];
    let mut queue = BinaryHeap::new();
    for dir in [2, 3] {
        dist[0][0][dir][3] = 0;
        queue.push((Reverse(0), 0, 0, dir, 3));
    }

    while let Some((d, y, x, in_dir, in_streak)) = queue.pop() {
        let Reverse(d) = d;
        if y == height - 1 && x == width - 1 && in_streak >= 3 {
            return d;
        }
        if d > dist[y][x][in_dir][in_streak] {
            continue;
        }

        for (out_dir, (dy, dx)) in [(0, 1), (1, 0), (0, -1), (-1, 0)].into_iter().enumerate() {
            let Some(y2) = y.checked_add_signed(dy) else {
                continue;
            };
            let Some(x2) = x.checked_add_signed(dx) else {
                continue;
            };
            if y2 == height || x2 == width {
                continue;
            }

            let out_streak = if in_dir == out_dir { in_streak + 1 } else { 0 };
            if (in_dir + 2) % 4 == out_dir || out_streak > 9 || (in_dir != out_dir && in_streak < 3)
            {
                continue;
            }

            let d2 = d + u32::from(weights[y2][x2]);
            if d2 < dist[y2][x2][out_dir][out_streak] {
                dist[y2][x2][out_dir][out_streak] = d2;
                queue.push((Reverse(d2), y2, x2, out_dir, out_streak));
            }
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

    let part1 = part1(&weights);
    let part2 = part2(&weights);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
