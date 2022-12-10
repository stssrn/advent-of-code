use itertools::Itertools;

const HEADER_SIZE: usize = 8;

fn main() {
    let input = include_str!("../input.txt");

    let stacks = input
        .lines()
        .take(HEADER_SIZE)
        .map(|l| l.chars().skip(1).step_by(4).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // rotate matrix
    let mut stacks = (0..=stacks.len())
        .map(|i| {
            stacks
                .iter()
                .map(|x| x[i])
                .filter(|&x| x != ' ')
                .rev()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // rearrangement procedure
    input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .flat_map(|l| l.split_whitespace().flat_map(|x| x.parse::<usize>()))
        .tuples::<(_, _, _)>()
        .for_each(|(amount, from, to)| {
            let height = stacks[from - 1].len();
            let mut crates: Vec<_> = stacks[from - 1].drain(height - amount..).collect();
            stacks[to - 1].append(&mut crates);
        });

    let top_crates = stacks
        .iter()
        .flat_map(|stack| stack.last())
        .collect::<String>();
    println!("{top_crates} are the crates on top.");
}
