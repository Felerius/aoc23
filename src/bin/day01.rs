use std::io::{self, BufRead};

use aho_corasick::{AhoCorasick, AhoCorasickKind};
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let stdin = io::stdin().lock();
    let aho_corasick = AhoCorasick::builder()
        .kind(Some(AhoCorasickKind::DFA))
        .build(&[
            "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven",
            "7", "eight", "8", "nine", "9",
        ])?;

    let mut part1 = 0;
    let mut part2 = 0;
    for line in stdin.lines() {
        let line = line?;

        let mut digits = line.bytes().filter(u8::is_ascii_digit);
        let first_digit1 = digits.clone().next().context("no first digit")? - b'0';
        let last_digit1 = digits.next_back().context("no last digit")? - b'0';
        part1 += u32::from(first_digit1) * 10 + u32::from(last_digit1);

        let (first_digit2, last_digit2) = aho_corasick
            .find_overlapping_iter(&line)
            .fold(None, |acc, mat| {
                let digit = mat.pattern().as_u32() / 2 + 1;
                Some(acc.map_or((digit, digit), |(first, _)| (first, digit)))
            })
            .context("no digits found")?;
        part2 += u32::from(first_digit2) * 10 + u32::from(last_digit2);
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
