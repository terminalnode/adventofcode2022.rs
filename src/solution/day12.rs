use itertools::Itertools;
use crate::solution::Solution;

pub struct Day12 { file_name: String }

impl Day12 {
}

impl Solution for Day12 {
    fn new(file_name: String) -> Self { Day12 { file_name } }
    fn get_file_name(&self) -> String { self.file_name.clone() }
}
