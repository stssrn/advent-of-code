use itertools::Itertools;

fn main() {
    let cals: u32 = include_str!("../input.txt")
        .split("\n\n")
        .map(|x| {
            x.split_whitespace()
                .flat_map(|y| y.parse::<u32>())
                .sum::<u32>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum();
    println!("The top 3 elfs are carrying {cals} cals total");
}
