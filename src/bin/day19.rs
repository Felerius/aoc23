use std::{io, ops::Add, str::FromStr};

use anyhow::{bail, Context, Ok, Result};
use itertools::Itertools;
use rustc_hash::FxHashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Category {
    ExtremlyGoodLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

impl Category {
    fn as_index(self) -> usize {
        match self {
            Self::ExtremlyGoodLooking => 0,
            Self::Musical => 1,
            Self::Aerodynamic => 2,
            Self::Shiny => 3,
        }
    }
}

impl FromStr for Category {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "x" => Ok(Self::ExtremlyGoodLooking),
            "m" => Ok(Self::Musical),
            "a" => Ok(Self::Aerodynamic),
            "s" => Ok(Self::Shiny),
            _ => bail!("invalid category: {}", s),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Rule {
    category: Category,
    larger: bool,
    value: u32,
    workflow: usize,
}

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<Rule>,
    fallback: usize,
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin().lock())?;

    let workflow_labels: FxHashMap<_, _> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| Ok(line.split_once('{').context("invalid workflow spec")?.0))
        .chain([Ok("A"), Ok("R")])
        .enumerate()
        .map(|(i, label)| Ok((label?, i)))
        .try_collect()?;
    let in_workflow = workflow_labels["in"];
    let accept_workflow = workflow_labels["A"];

    let workflows: Vec<_> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (_label, tail) = line.split_once('{').context("invalid workflow")?;
            let mut rules_iter = tail.trim_end_matches('}').split(',');
            let fallback = rules_iter.next_back().context("empty workflow")?;
            let fallback = workflow_labels[fallback];

            let rules: Vec<_> = rules_iter
                .map(|rule_spec| {
                    let (predicate, workflow) =
                        rule_spec.split_once(':').context("invalid rule")?;
                    let category = predicate[..1].parse::<Category>()?;
                    let larger = &predicate[1..2] == ">";
                    let value = predicate[2..].parse::<u32>()?;
                    let workflow = workflow_labels[workflow];
                    Ok(Rule {
                        category,
                        larger,
                        value,
                        workflow,
                    })
                })
                .try_collect()?;

            Ok(Workflow { rules, fallback })
        })
        .try_collect()?;

    let part1 = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            let (x, m, a, s) = line[1..]
                .trim_end_matches('}')
                .split(',')
                .map(|part| part[2..].parse::<u32>())
                .collect_tuple()
                .context("invalid part")?;
            Ok([x?, m?, a?, s?])
        })
        .filter_ok(|&part| {
            let mut workflow_index = in_workflow;
            while let Some(workflow) = workflows.get(workflow_index) {
                workflow_index = workflow
                    .rules
                    .iter()
                    .find_map(|rule| {
                        let value = part[rule.category.as_index()];
                        let matches = if rule.larger {
                            value > rule.value
                        } else {
                            value < rule.value
                        };
                        matches.then_some(rule.workflow)
                    })
                    .unwrap_or(workflow.fallback);
            }
            workflow_index == accept_workflow
        })
        .map_ok(|part| part.into_iter().sum::<u32>())
        .fold_ok(0, Add::add)?;

    let mut part2 = 0;
    let mut queue = vec![(in_workflow, [(); 4].map(|_| 1..4001))];
    'outer: while let Some((workflow_index, mut part_spec)) = queue.pop() {
        let Some(workflow) = workflows.get(workflow_index) else {
            if workflow_index == accept_workflow {
                part2 += part_spec
                    .iter()
                    .map(|range| u64::from(range.end - range.start))
                    .product::<u64>();
            }
            continue;
        };

        for rule in &workflow.rules {
            let cat_index = rule.category.as_index();
            let mut split_spec = part_spec.clone();
            if rule.larger {
                split_spec[cat_index].start = rule.value + 1;
                part_spec[cat_index].end = rule.value + 1;
            } else {
                split_spec[cat_index].end = rule.value;
                part_spec[cat_index].start = rule.value;
            }

            if !split_spec[cat_index].is_empty() {
                queue.push((rule.workflow, split_spec));
            }
            if part_spec[cat_index].is_empty() {
                continue 'outer;
            }
        }

        queue.push((workflow.fallback, part_spec));
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
