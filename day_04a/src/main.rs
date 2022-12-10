use itertools::Itertools;

fn fully_contained<T: PartialOrd>(x1: T, x2: T, y1: T, y2: T) -> bool {
    (x1 <= y1 && x2 >= y2) || (y1 <= x1 && y2 >= x2)
}

fn main() {
    let sum = include_str!("../input.txt")
        .lines()
        .flat_map(|l| l.split(&[',', '-']))
        .flat_map(|x| x.parse::<u8>())
        .tuples::<(_, _, _, _)>()
        .filter(|(a, b, c, d)| fully_contained(a, b, c, d))
        .count();
    println!("{sum} pairs fully overlap");
}
