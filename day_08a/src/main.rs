fn main() {
    let input = include_bytes!("../input.txt");
    let grid = input
        .split(|&x| x == b'\n')
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();
    let size = grid.len();

    let amount_visible: usize = grid
        .iter()
        .enumerate()
        .skip(1)
        .take(size - 2)
        .map(|(ri, r)| {
            r.iter()
                .enumerate()
                .skip(1)
                .take(size - 2)
                .filter(|&(ci, &t)| {
                    grid[..ri].iter().all(|x| x[ci] < t)
                        || grid[ri][..ci].iter().all(|&x| x < t)
                        || grid[ri][ci + 1..].iter().all(|&x| x < t)
                        || grid[ri + 1..].iter().all(|x| x[ci] < t)
                })
                .count()
        })
        .sum::<usize>()
        + size * 4
        - 4;
    println!("{amount_visible:?} trees are visible from the grid");
}
