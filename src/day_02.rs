pub fn is_safe(input: &Vec<i32>) -> bool {
    let diffs: Vec<_> = input.windows(2).map(|slice| slice[0] - slice[1]).collect();
    if diffs.iter().all(|v| v.is_positive()) || diffs.iter().all(|v| v.is_negative()) {
        return diffs.iter().all(|v| v.abs() <= 3);
    }
    false
}

pub struct Report {
    pub levels: Vec<i32>,
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
                is_safe(&report.levels),
                "test of report number {index} failed"
            );
        }
    }

    #[test]
    fn test_is_sage_input() {
        let count = INPUT
            .lines()
            .map(|l| Report::from(l))
            .filter(|r| is_safe(&r.levels))
            .count();
        assert_eq!(count, 624)
    }
}
