use std::env;
use std::str::FromStr;

#[derive(Debug)]
struct Move {
    direction: String,
    distance: i64,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        let direction = parts[0].parse().unwrap();
        let distance = parts[1].parse().unwrap();
        Ok(Move { direction, distance })
    }
}

fn parse_file(filename: String) -> Vec<Move> {
    let input = std::fs::read_to_string(filename).expect("Error reading input file");
    //read line by line, check for empty lines and store in a vector if not empty
    let numbers: Vec<Move> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().expect("Error parsing input"))
        .collect();
    numbers
}

fn do_moves(moves: &Vec<Move>) -> (i64, i64) {
    let mut depth = 0;
    let mut h_position = 0;
    for m in moves {
        match m.direction.as_str() {
            "forward" => h_position += m.distance,
            "up" => depth -= m.distance,
            "down" => depth += m.distance,
            _ => (),
        }
    }
    (depth, h_position)
}

fn do_moves_with_aim(moves: &Vec<Move>) -> (i64, i64) {
    let mut depth = 0;
    let mut h_position = 0;
    let mut aim = 0;
    for m in moves {
        match m.direction.as_str() {
            "forward" => {
                h_position += m.distance; 
                depth += aim * m.distance;
            },
            "up" => aim -= m.distance,
            "down" => aim += m.distance,
            _ => (),
        }
    }
    (depth, h_position)
}

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing input file");
    let moves = parse_file(input_file_path);
    let (depth, h_position) = do_moves(&moves);
    println!("Result {}", depth * h_position);
    let (depth, h_position) = do_moves_with_aim(&moves);
    println!("Result with aim {}", depth * h_position);
}
