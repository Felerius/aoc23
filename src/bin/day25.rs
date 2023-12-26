use std::{collections::VecDeque, io};

use anyhow::{Context, Result};
use rand::Rng;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Default)]
struct LabelCompression<'a>(FxHashMap<&'a str, usize>);

impl<'a> LabelCompression<'a> {
    fn get(&mut self, label: &'a str) -> usize {
        let next = self.0.len();
        *self.0.entry(label).or_insert(next)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

fn run_bfs(
    adj: &[Vec<(usize, usize)>],
    queue: &mut VecDeque<usize>,
    inc: &mut [(usize, usize)],
    s: usize,
) {
    queue.push_back(s);
    inc.fill((usize::MAX, usize::MAX));
    inc[s] = (s, usize::MAX);
    while let Some(v) = queue.pop_front() {
        for (edge_idx, &(v2, _)) in adj[v].iter().enumerate() {
            if v2 < adj.len() && inc[v2].0 == usize::MAX {
                inc[v2] = (v, edge_idx);
                queue.push_back(v2);
            }
        }
    }
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin().lock())?;
    let mut labels = LabelCompression::default();
    let mut adj = Vec::new();
    for line in input.lines() {
        let (label, neighbors) = line.split_once(": ").context("invalid input")?;
        let v = labels.get(label);
        for label2 in neighbors.split_ascii_whitespace() {
            let v2 = labels.get(label2);
            adj.resize_with(labels.len(), Vec::new);

            assert_ne!(v, v2);
            let v_idx = adj[v].len();
            let v2_idx = adj[v2].len();
            adj[v].push((v2, v2_idx));
            adj[v2].push((v, v_idx));
        }
    }
    let n = adj.len();

    const MIN_CUT_VALUE: usize = 3;
    let mut rng = rand::thread_rng();
    let mut queue = VecDeque::new();
    let mut inc = vec![(0, 0); adj.len()];
    let part1 = 'outer: loop {
        for row in &mut adj {
            for (v, _) in row {
                if *v >= n {
                    *v = v.wrapping_neg();
                }
            }
        }

        let s = 0;
        let t = rng.gen_range(0..n);
        for _ in 0..MIN_CUT_VALUE {
            run_bfs(&adj, &mut queue, &mut inc, s);
            if inc[t].0 == usize::MAX {
                continue 'outer;
            }

            let mut v = t;
            while v != s {
                let (from, from_idx) = inc[v];
                let (to, to_idx) = adj[from][from_idx];
                if adj[to][to_idx].0 == from {
                    adj[from][from_idx] = (to.wrapping_neg(), to_idx);
                } else {
                    adj[to][to_idx] = (from, from_idx);
                }

                v = from;
            }
        }

        run_bfs(&adj, &mut queue, &mut inc, s);
        if inc[t].0 == usize::MAX {
            let num_reachable = inc.iter().filter(|(v, _)| *v != usize::MAX).count();
            break num_reachable * (n - num_reachable);
        }
    };

    println!("Part 1: {part1}");
    Ok(())
}
