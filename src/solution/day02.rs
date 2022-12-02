use crate::solution::solution::Solution;
use Choice::*;

// A, B, C => rock, paper, scissors
// X, Y, Z => rock, paper, scissors
// lost = 0, draw = 3, win = 6
pub struct Day02 { file_name: String }

#[derive(PartialEq, Clone, Copy)]
enum Choice {
    Rock = 1,
    Paper,
    Scissors,
}

impl Choice {
    fn new(c: char) -> Result<Choice, String> {
        // XYZ is offset by 23 from ABC
        let c32 = char::from_u32((c as u32) - 23);
        let ch = if c <= 'C' { c } else { c32.unwrap() };

        match ch {
            'A' => Ok(Rock),
            'B' => Ok(Paper),
            'C' => Ok(Scissors),
            _ => Err(format!("Failed to convert '{}' to rock/paper/scissors", ch)),
        }
    }

    fn get_loser(&self) -> Choice {
        match &self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn get_winner(&self) -> Choice {
        match &self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
}

/// Get score for a given combination.
///
/// - Loss: 0p
/// - Draw: 3p
/// - Win:  6p
fn get_score(opponent: Choice, you: Choice) -> i32 {
    let result_points = if opponent == you {
        3
    } else {
        match opponent {
            Rock => if you == Paper { 6 } else { 0 },
            Paper => if you == Scissors { 6 } else { 0 },
            Scissors => if you == Rock { 6 } else { 0 },
        }
    };

    result_points + (you as i32)
}

impl Day02 {
}

impl Solution for Day02 {
    fn new(file_name: String) -> Self { Day02 { file_name } }

    fn get_file_name(&self) -> String {
        self.file_name.clone()
    }

    fn part1(&self) -> Result<String, String> {
        let lines: Vec<String> = match self.read_file_as_lines() {
            Ok(ss) => ss,
            Err(e) => return Err(e.to_string()),
        };

        let score: i32 = lines.iter().map(|line| {
            let mut chars = line.chars();
            let opponent = Choice::new(chars.next().unwrap()).unwrap();
            let you = Choice::new(chars.next_back().unwrap()).unwrap();
            get_score(opponent, you)
        }).sum();

        Ok(format!("Total score is {}", score))
    }

    fn part2(&self) -> Result<String, String> {
        let lines: Vec<String> = match self.read_file_as_lines() {
            Ok(ss) => ss,
            Err(e) => return Err(e.to_string()),
        };

        let score: i32 = lines.iter().map(|line| {
            let mut chars = line.chars();
            let opponent = Choice::new(chars.next().unwrap()).unwrap();

            match chars.next_back().unwrap() {
                'X' => 0 + opponent.get_loser() as i32,
                'Y' => 3 + opponent as i32,
                'Z' => 6 + opponent.get_winner() as i32,
                _ => 0,
            }
        }).sum();

        Ok(format!("Total score is {}", score))
    }
}
