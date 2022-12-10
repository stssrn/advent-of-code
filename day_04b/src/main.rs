use itertools::Itertools;

fn main() {
    let sum = include_str!("../input.txt")
        .lines()
        .flat_map(|l| l.split(&[',', '-']))
        .flat_map(|x| x.parse::<u8>())
        .tuples::<(_, _, _, _)>()
        .filter(|(x1, x2, y1, y2)| x1 <= y2 && y1 <= x2)
        .count();
    println!("{sum} pairs partially overlap");
}
