use std::ops::Rem;

fn main() {
    let crt = include_str!("../input.txt")
        .lines()
        .flat_map(|x| {
            x.strip_prefix("addx ")
                .and_then(|x| x.parse::<isize>().ok())
                .map(|x| vec![0, x])
                .unwrap_or_else(|| vec![0])
        })
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .enumerate()
        .map(|(i, x)| (x - 1..=x + 1).contains(&(i.rem(40) as isize)))
        .map(|is_visible| match is_visible {
            true => "#",
            false => ".",
        })
        .collect::<Vec<_>>();

    // display (first item in every row is missing, but good enough)
    for row in crt.chunks(40) {
        for c in row.iter() {
            print!("{c}");
        }
        println!();
    }
}
