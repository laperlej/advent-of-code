use futures::executor::block_on;
use futures;
use rayon::prelude::*;

pub async fn solve_aoc_functional() -> (usize, usize) {
    let mut input_text: Vec<usize> = include_str!("../../inputs/2022_1.txt")
        .trim()
        .split("\n\n")
        .map(|s| s
                .lines()
                .flat_map(str::parse::<usize>)
                .sum::<usize>())
        .collect::<Vec<usize>>();
    input_text.sort_unstable_by(|a, b| b.cmp(a));
    (input_text[0], input_text[0..3].iter().sum::<usize>())
    //println!("Part1: {:?}", input_text[0]);
    //println!("Part2: {:?}", input_text[0..3].iter().sum::<usize>());
}

pub async fn solve_aoc() -> (usize, usize) {
    let input_text = include_str!("../../inputs/2022_1.txt");
    let split_input = input_text.split("\n\n");
    let mut total_calories: Vec<usize> = Vec::new();
    for part in split_input {
        let mut numbers = Vec::new();
        for number in part.lines() {
            numbers.push(number.parse::<usize>().expect("Error parsing input"));
        }
        total_calories.push(numbers.iter().sum())
    }
    total_calories.sort_unstable();
    total_calories.reverse();
    (total_calories[0], total_calories[0..3].iter().sum::<usize>())
    //println!("Part1: {:?}", total_calories[0]);
    //println!("Part2: {:?}", total_calories[0..3].into_iter().sum::<usize>());
}

pub async fn solve_aoc_parallel() -> (usize, usize) {
    let input_text = include_str!("../../inputs/2022_1.txt")
        .trim()
        .split("\n\n")
        .collect::<Vec<&str>>();
    //parse in parallel
    let mut total_calories = input_text
        .into_par_iter()
        .map(|part| part
             .lines()
             .flat_map(str::parse::<usize>)
             .sum::<usize>())
        .collect::<Vec<usize>>();
    total_calories.sort_unstable_by(|a, b| b.cmp(a));
    (total_calories[0], total_calories[0..3].iter().sum::<usize>())
    //println!("Part1: {:?}", total_calories[0]);
    //println!("Part2: {:?}", total_calories[0..3].iter().sum::<usize>());
    
}

fn main() {
    block_on(solve_aoc());
    block_on(solve_aoc_functional());
    block_on(solve_aoc_parallel());
}
