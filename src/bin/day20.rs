use std::{collections::VecDeque, io, str::FromStr};

use anyhow::{Context, Result};
use num_integer::Integer;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModuleType {
    Broadcast,
    FlipFlop,
    Conjunction,
}

#[derive(Debug, Clone)]
struct System {
    types: Vec<ModuleType>,
    wires: Vec<Vec<(usize, usize)>>,
    state_offsets: Vec<usize>,
    broadcast_module: usize,
}

impl System {
    fn state_len(&self) -> usize {
        self.state_offsets.last().copied().unwrap_or_default()
    }

    fn simulate_cycle(
        &self,
        state: &mut [bool],
        queue: &mut VecDeque<(usize, usize, bool)>,
    ) -> [usize; 2] {
        debug_assert_eq!(state.len(), self.state_len());

        let mut counts = [0; 2];
        queue.push_back((self.broadcast_module, 0, false));
        while let Some((module_idx, in_idx, high)) = queue.pop_front() {
            counts[usize::from(high)] += 1;
            match self.types[module_idx] {
                ModuleType::Broadcast => {
                    for &(to_idx, to_in_idx) in &self.wires[module_idx] {
                        queue.push_back((to_idx, to_in_idx, high));
                    }
                }
                ModuleType::FlipFlop if !high => {
                    let offset = self.state_offsets[module_idx];
                    state[offset] = !state[offset];
                    for &(to_idx, to_in_idx) in &self.wires[module_idx] {
                        queue.push_back((to_idx, to_in_idx, state[offset]));
                    }
                }
                ModuleType::FlipFlop => {}
                ModuleType::Conjunction => {
                    let state_begin = self.state_offsets[module_idx];
                    let state_end = self.state_offsets[module_idx + 1];
                    state[state_begin + in_idx] = high;
                    let all_high = state[state_begin..state_end].iter().all(|&b| b);
                    for &(to_idx, to_in_idx) in &self.wires[module_idx] {
                        queue.push_back((to_idx, to_in_idx, !all_high));
                    }
                }
            }
        }

        counts
    }
}

impl FromStr for System {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut types = Vec::new();
        let mut wires = Vec::new();
        let mut in_degrees = Vec::new();
        let mut labels = FxHashMap::default();
        for line in s.lines() {
            let (left, right) = line.split_once(" -> ").context("invalid module spec")?;

            let (label, typ) = if let Some(label) = left.strip_prefix('%') {
                (label, ModuleType::FlipFlop)
            } else if let Some(label) = left.strip_prefix('&') {
                (label, ModuleType::Conjunction)
            } else {
                (left, ModuleType::Broadcast)
            };
            let idx = *labels.entry(label).or_insert_with(|| {
                types.push(ModuleType::Broadcast);
                wires.push(Vec::new());
                in_degrees.push(0);
                types.len() - 1
            });
            types[idx] = typ;

            wires[idx] = right
                .split(", ")
                .map(|to_label| {
                    let to_idx = *labels.entry(to_label).or_insert_with(|| {
                        types.push(ModuleType::Broadcast);
                        wires.push(Vec::new());
                        in_degrees.push(0);
                        types.len() - 1
                    });
                    in_degrees[to_idx] += 1;
                    (to_idx, in_degrees[to_idx] - 1)
                })
                .collect();
        }

        let mut state_len = 0;
        for (in_degree, typ) in in_degrees.iter_mut().zip(&types) {
            let offset = state_len;
            state_len += match typ {
                ModuleType::Broadcast => 0,
                ModuleType::FlipFlop => 1,
                ModuleType::Conjunction => *in_degree,
            };
            *in_degree = offset;
        }
        in_degrees.push(state_len);

        let broadcast_module = *labels.get("broadcaster").context("no broadcaster module")?;
        Ok(Self {
            types,
            wires,
            state_offsets: in_degrees,
            broadcast_module,
        })
    }
}

#[derive(Debug, Clone, Default)]
struct IncrementalPrefixFunction<T>(Vec<(T, usize)>);

impl<T: Eq> IncrementalPrefixFunction<T> {
    fn push(&mut self, value: T) {
        let Some(mut i) = self.0.last().map(|(_, i)| *i) else {
            self.0.push((value, 0));
            return;
        };

        while i > 0 && self.0[i].0 != value {
            i = self.0[i - 1].1;
        }
        if self.0[i].0 == value {
            i += 1;
        }

        self.0.push((value, i));
    }

    fn cycle_len(&self) -> usize {
        self.0.len() - self.0.last().map(|(_, i)| *i).unwrap_or_default()
    }
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin().lock())?;
    let system = input.parse::<System>()?;

    const CYCLE_UPPER_BOUND_GUESS: usize = 10_000;
    let mut state = vec![false; system.state_len()];
    let mut queue = VecDeque::new();
    let mut part1_count_low = 0;
    let mut part1_count_high = 0;
    let mut cycles = vec![IncrementalPrefixFunction::default(); system.state_len()];
    for i in 0..CYCLE_UPPER_BOUND_GUESS.max(1000) {
        let [count_low, count_high] = system.simulate_cycle(&mut state, &mut queue);
        if i < 1000 {
            part1_count_low += count_low;
            part1_count_high += count_high;
        }

        for (cycle, s) in cycles.iter_mut().zip(&state) {
            cycle.push(*s);
        }
    }

    let part1 = part1_count_low * part1_count_high;
    let part2 = cycles
        .iter()
        .map(|c| c.cycle_len())
        .fold(1, |a, b| a.lcm(&b));

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
