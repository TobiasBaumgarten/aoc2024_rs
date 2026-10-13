const SEARCH_WORD: &'static str = "XMAS";

struct SearchWord<'a> {
    word: &'a str,
    rev: String,
}

impl<'a> SearchWord<'a> {
    pub fn new(word: &'a str) -> Self {
        Self {
            word,
            rev: word.chars().rev().collect::<String>(),
        }
    }

    pub fn count(&self, line: &str) -> u32 {
        line.matches(self.word).count() as u32 + line.matches(&self.rev).count() as u32
    }
}

fn rows(input: &str, sw: &SearchWord) -> u32 {
    input.lines().map(|l| sw.count(l)).sum::<u32>()
}

fn columns(input: &str, sw: &SearchWord) -> u32 {
    let grid: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let mut buff_string = String::with_capacity(grid.len());
    let mut count = 0;
    let width = grid[0].len();
    for i in 0..width {
        buff_string.clear();
        buff_string.extend(grid.iter().map(|row| row[i] as char));
        dbg!(&buff_string);
        count += sw.count(&buff_string);
    }

    count
}

fn diagonal(input: &str, sw: &SearchWord) -> u32 {
    let grid: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let mut count = 0;
    let width = grid[0].len();
    let buff_string = String::with_capacity(width + grid.len());

    count
}

#[cfg(test)]
mod tests {
    use crate::day_04;

    use super::*;
    const INPUT: &'static str = include_str!(concat!(env!("OUT_DIR"), "/aoc_input/day_04.txt"));
    const EXAMPLE: &'static str = "MMMSXXMASM
    MSAMXMSMSA
    AMXSXMAAMM
    MSAMASMSMX
    XMASAMXAMM
    XXAMMXXAMA
    SMSMSASXSS
    SAXAMASAAA
    MAMMMXMMMM
    MXMXAXMASX";

    #[test]
    fn multi_word() {
        let test_line = "XMAS..SAMX";
        assert_eq!(2, SearchWord::new(SEARCH_WORD).count(test_line));
    }

    #[test]
    fn rows() {
        let test_lines = "XMASXMAS
        SAMXSAMX";
        let sw = SearchWord::new(SEARCH_WORD);
        assert_eq!(4, day_04::rows(test_lines, &sw));
    }

    #[test]
    fn columns() {
        let test_lines = "X..S
M..A
A..M
S..X";

        let sw = SearchWord::new(SEARCH_WORD);
        assert_eq!(2, day_04::columns(test_lines, &sw));
    }
}
