use std::io::prelude::*;
use std::io::BufReader;
use std::collections::BTreeMap;
use log::{debug, info, error};

#[derive(Clone, Copy)]
struct BitCount {
    zero_count: i32,
    one_count: i32,
}

fn report_generator(input: &Vec<String>) -> BTreeMap<u32, BitCount> {
    let mut diagnostic_report: BTreeMap<u32, BitCount> = BTreeMap::new();
    let binary_length = &input[0].chars().count();
    for position in 0..*binary_length as u32 {
        diagnostic_report.insert(position, BitCount{ zero_count: 0, one_count: 0 });
    };

    for number in input.iter() {
        let mut index = 0;
        for bit in number.chars() {
            let mut bit_count = *diagnostic_report.get_mut(&index).unwrap();
            match bit.to_digit(2).unwrap() {
                0 => bit_count.zero_count += 1,
                1 => bit_count.one_count += 1,
                _ => error!("Invalid bit: {}", &bit),
            }
            diagnostic_report.insert(index, bit_count);
            index += 1
        }
    }
    diagnostic_report
}

fn gamma_rate_calculator(report: &BTreeMap<u32, BitCount>, length: u32) -> usize {
    let mut gamma_rate = 0b0000_0000_0000;
    for (position, bit_count) in report.iter() {
        if bit_count.one_count > bit_count.zero_count {
            info!("Bit {:2}: 1 ({} zeros, {} ones)", position + 1, bit_count.zero_count, bit_count.one_count);
            gamma_rate ^= 1 << (length - position) >> 1;
        } else {
            info!("Bit {:2}: 0 ({} zeros, {} ones)", position + 1, bit_count.zero_count, bit_count.one_count);
        }
    }
    gamma_rate
}

fn epsilon_rate_calculator(report: &BTreeMap<u32, BitCount>, length: u32) -> usize {
    let gamma_rate = gamma_rate_calculator(&report, length);
    usize::pow(2, length) - 1 ^ gamma_rate // Bit flip
}

pub fn one<R: Read>(input: &mut R) {
    let reader = BufReader::new(input);
    
    let numbers_vec: Vec<String> = reader.lines()
        .map(|l| l.unwrap())
        .collect();
    let diagnostic_report = report_generator(&numbers_vec);
    let binary_length: u32 = numbers_vec[0].chars().count() as u32;
    let gamma_rate = gamma_rate_calculator(&diagnostic_report, binary_length);
    let epsilon_rate = epsilon_rate_calculator(&diagnostic_report, binary_length);
    info!("Gamma Rate  : {} ({:0width$b})", gamma_rate, gamma_rate, width = binary_length as usize);
    info!("Epsilon Rate: {} ({:0width$b})", epsilon_rate, epsilon_rate, width = binary_length as usize);
    println!("Answer: {}", gamma_rate * epsilon_rate);
}

fn common_finder(list: &Vec<String>, pos: usize) -> char {
    let mut zeros = 0;
    let mut ones = 0;
    for number in list.iter() {
        match number.chars().nth(pos).unwrap() {
            '0' => zeros += 1,
            '1' => ones += 1,
            _ => error!("Invalid character")
        }
    }
    if zeros > ones {
        '0'
    } else {
        '1'
    }
}

fn uncommon_finder(list: &Vec<String>, pos: usize) -> char {
    let mut zeros = 0;
    let mut ones = 0;
    for number in list.iter() {
        match number.chars().nth(pos).unwrap() {
            '0' => zeros += 1,
            '1' => ones += 1,
            _ => error!("Invalid character")
        }
    }
    if zeros > ones {
        '1'
    } else {
        '0'
    }
}

pub fn two<R: Read>(input: &mut R) {
    let reader = BufReader::new(input);
    let og_numbers_vec: Vec<String> = reader.lines()
        .map(|l| l.unwrap())
        .collect();
    
    // Calculating Oxygen Generator Rating
    let mut numbers_vec = og_numbers_vec.clone();
    let mut oxygen_generator_rating = None;
    let mut checked_numbers: Vec<String> = Vec::new();
    let mut position = 0;
    while let None = oxygen_generator_rating {
        let common_bit = common_finder(&numbers_vec, position);
        debug!("Common bit: {}, (numbers left: {})", common_bit, numbers_vec.len());
        for number in numbers_vec.iter() {
            if number.chars().nth(position).unwrap() == common_bit {
                checked_numbers.push(number.to_string());
            }
        }
        if checked_numbers.len() == 1 {
            let winner = checked_numbers[0].clone(); // I'm a lazy POS
            debug!("Found number!: {} (numbers left: {})", winner, numbers_vec.len());
            oxygen_generator_rating = Some(usize::from_str_radix(&winner, 2).unwrap());
        }
        position += 1;
        numbers_vec = checked_numbers.clone(); // I'm a lazy POS
        checked_numbers.clear();
    }
    info!("Oxygen Generator Rating: {}", oxygen_generator_rating.unwrap());

    // CO2 Scrubber Rating
    let mut co2_scrubber_rating = None;
    let mut numbers_vec = og_numbers_vec.clone();
    let mut checked_numbers: Vec<String> = Vec::new();
    let mut position = 0;
    while let None = co2_scrubber_rating {
        let common_bit = uncommon_finder(&numbers_vec, position);
        debug!("Common bit: {}, (numbers left: {})", common_bit, numbers_vec.len());
        for number in numbers_vec.iter() {
            if number.chars().nth(position).unwrap() == common_bit {
                checked_numbers.push(number.to_string());
            }
        }
        if checked_numbers.len() == 1 {
            let winner = checked_numbers[0].clone(); // I'm a lazy POS
            debug!("Found number!: {} (number left: {})", winner, numbers_vec.len());
            co2_scrubber_rating = Some(usize::from_str_radix(&winner, 2).unwrap());
        }
        position += 1;
        numbers_vec = checked_numbers.clone(); // I'm a lazy POS
        checked_numbers.clear();
    }
    info!("CO2 Scrubber Rating: {}", co2_scrubber_rating.unwrap());
    println!("Answer: {}", oxygen_generator_rating.unwrap() * co2_scrubber_rating.unwrap());
}
