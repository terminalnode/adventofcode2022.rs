use itertools::Itertools;

use crate::solution::Solution;

pub struct Day11 { file_name: String }

impl Day11 {
}

impl Solution for Day11 {
    fn new(file_name: String) -> Self { Day11 { file_name } }
    fn get_file_name(&self) -> String { self.file_name.clone() }
}
