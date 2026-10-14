//! Solving the [day 04](https://adventofcode.com/2024/day/4) puzzle.
//!
//! Here you can find the challenge description from the website
//!
//!
#![doc = include_str!(concat!(env!("OUT_DIR"), "/challenges/", file!(), ".md"))]

pub const SEARCH_WORD: &str = "XMAS";

/// Helper struct
struct Needle<'a> {
    word: &'a str,
    rev: String,
}

impl<'a> Needle<'a> {
    pub fn new(word: &'a str) -> Self {
        Self {
            word,
            rev: word.chars().rev().collect::<String>(),
        }
    }

    pub fn count(&self, line: &str) -> u32 {
        if line.len() < self.word.len() {
            return 0;
        }
        self.positions(line).count() as u32
    }

    /// Gives the position of the center
    /// if there isn't a needle in the hay the vector will be empty
    pub fn positions(&self, line: &str) -> impl Iterator<Item = usize> {
        line.match_indices(&self.word)
            .chain(line.match_indices(&self.rev))
            .map(|(i, _)| i)
    }
}

/// handles the rows of the input
fn rows(input: &str, needle: &Needle) -> u32 {
    input.lines().map(|l| needle.count(l)).sum::<u32>()
}

/// handles the columns of the input
fn columns(input: &str, sw: &Needle) -> u32 {
    let grid: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let mut buff_string = String::with_capacity(grid.len());
    let mut count = 0;
    let width = grid[0].len();
    for i in 0..width {
        buff_string.clear();
        buff_string.extend(grid.iter().map(|row| row[i] as char));
        count += sw.count(&buff_string);
    }

    count
}

fn diagonal(input: &str, needle: &Needle) -> u32 {
    // The idea here is to walk from left to right and then from right to left
    // in a diagonal. To handle all cases i cannot start at 0,0
    // because of this i have to start at -height till 0 till width
    // here we skip all corrdinates
    let grid: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let mut count = 0;
    let width = grid[0].len();
    let height = grid.len();
    let size = width + height;
    let mut buff_string = String::with_capacity(size);
    for x_off in 0..size {
        // helper function to reduce repetition
        // the dir have to be 1 or -1
        // so i can walk from left to right and right to left.
        let mut build_dia = |x: i32, dir: i32| {
            buff_string.clear();
            for (y, _) in grid.iter().enumerate().take(height) {
                let x = x + (y as i32 * dir);
                // let x: i32 = width as i32 - size as i32 + x_off as i32 + y as i32;
                if x < 0_i32 || x >= width as i32 {
                    continue;
                }
                buff_string.push(grid[y][x as usize] as char);
            }
            count += needle.count(&buff_string);
        };
        build_dia(width as i32 - size as i32 + x_off as i32, 1);
        build_dia(x_off as i32, -1);
    }
    count
}

fn diagonal_cross(input: &str, needle: &Needle) -> usize {
    // The idea here is to walk from left to right and then from right to left
    // in a diagonal. To handle all cases i cannot start at 0,0
    // because of this i have to start at -height till 0 till width
    // here we skip all corrdinates
    let grid: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let mut positions = Vec::new();
    let width = grid[0].len();
    let height = grid.len();
    let size = width + height;
    let mut buff_string = String::with_capacity(size);
    for x_off in 0..size {
        // helper function to reduce repetition
        // the dir have to be 1 or -1
        // so i can walk from left to right and right to left.
        let mut build_dia = |x: i32, dir: i32| {
            buff_string.clear();
            for (y, _) in grid.iter().enumerate().take(height) {
                let x = x + (y as i32 * dir);
                // let x: i32 = width as i32 - size as i32 + x_off as i32 + y as i32;
                if x < 0_i32 || x >= width as i32 {
                    // here we have to push a filler instead
                    buff_string.push(' ');
                    continue;
                }
                buff_string.push(grid[y][x as usize] as char);
            }
            positions.extend(
                needle.positions(&buff_string).map(|i| {
                    // return i;
                    let offset = (i + 1) as i32;
                    (x + offset * dir, offset)
                }),
                // here we hardcoded the center of M*A*S
            );
        };
        build_dia(width as i32 - size as i32 + x_off as i32, 1);
        build_dia(x_off as i32, -1);
    }
    let original_length = positions.len();
    positions.sort_unstable();
    positions.dedup();
    original_length - positions.len()
}

/// Counts a word (`needle`) in a given hay stack.
pub fn count_words(hay: &str, needle: &str) -> u32 {
    let needle = Needle::new(needle);
    rows(hay, &needle) + columns(hay, &needle) + diagonal(hay, &needle)
}

/// counts all crossing needles
pub fn find_crossing(hey: &str, needle: &str) -> usize {
    assert!(
        needle.len() == 3,
        "currently only supports a 3 length needle"
    );
    let needle = Needle::new(needle);
    diagonal_cross(hey, &needle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_04;
    const INPUT: &str = include_str!(concat!(env!("OUT_DIR"), "/aoc_input/", file!(), ".txt"));
    const EXAMPLE: &'static str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    const EXAMPLE_TWO: &'static str = "\
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

    #[test]
    fn example_one() {
        let count = count_words(EXAMPLE, SEARCH_WORD);
        assert_eq!(count, 18);
    }

    #[test]
    fn example_two() {
        let count = find_crossing(EXAMPLE_TWO, "MAS");
        assert_eq!(count, 9);
    }

    #[test]
    fn input_one() {
        let count = count_words(INPUT, SEARCH_WORD);
        assert_eq!(count, 2551);
    }

    #[test]
    fn input_two() {
        assert_eq!(1985, find_crossing(INPUT, "MAS"));
    }

    #[test]
    fn multi_word() {
        let test_line = "XMAS..SAMX";
        assert_eq!(2, Needle::new(SEARCH_WORD).count(test_line));
    }

    #[test]
    fn rows() {
        let test_lines = "XMASXMAS
        SAMXSAMX";
        let sw = Needle::new(SEARCH_WORD);
        assert_eq!(4, day_04::rows(test_lines, &sw));
    }

    #[test]
    fn columns() {
        let test_lines = "X..S
M..A
A..M
S..X";

        let sw = Needle::new(SEARCH_WORD);
        assert_eq!(2, day_04::columns(test_lines, &sw));
    }

    #[test]
    fn diagonal() {
        let test_lines = "X..S
.MA.
.MA.
X..S";
        let sw = Needle::new(SEARCH_WORD);
        assert_eq!(2, day_04::diagonal(test_lines, &sw));
    }

    #[test]
    fn diagonal_cross() {
        let test_lines = "\
M.S
.A.
M.S";
        let needle = Needle::new("MAS");
        assert_eq!(1, day_04::diagonal_cross(test_lines, &needle));
    }
}
