//! Solving the [day 03](https://adventofcode.com/2024/day/3) puzzle.
//!
//! Here you can find the challenge description from the website
//!
//!
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/.challenges/", file!(), ".md"))]

use regex::Regex;

/// Solves part 1 of the puzzle
pub fn uncorrupt(sequence: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(sequence)
        .map(|c| c[1].parse::<i32>().unwrap() * c[2].parse::<i32>().unwrap())
        .sum()
}

/// solves part two of the puzzle
pub fn instruction_uncorrupt(sequence: &str) -> i32 {
    let re = Regex::new(r"(?<do>do\(\))|(?<dont>don't\(\))|mul\((?<a>\d+),(?<b>\d+)\)").unwrap();
    let mut can_mul = true;
    let mut sum = 0;
    for cap in re.captures_iter(sequence) {
        if cap.name("do").is_some() {
            can_mul = true;
        } else if cap.name("dont").is_some() {
            can_mul = false;
        } else if can_mul {
            let a = cap["a"].parse::<i32>().unwrap();
            let b = cap["b"].parse::<i32>().unwrap();
            sum += a * b;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/.aoc_input/",
        file!(),
        ".txt"
    ));
    const EXAMPLE: &'static str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_2: &'static str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    #[test]
    fn test_uncorrupt_example() {
        let sum = uncorrupt(EXAMPLE);
        assert_eq!(sum, 161)
    }

    #[test]
    fn test_uncorrupt_input() {
        let sum = uncorrupt(INPUT);
        assert_eq!(sum, 188741603);
    }

    #[test]
    fn test_instruction_uncorrupt_example() {
        let sum = instruction_uncorrupt(EXAMPLE_2);
        assert_eq!(sum, 48);
    }

    #[test]
    fn test_instruction_uncorrupt_input() {
        let sum = instruction_uncorrupt(INPUT);
        assert_eq!(sum, 67269798);
    }
}
