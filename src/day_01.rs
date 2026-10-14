//! Solving the [day 01](https://adventofcode.com/2024/day/1) puzzle.
//!
//! Here you can find the challenge description from the website
//!
//!
#![doc = include_str!(concat!(env!("OUT_DIR"), "/challenges/", file!(), ".md"))]
use std::{
    error::Error,
    fmt::{Debug, Display},
    iter::Sum,
    ops::Sub,
};

#[derive(Debug)]
pub enum DistanceError {
    LengthMismatch { left: usize, right: usize },
}

impl Display for DistanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DistanceError::LengthMismatch { left, right } => write!(
                f,
                "the length of the left vector with {left} doesn't math the length of the right vector {right}",
            ),
        }
    }
}

impl Error for DistanceError {}

/// Calculates the distance for part 1 of the puzzle.
/// The distance function is a bit "over engineered" because of exercise reasons
pub fn distance<T>(mut a: Vec<T>, mut b: Vec<T>) -> Result<T, DistanceError>
where
    T: Ord + Sum + Sub<T, Output = T>,
{
    if a.len() != b.len() {
        return Err(DistanceError::LengthMismatch {
            left: a.len(),
            right: b.len(),
        });
    }
    a.sort_unstable();
    b.sort_unstable();
    let dist = a
        .into_iter()
        .zip(b)
        .map(|(av, bv)| if av > bv { av - bv } else { bv - av })
        .sum();
    Ok(dist)
}

#[derive(Debug, Default)]
#[allow(dead_code)]
/// Parsed puzzle input: the left and right columns.
///
/// # Example
///
/// ```rust
/// # use aoc2024_rs::day_01::{distance, Lists};
///
/// # let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
/// let lists = Lists::from(input);
/// assert_eq!(distance(lists.a, lists.b).unwrap(), 11);
/// ```
pub struct Lists {
    pub a: Vec<i32>,
    pub b: Vec<i32>,
}

impl From<&str> for Lists {
    fn from(value: &str) -> Self {
        let mut result = Lists::default();
        value.lines().for_each(|line| {
            let mut numbers = line.split_whitespace();
            let av = numbers
                .next()
                .expect("numbers missing")
                .parse::<i32>()
                .expect("Number cannot be parsed");
            result.a.push(av);
            let bv = numbers
                .next()
                .expect("numbers missing")
                .parse::<i32>()
                .expect("Number cannot be parsed");
            result.b.push(bv);
        });
        result
    }
}

/// calculates the similarity score for part 2.
pub fn similarity_score(a: &[i32], b: &[i32]) -> i32 {
    a.iter()
        .map(|av| b.iter().filter(|bv| &av == bv).sum::<i32>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!(concat!(env!("OUT_DIR"), "/aoc_input/", file!(), ".txt"));
    const EXAMPLE_INPUT: &'static str = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";

    #[test]
    fn test_format_example() {
        let list = Lists::from(EXAMPLE_INPUT);
        assert_eq!(list.a, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(list.b, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_distance_example() {
        let list = Lists::from(EXAMPLE_INPUT);
        let Ok(distance) = distance(list.a, list.b) else {
            panic!("Failed test");
        };
        assert_eq!(distance, 11);
    }

    #[test]
    /// Only works with my input
    fn test_distance_input() {
        let list = Lists::from(INPUT);
        let Ok(distance) = distance(list.a, list.b) else {
            panic!("Failed test");
        };

        assert_eq!(distance, 1580061, "Works only with the input of this repro");
    }

    #[test]
    fn test_similarity_score_example() {
        let list = Lists::from(EXAMPLE_INPUT);
        let score = similarity_score(&list.a, &list.b);
        assert_eq!(score, 31);
    }

    #[test]
    fn test_similarity_score_input() {
        let list = Lists::from(INPUT);
        let score = similarity_score(&list.a, &list.b);
        assert_eq!(score, 23046913);
    }
}
