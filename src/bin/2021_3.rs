
fn most_common_bit(numbers: &Vec<u32>, position: usize) -> u32 {
    let mut ones_count = 0;
    let mut zeros_count = 0;
    for number in numbers {
        if number & (1 << position) == 0 {
            zeros_count += 1;
        } else {
            ones_count += 1;
        }
    }
    if ones_count >= zeros_count {
        return 1
    } else {
        return 0
    }
}

fn main () {
    let input = include_str!("../../inputs/2021_3.txt");
    //lines are strings of 12 1s and 0s, parse to u32
    let numbers = input.lines().map(|line| u32::from_str_radix(line, 2).unwrap()).collect::<Vec<u32>>();
    let gamma_rate: u32 = (0..12).map(|position| most_common_bit(&numbers, position) << position).sum();
    let epsilon_rate: u32 = (0..12).map(|position| (most_common_bit(&numbers, position) ^ 1) << position).sum();
    println!("Part1: {}", gamma_rate * epsilon_rate);
}
