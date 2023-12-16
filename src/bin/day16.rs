use std::{
    collections::VecDeque,
    io::{self, BufRead},
    vec,
};

use anyhow::{bail, Result};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Edge {
    InWest,
    InEast,
    InNorth,
    InSouth,
    OutWest,
    OutEast,
    OutNorth,
    OutSouth,
}

#[derive(Debug, Clone, Copy)]
struct GraphIndexing {
    width: usize,
    height: usize,
}

impl GraphIndexing {
    fn total(&self) -> usize {
        2 * self.height * (self.width + 1) + 2 * self.width * (self.height + 1)
    }

    fn east(&self, x: usize, y: usize) -> usize {
        y * (self.width + 1) + x
    }

    fn west(&self, x: usize, y: usize) -> usize {
        self.height * (self.width + 1) + y * (self.width + 1) + x
    }

    fn north(&self, x: usize, y: usize) -> usize {
        2 * self.height * (self.width + 1) + y * self.width + x
    }

    fn south(&self, x: usize, y: usize) -> usize {
        2 * self.height * (self.width + 1) + (self.height + 1) * self.width + y * self.width + x
    }

    fn index(&self, x: usize, y: usize, edge: Edge) -> usize {
        match edge {
            Edge::InWest => self.east(x, y),
            Edge::InEast => self.west(x + 1, y),
            Edge::InNorth => self.south(x, y),
            Edge::InSouth => self.north(x, y + 1),
            Edge::OutWest => self.west(x, y),
            Edge::OutEast => self.east(x + 1, y),
            Edge::OutNorth => self.north(x, y),
            Edge::OutSouth => self.south(x, y + 1),
        }
    }
}

fn build_graph(indexing: GraphIndexing, grid: &[Vec<u8>]) -> Result<Vec<Vec<usize>>> {
    let mut adj = vec![vec![]; indexing.total()];
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            let pairs: &[_] = match c {
                b'.' => &[
                    (Edge::InWest, Edge::OutEast),
                    (Edge::InEast, Edge::OutWest),
                    (Edge::InNorth, Edge::OutSouth),
                    (Edge::InSouth, Edge::OutNorth),
                ],
                b'/' => &[
                    (Edge::InWest, Edge::OutNorth),
                    (Edge::InEast, Edge::OutSouth),
                    (Edge::InNorth, Edge::OutWest),
                    (Edge::InSouth, Edge::OutEast),
                ],
                b'\\' => &[
                    (Edge::InWest, Edge::OutSouth),
                    (Edge::InEast, Edge::OutNorth),
                    (Edge::InNorth, Edge::OutEast),
                    (Edge::InSouth, Edge::OutWest),
                ],
                b'-' => &[
                    (Edge::InWest, Edge::OutEast),
                    (Edge::InEast, Edge::OutWest),
                    (Edge::InNorth, Edge::OutWest),
                    (Edge::InNorth, Edge::OutEast),
                    (Edge::InSouth, Edge::OutWest),
                    (Edge::InSouth, Edge::OutEast),
                ],
                b'|' => &[
                    (Edge::InWest, Edge::OutNorth),
                    (Edge::InWest, Edge::OutSouth),
                    (Edge::InEast, Edge::OutNorth),
                    (Edge::InEast, Edge::OutSouth),
                    (Edge::InNorth, Edge::OutSouth),
                    (Edge::InSouth, Edge::OutNorth),
                ],
                _ => bail!("invalid grid character: {:?}", c),
            };

            for &(from, to) in pairs {
                adj[indexing.index(x, y, from)].push(indexing.index(x, y, to));
            }
        }
    }

    Ok(adj)
}

fn run_bfs(adj: &[Vec<usize>], indexing: GraphIndexing, v0: usize) -> usize {
    let mut seen = vec![false; indexing.total()];
    let mut queue = VecDeque::new();
    seen[v0] = true;
    queue.push_back(v0);
    while let Some(v) = queue.pop_front() {
        for &v2 in &adj[v] {
            if !seen[v2] {
                seen[v2] = true;
                queue.push_back(v2);
            }
        }
    }

    (0..indexing.height)
        .flat_map(|x| (0..indexing.width).map(move |y| (x, y)))
        .filter(|&(x, y)| {
            [Edge::InWest, Edge::InEast, Edge::InNorth, Edge::InSouth]
                .iter()
                .any(|&edge| seen[indexing.index(x, y, edge)])
        })
        .count()
}

fn main() -> Result<()> {
    let grid: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map_ok(|line| line.into_bytes())
        .try_collect()?;

    let indexing = GraphIndexing {
        width: grid[0].len(),
        height: grid.len(),
    };
    let adj = build_graph(indexing, &grid)?;

    let part1 = run_bfs(&adj, indexing, indexing.index(0, 0, Edge::InWest));
    let part2 = (0..indexing.height)
        .flat_map(|y| [(0, y, Edge::InWest), (indexing.width - 1, y, Edge::InEast)])
        .chain((0..indexing.width).flat_map(|x| {
            [
                (x, 0, Edge::InNorth),
                (x, indexing.height - 1, Edge::InSouth),
            ]
        }))
        .map(|(x, y, edge)| run_bfs(&adj, indexing, indexing.index(x, y, edge)))
        .max()
        .unwrap_or_default();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
