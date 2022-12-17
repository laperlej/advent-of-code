use futures::executor::block_on;
use futures;

//define type of game where each player get a char

pub async fn solve_aoc() {
    let input_text = include_str!("../../inputs/2022_2.txt");

    let mut score = 0;
    let games = input_text.lines();
    for game in games {
        match game {
            "A X" => score += 3 + 0,
            "A Y" => score += 1 + 3,
            "A Z" => score += 2 + 6,
            "B X" => score += 1 + 0,
            "B Y" => score += 2 + 3,
            "B Z" => score += 3 + 6,
            "C X" => score += 2 + 0,
            "C Y" => score += 3 + 3,
            "C Z" => score += 1 + 6,
            _ => (),
        }
    }
    println!("score: {}", score);
}


fn main() {
    block_on(solve_aoc());
}
