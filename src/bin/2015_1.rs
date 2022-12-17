use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};


fn main() {
    let input_file = env::args().nth(1).unwrap();
    //open file
    let file = File::open(input_file).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let mut count = 0;
    for (i, char) in line.chars().enumerate() {
        match char {
            '(' => count += 1,
            ')' => count -= 1,
            _ => {},
        }
        if count == -1 {
            println!("Entered basement at position {}", i + 1);
            return;
        }
    }
    println!("{}", count);
}
