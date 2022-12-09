use itertools::Itertools;

#[derive(Clone, Copy)]
enum Moves {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Moves {
    fn score(opp: Self, me: Self) -> usize {
        let win_score = match (opp, me) {
            (Self::Rock, Self::Paper) => 6,
            (Self::Rock, Self::Scissors) => 0,
            (Self::Paper, Self::Rock) => 0,
            (Self::Paper, Self::Scissors) => 6,
            (Self::Scissors, Self::Rock) => 6,
            (Self::Scissors, Self::Paper) => 0,
            (_, _) => 3,
        };
        win_score + me as usize
    }
}

impl TryFrom<char> for Moves {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Moves::Rock),
            'B' | 'Y' => Ok(Moves::Paper),
            'C' | 'Z' => Ok(Moves::Scissors),
            _ => Err("Failed to parse char"),
        }
    }
}

fn main() {
    let total_score: usize = include_str!("../input.txt")
        .chars()
        .flat_map(Moves::try_from)
        .tuples()
        .map(|(opp, me)| Moves::score(opp, me))
        .sum();
    println!("Total score {total_score}")
}
