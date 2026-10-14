use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod aoc {
    use super::*;
    #[pymodule]
    mod day_04 {
        use aoc2024_rs::day_04;
        use pyo3::prelude::*;

        /// Counts a word (`needle`) in a given hay stack.
        #[pyfunction]
        pub fn count_words(hay: &str, needle: &str) -> PyResult<u32> {
            Ok(day_04::count_words(hay, needle))
        }
    }
}
