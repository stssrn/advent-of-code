use std::io::prelude::*;
use std::io::BufReader;
use log::{info, debug};

struct SubmarinePosition {
    x: i32,
    y: i32,
    aim: i32,
}

pub fn one<R: Read>(input: &mut R) {
    let reader = BufReader::new(input);
    
    let mut submarine = SubmarinePosition { x: 0, y: 0 , aim: 0};
    for line in reader.lines() {
        let line = line.unwrap();
        let iter = line.split_whitespace();
        let commands: Vec<&str> = iter.collect::<Vec<&str>>().to_owned();
        let keyword: &str = commands[0];
        let value: i32 = commands[1].parse().unwrap();
        match keyword.as_ref() {
            "forward" => {
                submarine.x += value;
                debug!("Forward: {} (X = {}, Y = {})", value, submarine.x, submarine.y);
            },
            "down" => {
                submarine.y += value;
                debug!("Down:    {} (X = {}, Y = {})", value, submarine.x, submarine.y);
            },
            "up" => {
                submarine.y -= value;
                debug!("Up:      {} (X = {}, Y = {})", value, submarine.x, submarine.y);
            },
            _ => eprintln!("Invalid command!"),
        }
    }
    info!("Horizontal position: {}", &submarine.x);
    info!("Depth position     : {}", &submarine.y);
    println!("Answer: {}", submarine.x * submarine.y);
}

pub fn two<R: Read>(input: &mut R) {
    let reader = BufReader::new(input);
    
    let mut submarine = SubmarinePosition { x: 0, y: 0 , aim: 0};
    for line in reader.lines() {
        let line = line.unwrap();
        let iter = line.split_whitespace();
        let commands: Vec<&str> = iter.collect::<Vec<&str>>().to_owned();
        let keyword: &str = commands[0];
        let value: i32 = commands[1].parse().unwrap();
        match keyword.as_ref() {
            "forward" => {
                submarine.x += value;
                submarine.y += submarine.aim * value;
                debug!("Forward: {} (X = {}, Y = {}, Aim = {})", value, submarine.x, submarine.y, submarine.aim);
            },
            "down" => {
                submarine.aim += value;
                debug!("Down:    {} (X = {}, Y = {}, Aim = {})", value, submarine.x, submarine.y, submarine.aim);
            },
            "up" => {
                submarine.aim -= value;
                debug!("Up:      {} (X = {}, Y = {}, Aim = {})", value, submarine.x, submarine.y, submarine.aim);
            },
            _ => eprintln!("Invalid command!"),
        }
    }
    info!("Horizontal position: {}", submarine.x);
    info!("Depth position     : {}", submarine.y);
    info!("Aim                : {}", submarine.aim);
    println!("Answer: {}", submarine.x * submarine.y);
}
