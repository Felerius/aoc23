use std::{
    array,
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

    let mut dist = vec![vec![[u32::MAX; 4]; width]; height];
    let mut queue = FixedPriorityQueue::<_, QUEUE_SIZE>::new();
    for dir in [2, 3] {
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

        for (out_dir, dy, dx) in TURNS[in_dir] {
            (1..=MAX)
                .scan((y, x, 0), |(y2, x2, w), _| {
                    *y2 = y2.checked_add_signed(dy).filter(|&y2| y2 < height)?;
                    *x2 = x2.checked_add_signed(dx).filter(|&x2| x2 < width)?;
                    *w += u32::from(weights[*y2][*x2]);
                    Some((*y2, *x2, *w))
                })
                .skip(MIN - 1)
                .for_each(|(y2, x2, w)| {
                    if d + w < dist[y2][x2][out_dir] {
                        dist[y2][x2][out_dir] = d + w;
                        queue.push(w, (y2, x2, out_dir));
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

    let part1 = dijkstra::<1, 3, { 3 * 9 + 1 }>(&weights);
    let part2 = dijkstra::<4, 10, { 10 * 9 + 1 }>(&weights);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
