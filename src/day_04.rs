use regex::Regex;

const SEARCH_WORD: &'static str = "XMAS";

struct SearchWord {
    re: Regex,
}

impl SearchWord {
    pub fn new(word: &str) -> Self {
        let reverse: String = word.chars().rev().collect();
        let re = Regex::new(&format!("{}|{}", word, reverse)).unwrap();
        Self { re }
    }

    pub fn count(&self, content: &str) -> i32 {
        self.re.captures(content).iter().count() as i32
    }
}

fn count_xmax(word: &str, content: &str) -> i32 {
    let sw = SearchWord::new(word);

    let row_count = sw.count(content);
    let lines: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();
    let width = lines[0].len();
    let height = lines.len();
    let column = (0..width)
        .map(|index| lines.iter().map(|row| row[index]).collect::<String>())
        .collect::<String>();
    let colum_count = sw.count(&column);

    // left dia
    // right dia

    row_count + colum_count
}

#[cfg(test)]
mod tests {
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
    fn test_contains_xmas() {
        assert!(contains_xmas("MMMSXXMASM"));
    }
}
