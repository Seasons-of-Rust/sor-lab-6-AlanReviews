use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn opcode_one(nums: &mut [i32], pointer: usize) {
    let pos1: usize = nums[pointer + 1] as usize;
    let pos2: usize = nums[pointer + 2] as usize;
    let result_pos: usize = nums[pointer + 3] as usize;
    let total: i32 = nums[pos1] + nums[pos2];
    nums[result_pos] = total;
}

fn opcode_two(nums: &mut [i32], pointer: usize) {
    let pos1: usize = nums[pointer + 1] as usize;
    let pos2: usize = nums[pointer + 2] as usize;
    let result_pos: usize = nums[pointer + 3] as usize;
    let total: i32 = nums[pos1] * nums[pos2];
    nums[result_pos] = total;
}

fn main() {
    let mut numbers: Vec<i32> = BufReader::new(File::open("input.txt").expect("file not found"))
        .lines() // Go through each line
        .next() // Only take the first line
        .unwrap() // Unwrap the option of whether there was a next line
        .unwrap() // Unwrap the string result of the lines
        .split(',') // Split by commas
        .map(|number| {
            number
                .parse() // Parse the number
                .expect("could not parse number") // Panic with a message if it fails
        })
        .collect();
    let mut pointer = 0;
    while numbers[pointer] != 99 {
        match numbers[pointer] {
            1 => opcode_one(&mut numbers, pointer),
            2 => opcode_two(&mut numbers, pointer),
            _ => unreachable!(),
        }
        pointer += 4;
    }
    println!("{:?}", numbers);
}
