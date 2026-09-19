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

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!(concat!(env!("OUT_DIR"), "/aoc_input/level_01.txt"));
    const EXAMPLE_INPUT: &'static str = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";
    #[test]
    fn test_name() {}
}
