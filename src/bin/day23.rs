use std::{io, iter, str::FromStr};

use anyhow::{Context, Result};
use itertools::{iproduct, Itertools};

#[derive(Debug, Clone)]
struct Graph {
    num_nodes: usize,
    adj: [u128; Self::MAX_SIZE],
    node_weights: [u16; Self::MAX_SIZE],
    start: usize,
    target: usize,
}

impl Graph {
    const MAX_SIZE: usize = 128;

    fn neighbors(&self, v: usize, mask: u128) -> impl Iterator<Item = usize> {
        let mut bs = self.adj[v] & mask;
        iter::from_fn(move || {
            bs.checked_sub(1).map(|bs_minus_one| {
                let v2 = bs.trailing_zeros() as usize;
                bs &= bs_minus_one;
                v2
            })
        })
    }
}

impl FromStr for Graph {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let grid: Vec<_> = s.lines().map(|line| line.as_bytes()).collect();
        let height = grid.len();
        let width = grid[0].len();

        let mut num_nodes = 0;
        let mut adj = [0; Self::MAX_SIZE];
        let mut node_weights = [0; Self::MAX_SIZE];
        let mut endpoints = vec![vec![None; width]; height];
        for (y, x) in iproduct!(0..height, 0..width) {
            if grid[y][x] != b'.' || endpoints[y][x].is_some() {
                continue;
            }
            let Ok(maybe_only_neighbor) = grid_neighbors(y, x, &grid)
                .filter_map(|(y, x, c)| (c == b'.').then_some((y, x)))
                .at_most_one()
            else {
                continue;
            };

            if let Some(mut cur) = maybe_only_neighbor {
                let mut len = 2;
                let mut prev = (y, x);
                while let Ok((next_y, next_x, _)) = grid_neighbors(cur.0, cur.1, &grid)
                    .filter(|&(y, x, c)| (y, x) != prev && c == b'.')
                    .exactly_one()
                {
                    len += 1;
                    prev = cur;
                    cur = (next_y, next_x);
                }

                endpoints[y][x] = Some(num_nodes);
                endpoints[cur.0][cur.1] = Some(num_nodes);
                node_weights[num_nodes] = len;
            } else {
                endpoints[y][x] = Some(num_nodes);
                node_weights[num_nodes] = 1;
            }

            num_nodes += 1;
        }

        for (y, x) in iproduct!(0..height, 0..width) {
            match grid[y][x] {
                b'<' | b'>' => {
                    let v_left = endpoints[y][x - 1].context("slope does not connect endpoints")?;
                    let v_right =
                        endpoints[y][x + 1].context("slope does not connect endpoints")?;
                    if grid[y][x] == b'>' {
                        adj[v_left] |= 1 << v_right;
                    } else {
                        adj[v_right] |= 1 << v_left;
                    }
                }
                b'^' | b'v' => {
                    let v_top = endpoints[y - 1][x].context("slope does not connect endpoints")?;
                    let v_bottom =
                        endpoints[y + 1][x].context("slope does not connect endpoints")?;
                    if grid[y][x] == b'v' {
                        adj[v_top] |= 1 << v_bottom;
                    } else {
                        adj[v_bottom] |= 1 << v_top;
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

        Ok(Self {
            num_nodes,
            adj,
            node_weights,
            start,
            target,
        })
    }
}

fn grid_neighbors<'a>(
    y: usize,
    x: usize,
    grid: &'a [&'a [u8]],
) -> impl Iterator<Item = (usize, usize, u8)> + 'a {
    [
        (y + 1, x),
        (y, x + 1),
        (y.wrapping_sub(1), x),
        (y, x.wrapping_sub(1)),
    ]
    .into_iter()
    .filter_map(|(y, x)| Some((y, x, *grid.get(y)?.get(x)?)))
}

fn dag_dfs(v: usize, graph: &Graph, longest_path: &mut [u16]) -> u16 {
    if longest_path[v] == u16::MAX {
        longest_path[v] = graph
            .neighbors(v, u128::MAX)
            .map(|v2| dag_dfs(v2, graph, longest_path) + 1 + graph.node_weights[v])
            .max()
            .unwrap_or(0);
    }

    longest_path[v]
}

fn longest_path_brute_force(v: usize, len: u16, graph: &Graph, seen: u128) -> u16 {
    if v == graph.target {
        return len + graph.node_weights[v];
    }

    let ans = graph
        .neighbors(v, !seen)
        .map(|v2| {
            longest_path_brute_force(v2, len + graph.node_weights[v] + 1, graph, seen | (1 << v))
        })
        .max()
        .unwrap_or(0);

    ans
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin().lock())?;
    let graph = Graph::from_str(&input)?;

    let mut longest_path = vec![u16::MAX; graph.num_nodes];
    longest_path[graph.target] = graph.node_weights[graph.target];
    let part1 = dag_dfs(graph.start, &graph, &mut longest_path) - 1;

    let mut graph_undirected = graph.clone();
    for v in 0..graph.num_nodes {
        for v2 in graph.neighbors(v, u128::MAX) {
            graph_undirected.adj[v2] |= 1 << v;
        }
    }
    let part2 = longest_path_brute_force(graph_undirected.start, 0, &graph_undirected, 0) - 1;

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
