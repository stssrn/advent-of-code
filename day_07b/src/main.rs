use std::collections::HashMap;

const DISK_SIZE: usize = 70_000_000;
const UPDATE_SIZE: usize = 30_000_000;

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

    let space_used = dir_map.get("/").unwrap();
    let space_left = DISK_SIZE - space_used;
    let space_needed = UPDATE_SIZE - space_left;

    let mut sizes: Vec<usize> = dir_map.into_values().collect();
    sizes.sort();
    let smallest = sizes.into_iter().find(|&x| x >= space_needed).unwrap();
    println!("Delete the directory of size {smallest:?}")
}
