use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    loop {
        println!("Rust exercises on collections");
        println!("1. Find median, mode of a list of integers");
        println!("0. Exit");

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
        match option.trim() {
            "0" => {
                println!("Bye!");
                break;
            }
            "1" => median_mode(),
            _ => {
                println!("Invalid option selected. Please try again.");
                continue;
            }
        }
    }
}

fn median_mode() {
    println!("Enter list of integers separated by space");
    let reader = io::stdin();
    let mut numbers: Vec<i32> = reader
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    numbers.sort();
    let mut max_count = 0;
    let mut mode: Option<i32> = None;
    let mut map = HashMap::new();
    for num in &numbers {
        let count = map.entry(num).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count= *count;
            mode = Some(*num);
        }
    }
    if numbers.len() == 0 {
        println!("Median: N/A");
    } else {
        let mid = if numbers.len() % 2 == 0 {
            numbers.len() / 2
        } else {
            (numbers.len() - 1) / 2
        };
        println!("Median: {}", numbers[mid]);
    }

    if let Some(val) = mode {
        println!("Mode: {}", val);
    } else {
        println!("Mode: N/A");
    }
}
