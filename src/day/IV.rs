use std::io::prelude::*;
use std::io::BufReader;
use log::{debug, info, error};

const BOARD_SIZE: usize = 5;

enum Spot {
    Marked(u32),
    Unmarked(u32),
}

struct BingoBoard {
    board: Vec<Vec<Spot>>,
}

pub fn one<R: Read>(input: &mut R) {
    let reader = BufReader::new(input);
    let mut iter = reader
        .lines()
        .map(|l| l.unwrap());
    
    let number_pool: Vec<u8> = iter
        .take(1)
        .collect::<String>()
        .split(",")
        .map(|n| n.parse::<u8>().unwrap())
        .collect();
    debug!("{:?}", number_pool);

    let mut boards: Vec<BingoBoard> = reader
        .collect::<String>()
        .split("\n\n")
        .fold(vec![vec![Spot]], |acc, n|
              n
        

}
