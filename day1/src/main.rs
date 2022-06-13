use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate_fuel(mut amount: i32) -> i32 {
    amount /= 3;
    amount -= 2;
    amount
}

fn calculate_fuel_module(mut amount: i32) -> i32 {
    let mut total = 0;
    while amount > 0 {
        amount = calculate_fuel(amount);
        if amount > 0 {
            total += amount;
        }
    }
    total
}

fn main() {
    let mut total = 0;
    let mut module_total = 0;
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
        module_total += calculate_fuel_module(*num);
    }
    println!("The total is {}", total);
    println!("The total fuel module is {}", module_total);
}
