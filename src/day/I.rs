use std::io::prelude::*;
use std::io::BufReader;
use log::info;

pub fn one<R: Read>(input: &mut R) {
    let reader = BufReader::new(input);
    
    let mut previous_number = None;
    let mut increased_count = 0;
    for line in reader.lines() {
        let current_number: u32 = line.unwrap().parse().expect("Couldn't parse file");
        if let Some(n) = previous_number {
            if current_number > n {
                info!("{} (increased)", current_number);
                increased_count += 1;
            } else if current_number < n {
                info!("{} (decreased)", current_number)
            } else {
                info!("{} (no change)", current_number)
            }
        } else {
            info!("{} (N/A - no previous measurement)", current_number)
        }
        previous_number = Some(current_number);
    }
    println!("Answer: {}", increased_count);
}

pub fn two<R: Read>(input: &mut R) {
    let reader = BufReader::new(input);

    let all_numbers: Vec<u32> = reader.lines()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .iter()
        .map(|string| string.parse::<u32>().expect("Couldn't parse file"))
        .collect();
    let all_numbers_length = all_numbers.len();
    let mut window_start: usize = 0;
    let mut window_end: usize = 3;
    let mut previous_window_sum = None;
    let mut increased_count = 0;
    while window_end <= all_numbers_length as usize {
        let current_window: usize = all_numbers[window_start..window_end].to_vec().into_iter().sum::<u32>() as usize;
        if let Some(n) = previous_window_sum {
           if current_window > n {
               info!("{}: {} (increased)", window_start, current_window);
               increased_count += 1;
           } else if current_window < n {
               info!("{}: {} (decreased)", window_start, current_window);
           } else {
               info!("{}: {} (no change)", window_start, current_window);
           }
        } else {
            info!("{}: {} (N/A - no previous sum)", window_start, current_window);
        }
        previous_window_sum = Some(current_window);
        window_start += 1;
        window_end += 1;
    }
    println!("Answer: {}", increased_count);
}
