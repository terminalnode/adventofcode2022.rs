use crate::solution::Solution;

pub struct Day04 { file_name: String }

type Range = (i32, i32);
type ElfPair = (Range, Range);

impl Day04 {
    fn parse_list(&self) -> Result<Vec<ElfPair>, String> {
        let ls: Vec<String> = self.read_file_as_lines()?;
        let list = ls.iter().map(|l| {
            let s: Vec<i32> = l.split(|c| c == ',' || c == '-')
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            ((s[0], s[1]), (s[2], s[3]))
        }).collect::<Vec<ElfPair>>();

        Ok(list)
    }
}

impl Solution for Day04 {
    fn new(file_name: String) -> Self { Day04 { file_name } }
    fn get_file_name(&self) -> String { self.file_name.clone() }

    fn part1(&self) -> Result<String, String> {
        let list = match self.parse_list() {
            Ok(it) => it,
            Err(e) => return Err(e),
        };

        let result = list.iter().filter(|pair| {
            let (r1, r2) = pair;
            let r1_in_r2 = r1.0 >= r2.0 && r1.1 <= r2.1;
            let r2_in_r1 = r2.0 >= r1.0 && r2.1 <= r1.1;
            r1_in_r2 || r2_in_r1
        }).count();
        Ok(format!("Result: {}", result))
    }

    fn part2(&self) -> Result<String, String> {
        let list = match self.parse_list() {
            Ok(it) => it,
            Err(e) => return Err(e),
        };

        let result = list.iter().filter(|pair| {
            let (r1, r2) = pair;
            let r1p1_in_r2 = r1.0 >= r2.0 && r1.0 <= r2.1;
            let r1p2_in_r2 = r1.1 >= r2.0 && r1.1 <= r2.1;
            let r2p1_in_r1 = r2.0 >= r1.0 && r2.0 <= r1.1;

            // Pretty sure I can get away without checking r2p2_in_r1, works for my input at least
            r1p1_in_r2 || r1p2_in_r2 || r2p1_in_r1
        }).count();
        Ok(format!("Result: {}", result))
    }
}
