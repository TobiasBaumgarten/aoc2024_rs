//! Solving the [day 02](https://adventofcode.com/2024/day/2) puzzle.

pub fn is_safe(input: &Vec<i32>) -> bool {
    let diffs: Vec<_> = input.windows(2).map(|slice| slice[0] - slice[1]).collect();
    if diffs.iter().all(|v| v.is_positive()) || diffs.iter().all(|v| v.is_negative()) {
        return diffs.iter().all(|v| v.abs() <= 3);
    }
    false
}

/// Used to extract the report from the line
///
/// # Example
///
/// ```rust
/// use aoc2024_rs::day_02::Report;
///
/// let input = "7 6 4 2 1";
/// let report: Report = input.into();
/// assert_eq!(report.levels, vec![7,6,4,2,1]);
/// assert_eq!(report.is_safe(), true);
/// ```
pub struct Report {
    pub levels: Vec<i32>,
}

impl Report {
    /// solves puzzle part 1
    pub fn is_safe(&self) -> bool {
        is_safe(&self.levels)
    }

    /// solves puzzle part 2
    pub fn is_safe_tolerate(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        for index in 0..self.levels.len() {
            let temp = self
                .levels
                .iter()
                .enumerate()
                .filter(|(idx, _)| idx != &index)
                .map(|(_, v)| *v)
                .collect();
            if is_safe(&temp) {
                return true;
            }
        }
        false
    }
}

impl From<&str> for Report {
    fn from(value: &str) -> Self {
        Self {
            levels: value
                .split_whitespace()
                .map(|n| n.parse::<i32>().expect("the number has to be an integer"))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!(concat!(env!("OUT_DIR"), "/aoc_input/day_02.txt"));
    const EXAMPLE: &'static str = "7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";

    #[test]
    fn test_is_safe_example() {
        let reports: Vec<_> = EXAMPLE.lines().map(|l| Report::from(l)).collect();
        let safe_reports = vec![true, false, false, false, false, true];

        for (index, (report, safe)) in reports.iter().zip(safe_reports).enumerate() {
            assert_eq!(
                safe,
                report.is_safe(),
                "test of report number {index} failed"
            );
        }
    }

    #[test]
    fn test_is_safe_input() {
        let count = INPUT
            .lines()
            .map(|l| Report::from(l))
            .filter(|r| r.is_safe())
            .count();
        assert_eq!(count, 624)
    }

    #[test]
    fn test_is_safe_tolerate_example() {
        let reports: Vec<_> = EXAMPLE.lines().map(|l| Report::from(l)).collect();
        let safe_reports = vec![true, false, false, true, true, true];

        for (index, (report, safe)) in reports.iter().zip(safe_reports).enumerate() {
            assert_eq!(
                safe,
                report.is_safe_tolerate(),
                "test of report number {index} failed"
            );
        }
    }

    #[test]
    fn test_is_safe_tolerate_input() {
        let count = INPUT
            .lines()
            .map(|l| Report::from(l))
            .filter(|r| r.is_safe_tolerate())
            .count();
        assert_eq!(count, 658)
    }
}
