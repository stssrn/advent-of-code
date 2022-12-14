use std::collections::HashSet;

const WINDOW_SIZE: usize = 4;

fn main() {
    let index = include_bytes!("../input.txt")
        .windows(WINDOW_SIZE)
        .map(HashSet::<_>::from_iter)
        .take_while(|x| x.len() != WINDOW_SIZE)
        .count()
        + WINDOW_SIZE;
    println!("Found marker at {index}")
}
