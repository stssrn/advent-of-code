fn main() {
    let cals: u32 = include_str!("../input.txt")
        .split("\n\n")
        .map(|x| x.split_whitespace().flat_map(|y| y.parse::<u32>()).sum())
        .max()
        .unwrap();
    println!("The elf carrying the most calories carries {cals} cals");
}
