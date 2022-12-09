use itertools::Itertools;
use std::collections::HashSet;

fn find_badge(bp1: &str, bp2: &str, bp3: &str) -> Option<char> {
    let one = HashSet::<char>::from_iter(bp1.chars());
    let two = HashSet::from_iter(bp2.chars());
    let three = HashSet::from_iter(bp3.chars());
    let mut badge = &one & &two;
    badge = &badge & &three;
    badge.into_iter().next()
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
        .tuples::<(_, _, _)>()
        .flat_map(|(a, b, c)| find_badge(a, b, c))
        .flat_map(char_to_priority)
        .sum();
    println!("The priority sum is {sum}")
}
