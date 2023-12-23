use std::io::{self, BufRead};

use anyhow::{Context, Result};
use itertools::{iproduct, Itertools};

fn neighbors(
    y: usize,
    x: usize,
    grid: &[Vec<u8>],
) -> impl Iterator<Item = (usize, usize, u8)> + '_ {
    [
        (y + 1, x),
        (y, x + 1),
        (y.wrapping_sub(1), x),
        (y, x.wrapping_sub(1)),
    ]
    .into_iter()
    .filter_map(|(y, x)| Some((y, x, *grid.get(y)?.get(x)?)))
}

fn dag_dfs(v: usize, adj: &[Vec<usize>], weight: &[usize], longest_path: &mut [usize]) -> usize {
    if longest_path[v] == usize::MAX {
        longest_path[v] = adj[v]
            .iter()
            .map(|&v2| dag_dfs(v2, adj, weight, longest_path) + 1 + weight[v])
            .max()
            .unwrap_or(0);
    }

    longest_path[v]
}

fn longest_path_brute_force(
    v: usize,
    len: usize,
    adj: &[Vec<usize>],
    weight: &[usize],
    target: usize,
    seen: &mut [bool],
) -> usize {
    if v == target {
        return len + weight[v];
    }

    seen[v] = true;
    let ans = adj[v]
        .iter()
        .copied()
        .filter_map(|v2| {
            (!seen[v2]).then(|| {
                longest_path_brute_force(v2, len + weight[v] + 1, adj, weight, target, seen)
            })
        })
        .max()
        .unwrap_or(0);
    seen[v] = false;

    ans
}

fn main() -> Result<()> {
    let grid: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map_ok(|line| line.into_bytes())
        .try_collect()?;
    let height = grid.len();
    let width = grid[0].len();

    let mut adj: Vec<Vec<usize>> = Vec::new();
    let mut weight = Vec::new();
    let mut endpoints = vec![vec![None; width]; height];
    for (y, x) in iproduct!(0..height, 0..width) {
        if grid[y][x] != b'.' || endpoints[y][x].is_some() {
            continue;
        }

        let only_neighbor = neighbors(y, x, &grid)
            .filter_map(|(y, x, c)| (c == b'.').then_some((y, x)))
            .at_most_one();
        match only_neighbor {
            Ok(Some(mut cur)) => {
                let mut len = 2;
                let mut prev = (y, x);
                while let Ok((next_y, next_x, _)) = neighbors(cur.0, cur.1, &grid)
                    .filter(|&(y, x, c)| (y, x) != prev && c == b'.')
                    .exactly_one()
                {
                    len += 1;
                    prev = cur;
                    cur = (next_y, next_x);
                }

                let v = adj.len();
                endpoints[y][x] = Some(v);
                endpoints[cur.0][cur.1] = Some(v);
                adj.push(Vec::new());
                weight.push(len);
            }
            Ok(None) => {
                endpoints[y][x] = Some(adj.len());
                adj.push(Vec::new());
                weight.push(1);
            }
            _ => {}
        }
    }

    for (y, x) in iproduct!(0..height, 0..width) {
        match grid[y][x] {
            b'<' | b'>' => {
                let v_left = endpoints[y][x - 1].context("slope does not connect endpoints")?;
                let v_right = endpoints[y][x + 1].context("slope does not connect endpoints")?;
                if grid[y][x] == b'>' {
                    adj[v_left].push(v_right);
                } else {
                    adj[v_right].push(v_left);
                }
            }
            b'^' | b'v' => {
                let v_top = endpoints[y - 1][x].context("slope does not connect endpoints")?;
                let v_bottom = endpoints[y + 1][x].context("slope does not connect endpoints")?;
                if grid[y][x] == b'v' {
                    adj[v_top].push(v_bottom);
                } else {
                    adj[v_bottom].push(v_top);
                }
            }
            _ => {}
        }
    }

    let start = (0..width)
        .find_map(|x| endpoints[0][x])
        .context("no start")?;
    let target = (0..width)
        .find_map(|x| endpoints[height - 1][x])
        .context("no target")?;
    let mut longest_path = vec![usize::MAX; adj.len()];
    longest_path[target] = weight[target];
    let part1 = dag_dfs(start, &adj, &weight, &mut longest_path) - 1;

    let mut adj2 = adj.clone();
    for (v, row) in adj.iter().enumerate() {
        for &v2 in row {
            adj2[v2].push(v);
        }
    }

    let mut seen = vec![false; adj.len()];
    let part2 = longest_path_brute_force(start, 0, &adj2, &weight, target, &mut seen) - 1;

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
