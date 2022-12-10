use std::collections::HashSet;

fn find_badge<'a, T, I>(backpacks: T) -> Option<char>
where
    T: IntoIterator<Item = &'a I>,
    I: AsRef<str> + 'a,
{
    backpacks
        .into_iter()
        .map(|bp| bp.as_ref().chars().collect::<HashSet<_>>())
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
        .collect::<Vec<&str>>()
        .chunks(3)
        .flat_map(find_badge)
        .flat_map(char_to_priority)
        .sum();
    println!("The priority sum is {sum}")
}
