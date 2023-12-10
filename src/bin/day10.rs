use std::{
    collections::VecDeque,
    io::{self, BufRead},
    mem, vec,
};

use anyhow::{anyhow, bail, ensure, Context, Result};

fn parse_graph() -> Result<(Vec<Vec<Option<[(usize, usize); 2]>>>, (usize, usize))> {
    let mut start = None;
    let mut graph = io::stdin()
        .lock()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let line = line?;
            line.bytes()
                .enumerate()
                .map(|(x, b)| {
                    let neighbors = match b {
                        b'|' => Some([(y.wrapping_sub(1), x), (y + 1, x)]),
                        b'-' => Some([(y, x.wrapping_sub(1)), (y, x + 1)]),
                        b'L' => Some([(y.wrapping_sub(1), x), (y, x + 1)]),
                        b'J' => Some([(y.wrapping_sub(1), x), (y, x.wrapping_sub(1))]),
                        b'7' => Some([(y + 1, x), (y, x.wrapping_sub(1))]),
                        b'F' => Some([(y + 1, x), (y, x + 1)]),
                        b'.' => None,
                        b'S' => {
                            ensure!(start.is_none(), "multiple start tiles");
                            start = Some((y, x));
                            None
                        }
                        _ => bail!("invalid tile: {:?}", char::from(b)),
                    };
                    Ok(neighbors)
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;

    let start = start.context("no start tile")?;
    let (sy, sx) = start;
    let start_neighbors: Vec<_> = [
        (sy.wrapping_sub(1), sx),
        (sy + 1, sx),
        (sy, sx.wrapping_sub(1)),
        (sy, sx + 1),
    ]
    .into_iter()
    .filter(|&(y, x)| {
        graph
            .get(y)
            .and_then(|row| row.get(x).copied().flatten())
            .into_iter()
            .flatten()
            .any(|neighbor| neighbor == start)
    })
    .collect();
    graph[sy][sx] = Some(
        start_neighbors
            .try_into()
            .map_err(|_| anyhow!("start does not have exactly two neighbors"))?,
    );

    Ok((graph, start))
}

fn part1(
    graph: &[Vec<Option<[(usize, usize); 2]>>],
    start: (usize, usize),
) -> (usize, Vec<Vec<bool>>) {
    let height = graph.len();
    let width = graph[0].len();
    let mut seen = vec![vec![false; width]; height];
    let mut queue = VecDeque::new();
    let mut max_dist = 0;
    seen[start.0][start.1] = true;
    queue.push_back((start, 0));
    while let Some(((y, x), distance)) = queue.pop_front() {
        max_dist = distance;
        for (ny, nx) in graph[y][x].unwrap() {
            if !seen[ny][nx] {
                seen[ny][nx] = true;
                queue.push_back(((ny, nx), distance + 1));
            }
        }
    }

    (max_dist, seen)
}

fn part2(graph: &[Vec<Option<[(usize, usize); 2]>>], is_main_loop: &[Vec<bool>]) -> Result<usize> {
    let height = graph.len();
    let width = graph[0].len();
    let num_tiles = height * width;
    let num_corners = (height + 1) * (width + 1);
    let num_total = num_tiles + num_corners;
    let mut adj = vec![Vec::new(); num_total];
    let mut empty_tiles = 0;

    // Collect neighbors of empty tiles and blocked corner neighbors of corners
    for y in 0..height {
        for x in 0..width {
            let tile = y * width + x;
            let corners = [(y, x), (y, x + 1), (y + 1, x), (y + 1, x + 1)]
                .map(|(ny, nx)| ny * (width + 1) + nx + num_tiles);
            let [top_left, top_right, bottom_left, bottom_right] = corners;
            if let Some(neighbors) = graph[y][x].filter(|_| is_main_loop[y][x]) {
                for neighbor in neighbors {
                    let (corner1, corner2) = if neighbor == (y.wrapping_sub(1), x) {
                        (top_left, top_right)
                    } else if neighbor == (y + 1, x) {
                        (bottom_left, bottom_right)
                    } else if neighbor == (y, x.wrapping_sub(1)) {
                        (top_left, bottom_left)
                    } else if neighbor == (y, x + 1) {
                        (top_right, bottom_right)
                    } else {
                        bail!("invalid neighbor {neighbor:?} for tile ({y}, {x})");
                    };

                    adj[corner1].push(corner2);
                    adj[corner2].push(corner1);
                }
            } else {
                empty_tiles += 1;
                adj[tile] = corners.into_iter().collect();
            }
        }
    }

    // Turn blocked corner neighbors of corners into non-blocked corner neighbors
    for y in 0..=height {
        for x in 0..=width {
            let corner = y * (width + 1) + x + num_tiles;
            adj[corner] = [
                (y.wrapping_sub(1), x),
                (y + 1, x),
                (y, x.wrapping_sub(1)),
                (y, x + 1),
            ]
            .into_iter()
            .filter(|&(ny, nx)| ny < height && nx < width)
            .map(|(ny, nx)| ny * (width + 1) + nx + num_tiles)
            .filter(|neighbor| !adj[corner].contains(neighbor))
            .collect();
        }
    }

    // Add corner -> empty tile edges
    for y in 0..height {
        for x in 0..width {
            let tile = y * width + x;
            let tile_adj = mem::take(&mut adj[tile]);
            for &corner in &tile_adj {
                adj[corner].push(tile);
            }
            adj[tile] = tile_adj;
        }
    }

    let mut seen = vec![false; num_total];
    let mut queue: VecDeque<_> = (0..=height)
        .flat_map(|y| [(y, 0), (y, width)])
        .chain((0..=width).flat_map(|x| [(0, x), (height, x)]))
        .map(|(y, x)| y * (width + 1) + x + num_tiles)
        .inspect(|&corner| seen[corner] = true)
        .collect();
    while let Some(v) = queue.pop_front() {
        for &v2 in &adj[v] {
            if !seen[v2] {
                seen[v2] = true;
                queue.push_back(v2);
            }
        }
    }

    let seen_tiles = seen[..num_tiles].iter().filter(|&&b| b).count();
    Ok(empty_tiles - seen_tiles)
}

fn main() -> Result<()> {
    let (graph, start) = parse_graph()?;
    let (part1, is_main_loop) = part1(&graph, start);
    let part2 = part2(&graph, &is_main_loop)?;

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
