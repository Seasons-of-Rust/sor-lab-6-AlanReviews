use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate_fuel(mut amount: i32) -> i32 {
    amount /= 3;
    amount -= 2;
    println!("The fuel amount is {}", amount);
    amount
}

fn main() {
    let mut total = 0;
    let numbers: Vec<i32> = BufReader::new(File::open("input.txt").expect("file not found"))
        .lines() // Go through each line
        .map(|line| {
            line.expect("could not parse line") // Unwrap the result of the line
                .parse() // Try to parse it to what we expect (i32 from the annotation)
                .expect("could not parse number") // Unwrap the result of the parse
        })
        .collect();
    let number_iterator = numbers.iter();
    for num in number_iterator {
        total += calculate_fuel(*num);
    }
    println!("The total is {}", total)
}
