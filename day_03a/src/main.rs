use std::collections::HashSet;

fn find_mistake(content: &str) -> Option<char> {
    let size = content.len();
    let mut content_iter = content.chars();
    let left = HashSet::<char>::from_iter(content_iter.by_ref().take(size / 2));
    let right = HashSet::from_iter(content_iter);
    let mistake = &left & &right;
    mistake.into_iter().next()
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
        .flat_map(find_mistake)
        .flat_map(char_to_priority)
        .sum();
    println!("The priority sum is {sum}")
}
