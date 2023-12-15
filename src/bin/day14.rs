use std::io::{self, BufRead};

use anyhow::Result;
use itertools::Itertools;
use rustc_hash::FxHashMap;

fn tilt(
    len_tilt: usize,
    len_non_tilt: usize,
    grid: &mut [u8],
    mut index: impl FnMut(usize, usize) -> usize,
) {
    for i in 0..len_non_tilt {
        let mut next = 0;
        for j in 0..len_tilt {
            let idx = index(j, i);
            if grid[idx] == b'#' {
                next = j + 1;
            } else if grid[idx] == b'O' {
                grid[idx] = b'.';
                grid[index(next, i)] = b'O';
                next += 1;
            }
        }
    }
}

fn tilt_cycle(grid: &mut [u8], height: usize, width: usize) {
    tilt(height, width, grid, |y, x| y * width + x);
    tilt(width, height, grid, |x, y| y * width + x);
    tilt(height, width, grid, |y, x| (height - 1 - y) * width + x);
    tilt(width, height, grid, |x, y| y * width + (width - 1 - x));
}

fn eval_grid(grid: &[u8], height: usize, width: usize) -> usize {
    grid.iter()
        .enumerate()
        .filter(|&(_, &c)| c == b'O')
        .map(|(i, _)| (height - i / width))
        .sum()
}

fn part1(mut grid: Vec<u8>, height: usize, width: usize) -> usize {
    tilt(height, width, &mut grid, |y, x| y * width + x);
    eval_grid(&grid, height, width)
}

fn part2(mut grid: Vec<u8>, height: usize, width: usize) -> usize {
    let mut seen = FxHashMap::default();
    seen.insert(grid.clone(), 0);
    loop {
        tilt_cycle(&mut grid, height, width);
        if let Some(cycle_start) = seen.insert(grid.clone(), seen.len()) {
            let cycle_len = seen.len() - cycle_start;
            let idx_in_cycle = (1_000_000_000 - cycle_start) % cycle_len;
            let idx_in_path = idx_in_cycle + cycle_start;
            let (final_grid, _) = seen.iter().find(|&(_, &idx)| idx == idx_in_path).unwrap();
            break eval_grid(final_grid, height, width);
        }
    }
}

fn main() -> Result<()> {
    let mut height = 0;
    let grid: Vec<_> = io::stdin()
        .lock()
        .lines()
        .inspect(|_| height += 1)
        .map_ok(|line| line.into_bytes())
        .flatten_ok()
        .collect::<Result<_, _>>()?;
    let width = grid.len() / height;

    let part1 = part1(grid.clone(), height, width);
    let part2 = part2(grid.clone(), height, width);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
