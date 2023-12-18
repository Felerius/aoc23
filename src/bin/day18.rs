use std::io::{self, BufRead};

use anyhow::{bail, Context, Ok, Result};
use itertools::Itertools;

fn polygon_area(points: &[(i64, i64)]) -> i64 {
    let mut area = 0;
    let mut outline = 0;
    for ((x1, y1), (x2, y2)) in points.iter().copied().tuple_windows() {
        outline += (x2 - x1).abs() + (y2 - y1).abs();
        area += (y1 + y2) * (x1 - x2);
    }
    (area.abs() + outline) / 2 + 1
}

fn main() -> Result<()> {
    let mut points1 = vec![(0, 0)];
    let mut points2 = vec![(0, 0)];
    for line in io::stdin().lock().lines() {
        let line = line?;
        let (dir1, length1, color) = line
            .split_ascii_whitespace()
            .collect_tuple()
            .context("invalid input")?;

        let length1 = length1.parse::<i64>()?;
        let (x1, y1) = points1
            .last()
            .copied()
            .expect("points1 should never be empty");
        let p1 = match dir1 {
            "R" => (x1 + length1, y1),
            "D" => (x1, y1 + length1),
            "L" => (x1 - length1, y1),
            "U" => (x1, y1 - length1),
            _ => bail!("invalid direction: {dir1:?}"),
        };
        points1.push(p1);

        let dir2 = color.as_bytes()[7] - b'0';
        let length2 = i64::from_str_radix(&color[2..7], 16)?;
        let (x2, y2) = points2
            .last()
            .copied()
            .expect("points2 should never be empty");
        let p2 = match dir2 {
            0 => (x2 + length2, y2),
            1 => (x2, y2 + length2),
            2 => (x2 - length2, y2),
            3 => (x2, y2 - length2),
            _ => bail!("invalid direction: {dir2:?}"),
        };
        points2.push(p2);
    }

    assert_eq!(points1.last(), points1.first());
    let part1 = polygon_area(&points1);

    assert_eq!(points2.last(), points2.first());
    let part2 = polygon_area(&points2);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
