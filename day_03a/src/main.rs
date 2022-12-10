use std::collections::HashSet;

fn find_mistake(left: &str, right: &str) -> Option<char> {
    [left, right]
        .into_iter()
        .map(|x| x.chars().collect::<HashSet<_>>())
        .reduce(|acc, x| &acc & &x)
        .unwrap()
        .into_iter()
        .next()
}

fn char_to_priority(c: char) -> Option<usize> {
    (b'a'..=b'z')
        .chain(b'A'..=b'Z')
        .position(|x| x == c as u8)
        .map(|x| x + 1)
}

fn main() {
    let sum: usize = include_str!("../input.txt")
        .split_whitespace()
        .map(|x| x.split_at(x.len() / 2))
        .flat_map(|(a, b)| find_mistake(a, b))
        .flat_map(char_to_priority)
        .sum();
    println!("The priority sum is {sum}")
}
