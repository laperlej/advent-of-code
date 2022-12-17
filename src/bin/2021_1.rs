use std::env;

fn number_of_increases(nums: &Vec<u32>) -> u32 {
    //if 0 or 1 numbers, no increases
    if nums.len() < 2 {
        return 0;
    }
    let mut i  = 1;
    let mut count = 0;
    while i < nums.len() {
        if nums[i] > nums[i-1] {
            count += 1
        }
        i += 1
    }
    return count
}

fn sliding_window_increases(nums: &Vec<u32>) -> u32 {
    if nums.len() < 4 {
        return 0;
    }
    let mut i = 3;
    let mut count = 0;
    while i < nums.len() {
        if nums[i-2..i+1].iter().sum::<u32>() > nums[i-3..i].iter().sum::<u32>() {
            count += 1
        }
        i += 1
    }
    return count
}

fn parse_file(filename: String) -> Vec<u32> {
    let input = std::fs::read_to_string(filename).expect("Error reading input file");
    //read line by line, check for empty lines and store in a vector if not empty
    let numbers: Vec<u32> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().expect("Error parsing input"))
        .collect();
    numbers
}

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing input file");
    let numbers = parse_file(input_file_path);
    let n: u32 =  number_of_increases(&numbers);
    println!("Number of increases: {:?}", n);
    let n: u32 =  sliding_window_increases(&numbers);
    println!("Number of increases in sliding window of size 3: {:?}", n);
}
