use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use anyhow::Result;
use itertools::Itertools;

fn tilt(
    len_tilt: usize,
    len_non_tilt: usize,
    mut set_or_get: impl FnMut(usize, usize, Option<u8>) -> u8,
) {
    for i in 0..len_non_tilt {
        let mut next = 0;
        for j in 0..len_tilt {
            let c = set_or_get(j, i, None);
            if c == b'#' {
                next = j + 1;
            } else if c == b'O' {
                set_or_get(j, i, Some(b'.'));
                set_or_get(next, i, Some(b'O'));
                next += 1;
            }
        }
    }
}

fn tilt_cycle(grid: &mut [Vec<u8>]) {
    let height = grid.len();
    let width = grid[0].len();
    tilt(height, width, |y, x, c| {
        grid[y][x] = c.unwrap_or(grid[y][x]);
        grid[y][x]
    });
    tilt(width, height, |x, y, c| {
        grid[y][x] = c.unwrap_or(grid[y][x]);
        grid[y][x]
    });
    tilt(height, width, |y, x, c| {
        let y = height - y - 1;
        grid[y][x] = c.unwrap_or(grid[y][x]);
        grid[y][x]
    });
    tilt(width, height, |x, y, c| {
        let x = width - x - 1;
        grid[y][x] = c.unwrap_or(grid[y][x]);
        grid[y][x]
    });
}

fn eval_grid(grid: &[Vec<u8>]) -> usize {
    grid.iter()
        .enumerate()
        .map(|(y, row)| row.iter().filter(|&&c| c == b'O').count() * (grid.len() - y))
        .sum()
}

fn part1(mut grid: Vec<Vec<u8>>) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    tilt(height, width, |y, x, c| {
        if let Some(c) = c {
            grid[y][x] = c;
        }
        grid[y][x]
    });
    eval_grid(&grid)
}

fn part2(mut grid: Vec<Vec<u8>>) -> usize {
    let mut path = vec![grid.clone()];
    let mut seen = HashMap::new();
    seen.insert(grid.clone(), 0);
    loop {
        tilt_cycle(&mut grid);
        if let Some(cycle_start) = seen.insert(grid.clone(), path.len()) {
            let cycle_len = path.len() - cycle_start;
            let idx_in_cycle = (1_000_000_000 - cycle_start) % cycle_len;
            break eval_grid(&path[cycle_start + idx_in_cycle]);
        }
        path.push(grid.clone());
    }
}

fn main() -> Result<()> {
    let grid: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map_ok(|line| line.into_bytes())
        .collect::<Result<_, _>>()?;

    let part1 = part1(grid.clone());
    let part2 = part2(grid.clone());

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
