use std::io::{self, BufRead};

use anyhow::{Context, Result};
use regex::Regex;

fn to_digit(name: &str) -> u32 {
    match name {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => u32::from(name.as_bytes()[0] - b'0'),
    }
}

fn to_digit_rev(name: &str) -> u32 {
    match name {
        "eno" => 1,
        "owt" => 2,
        "eerht" => 3,
        "ruof" => 4,
        "evif" => 5,
        "xis" => 6,
        "neves" => 7,
        "thgie" => 8,
        "enin" => 9,
        _ => u32::from(name.as_bytes()[0] - b'0'),
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin().lock();

    let regex = Regex::new("(?-u)[0-9]|one|two|three|four|five|six|seven|eight|nine")?;
    let regex_rev = Regex::new("(?-u)[0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin")?;
    let mut part1 = 0;
    let mut part2 = 0;
    for line in stdin.lines() {
        let line = line?;

        let mut digits = line.bytes().filter(u8::is_ascii_digit);
        let first_digit1 = digits.clone().next().context("no first digit")? - b'0';
        let last_digit1 = digits.next_back().context("no last digit")? - b'0';
        part1 += u32::from(first_digit1) * 10 + u32::from(last_digit1);

        let first_digit2 = to_digit(regex.find(&line).context("no first digit")?.as_str());
        let mut line_bytes = line.into_bytes();
        line_bytes.reverse();
        let line = String::from_utf8(line_bytes)?;
        let last_digit2 = to_digit_rev(regex_rev.find(&line).context("no last digit")?.as_str());
        part2 += u32::from(first_digit2) * 10 + u32::from(last_digit2);
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
