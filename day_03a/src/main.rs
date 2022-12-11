fn char_to_priority(c: char) -> Option<usize> {
    (b'a'..=b'z')
        .chain(b'A'..=b'Z')
        .position(|x| x == c as u8)
        .map(|x| x + 1)
}

fn main() {
    let sum: usize = include_str!("../input.txt")
        .lines()
        .map(|x| x.split_at(x.len() / 2))
        .flat_map(|(a, b)| a.chars().find(|&c| b.contains(c)))
        .flat_map(char_to_priority)
        .sum();
    println!("The priority sum is {sum}")
}
