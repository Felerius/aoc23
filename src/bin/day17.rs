use std::{
    array,
    io::{self, BufRead},
};

use anyhow::Result;
use itertools::Itertools;

struct FixedPriorityQueue<T, const N: usize> {
    queues: [Vec<T>; N],
    offset: usize,
    base_weight: u32,
}

impl<T, const N: usize> FixedPriorityQueue<T, N> {
    fn new() -> Self {
        Self {
            queues: array::from_fn(|_| Vec::new()),
            offset: 0,
            base_weight: 0,
        }
    }

    fn pop(&mut self) -> Option<(u32, T)> {
        for _ in 0..N {
            if let Some(item) = self.queues[self.offset].pop() {
                return Some((self.base_weight, item));
            }

            self.offset += 1;
            if self.offset == N {
                self.offset = 0;
            }
            self.base_weight += 1;
        }

        None
    }

    fn push(&mut self, weight_offset: u32, item: T) {
        debug_assert!((weight_offset as usize) < N);
        let mut idx = self.offset + weight_offset as usize;
        if idx >= N {
            idx -= N;
        }

        self.queues[idx].push(item);
    }
}

fn dijkstra<const MIN: usize, const MAX: usize, const QUEUE_SIZE: usize>(
    weights: &[Vec<u8>],
) -> u32 {
    let height = weights.len();
    let width = weights[0].len();

    let mut dist = vec![vec![[u32::MAX; 2]; width]; height];
    let mut queue = FixedPriorityQueue::<_, QUEUE_SIZE>::new();
    for dir in 0..2 {
        dist[0][0][dir] = 0;
        queue.push(0, (0, 0, dir));
    }

    while let Some((d, (y, x, in_dir))) = queue.pop() {
        if y == height - 1 && x == width - 1 {
            return d;
        }
        if d > dist[y][x][in_dir] {
            continue;
        }

        let (start_coord, upper_bound) = if in_dir == 0 { (y, height) } else { (x, width) };
        let to_point = |coord| if in_dir == 0 { (coord, x) } else { (y, coord) };

        let min_coord = start_coord.checked_sub(MAX).unwrap_or(0);
        let max_coord = (start_coord + MAX).min(upper_bound - 1);
        let decreasing = (min_coord..start_coord)
            .rev()
            .scan(0, |wsum, coord| {
                let (y, x) = to_point(coord);
                *wsum += u32::from(weights[y][x]);
                Some((y, x, *wsum))
            })
            .skip(MIN - 1);
        let increasing = ((start_coord + 1)..=max_coord)
            .scan(0, |wsum, coord| {
                let (y, x) = to_point(coord);
                *wsum += u32::from(weights[y][x]);
                Some((y, x, *wsum))
            })
            .skip(MIN - 1);

        let out_dir = 1 - in_dir;
        for (y, x, wsum) in decreasing.chain(increasing) {
            if d + wsum < dist[y][x][out_dir] {
                dist[y][x][out_dir] = d + wsum;
                queue.push(wsum, (y, x, out_dir));
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

    let part1 = dijkstra::<1, 3, { 3 * 9 + 1 }>(&weights);
    let part2 = dijkstra::<4, 10, { 10 * 9 + 1 }>(&weights);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
