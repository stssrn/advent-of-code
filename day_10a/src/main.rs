fn main() {
    let signal_strength: usize = include_str!("../input.txt")
        .lines()
        .flat_map(|x| {
            x.strip_prefix("addx ")
                .and_then(|x| x.parse::<isize>().ok())
                .map(|x| vec![0, x])
                .unwrap_or_else(|| vec![0])
        })
        .scan(1, |state, x| {
            *state += x;
            Some(*state)
        })
        .enumerate()
        .skip(18)
        .step_by(40)
        .map(|(i, x)| (i + 2) * x as usize)
        .sum();
    println!("The sum of the six signal strenghts is {signal_strength}");
}
