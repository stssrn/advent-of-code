use std::collections::HashMap;

fn main() {
    let mut dir_map = HashMap::<String, usize>::new();
    let mut dir = Vec::new();
    include_str!("../input.txt")
        .lines()
        .flat_map(|l| l.rsplit_once(' '))
        .for_each(|(l, r)| match (l, r) {
            ("$ cd", "..") => {
                dir.pop();
            }
            ("$ cd", name) => dir.push(name),
            ("$" | "dir", _) => (),
            (size, _) => {
                for i in 0..dir.len() {
                    let dir = String::from_iter(dir[0..=i].to_owned());
                    let size: usize = size.parse().unwrap();
                    dir_map
                        .entry(dir)
                        .and_modify(|x| *x += size)
                        .or_insert(size);
                }
            }
        });
    let sum: usize = dir_map.into_values().filter(|&x| x <= 100_000).sum();
    println!("The sum of all directories of at most 100.000 is {sum:?}")
}
