use std::fs::File;
use std::io::{BufReader, BufRead};
use ndarray::Array2;

fn next_house(char: char, houses: &mut Array2<i32>, x: &mut usize, y: &mut usize) {
    match char {
        '^' => *y += 1,
        'v' => *y -= 1,
        '>' => *x += 1,
        '<' => *x -= 1,
        _ => (),
    }
    let x = *x as usize;
    let y = *y as usize;
    houses[[x, y]] += 1;
}

fn part1(line: &String) -> i64 {
    let mut houses: Array2<i32> = Array2::zeros((1000, 1000));
    let mut x = 500;
    let mut y = 500;
    houses[[x, y]] = 1;
    for char in line.chars() {
        next_house(char, &mut houses, &mut x, &mut y);
    }
    houses.iter().filter(|&&x| x > 0).count() as i64
}

fn part2(line: &String) -> i64 {
    let mut houses: Array2<i32> = Array2::zeros((1000, 1000));
    let mut x1 = 500;
    let mut y1 = 500;
    let mut x2 = 500;
    let mut y2 = 500;
    houses[[x1, y1]] = 2;
    for (i, char) in line.chars().enumerate() {
        if i % 2 == 0 {
            next_house(char, &mut houses, &mut x1, &mut y1);
        } else {
            next_house(char, &mut houses, &mut x2, &mut y2);
        }
    }
    houses.iter().filter(|&&x| x > 0).count() as i64
}

fn main() {
    //open file
    let file = File::open("../../inputs/2015_3.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    println!("Part1: {}", part1(&line));
    println!("Part2: {}", part2(&line));
}
