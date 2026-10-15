use std::{
    cmp::Ordering::{self, Greater},
    collections::{HashMap, HashSet},
};

#[derive(Debug, Default, Clone)]
pub struct SafetyScanner {
    // Key: Rule Head, Value: All that have to be after/lower
    rules: HashMap<u32, HashSet<u32>>,
    sequences: Vec<Vec<u32>>,
}

impl SafetyScanner {
    pub fn new(input: &str) -> Self {
        let mut s = Self::default();
        let mut parse_switch = false;
        for line in input.lines() {
            if line.is_empty() {
                parse_switch = true;
                continue;
            }
            if !parse_switch {
                let (key, value) = Self::parse_rule(line);
                s.rules.entry(key).or_default().insert(value);
            } else {
                s.sequences.push(Self::parse_sequence(line));
            }
        }
        s
    }

    fn get_valid_sequences(&self) -> impl Iterator<Item = &Vec<u32>> {
        self.sequences.iter().filter(|s| self.check_sequence(s))
    }

    /// used for [`Self::get_fixed_sequences`]
    fn compare(&self, a: &u32, b: &u32) -> Ordering {
        if self.rules.get(a).is_some_and(|after| after.contains(b)) {
            Ordering::Less
        } else if self.rules.get(b).is_some_and(|after| after.contains(a)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    /// fixes the sequence by reordering
    fn get_fixed_sequences(&self) -> impl Iterator<Item = Vec<u32>> {
        let incorrect = self.sequences.iter().filter(|s| !self.check_sequence(s));
        incorrect.map(|sequence| {
            let mut sequence = sequence.clone();
            sequence.sort_by(|a, b| self.compare(a, b));
            sequence
        })
    }

    pub fn middle_sum_incorrect(&self) -> u32 {
        self.get_fixed_sequences().map(|s| s[s.len() / 2]).sum()
    }

    /// the function for the first part of the puzzle
    /// gives the sum of all middle numbers that are valid valid
    pub fn middle_sum_correct(&self) -> u32 {
        self.get_valid_sequences().map(|s| s[s.len() / 2]).sum()
    }

    fn check_sequence(&self, sequence: &[u32]) -> bool {
        sequence.is_sorted_by(|a, b| self.compare(a, b) != Greater)
    }

    fn parse_rule(line: &str) -> (u32, u32) {
        let mut numbers = line
            .trim() // the aoc input doesn't need this, but i think its a good habit.
            .split('|')
            .map(|s| s.parse::<u32>().expect("to be an type of u32"));
        assert_eq!(
            numbers.clone().count(),
            2usize,
            "the rule set should be defined with two numbers as `A | B` where A and B are u32"
        );
        (numbers.next().unwrap(), numbers.next().unwrap())
    }

    fn parse_sequence(line: &str) -> Vec<u32> {
        line.split(",")
            .map(|s| s.trim().parse::<u32>().expect("to be an type of u32"))
            .collect()
    }
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

    #[test]
    fn test_parse_rule() {
        assert_eq!((53, 13), SafetyScanner::parse_rule("53|13"));
    }

    #[test]
    #[should_panic]
    fn test_parse_rule_panic() {
        SafetyScanner::parse_rule("53|12|13");
    }

    #[test]
    fn test_example() {
        let s = SafetyScanner::new(EXAMPLE);
        assert_eq!(3, s.get_valid_sequences().count());
        assert_eq!(143, s.middle_sum_correct());
        assert_eq!(123, s.middle_sum_incorrect());
    }

    #[test]
    fn test_input_one() {
        let s = SafetyScanner::new(INPUT);
        assert_eq!(5639, s.middle_sum_correct());
    }

    #[test]
    fn test_input_two() {
        let s = SafetyScanner::new(INPUT);
        assert_eq!(5273, s.middle_sum_incorrect());
    }

    #[test]
    fn test_example_parsing() {
        let s = SafetyScanner::new(EXAMPLE);
        // check if there a 6 sequences
        assert_eq!(s.sequences.len(), 6);
        // check if all rules are in it
        assert_eq!(21usize, s.rules.iter().map(|(_, set)| set.len()).sum());
    }

    const EXAMPLE: &str = "\
    47|53
    97|13
    97|61
    97|47
    75|29
    61|13
    75|53
    29|13
    97|29
    53|29
    61|53
    97|53
    61|29
    47|13
    75|47
    97|75
    47|61
    75|61
    47|29
    75|13
    53|13

    75,47,61,53,29
    97,61,53,29,13
    75,29,13
    75,97,47,61,53
    61,13,29
    97,13,75,29,47";
}
