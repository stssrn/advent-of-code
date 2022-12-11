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
        .flat_map(|x| {
            x[0].chars()
                .find(|&c| x[1..=2].iter().all(|y| y.contains(c)))
        })
        .flat_map(char_to_priority)
        .sum();
    println!("The priority sum is {sum}")
}
