use itertools::Itertools;

#[derive(Clone, Copy)]
enum GameResult {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl TryFrom<char> for GameResult {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(GameResult::Lose),
            'Y' => Ok(GameResult::Draw),
            'Z' => Ok(GameResult::Win),
            _ => Err("Failed to parse char"),
        }
    }
}

#[derive(Clone, Copy)]
enum Moves {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl TryFrom<char> for Moves {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Moves::Rock),
            'B' => Ok(Moves::Paper),
            'C' => Ok(Moves::Scissors),
            _ => Err("Failed to parse char"),
        }
    }
}

fn score(opp: Moves, me: GameResult) -> usize {
    let win_score = match (opp, me) {
        (Moves::Rock, GameResult::Win) => Moves::Paper,
        (Moves::Rock, GameResult::Lose) => Moves::Scissors,
        (Moves::Paper, GameResult::Win) => Moves::Scissors,
        (Moves::Paper, GameResult::Lose) => Moves::Rock,
        (Moves::Scissors, GameResult::Win) => Moves::Rock,
        (Moves::Scissors, GameResult::Lose) => Moves::Paper,
        (x, GameResult::Draw) => x,
    };
    win_score as usize + me as usize
}

fn main() {
    let total_score: usize = include_str!("../input.txt")
        .split_whitespace()
        .flat_map(|x| x.chars())
        .tuples()
        .map(|(opp, me)| score(opp.try_into().unwrap(), me.try_into().unwrap()))
        .sum();
    println!("Total score {total_score}")
}
