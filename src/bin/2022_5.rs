use std::str::FromStr;
use std::str::from_utf8;
use std::fs::read_to_string;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Stacks {
    stacks: Vec<Vec<u8>>
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    number: usize,
    from: usize,
    to: usize,
}

impl Stacks {
    fn move_crates1(&mut self, m: &Move) {
        for _ in 0..m.number {
            let crate_name = self.stacks[m.from-1].pop().unwrap();
            self.stacks[m.to-1].push(crate_name);
        }
    }
    fn move_crates2(&mut self, m: &Move) {
        let split_index = self.stacks[m.from-1].len() - m.number;
        let mut crates = self.stacks[m.from-1].split_off(split_index);
        self.stacks[m.to-1].append(&mut crates);
    }
}

impl FromStr for Stacks {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //                [M]     [W] [M]
        //            [L] [Q] [S] [C] [R]
        //            [Q] [F] [F] [T] [N] [S]
        //    [N]     [V] [V] [H] [L] [J] [D]
        //    [D] [D] [W] [P] [G] [R] [D] [F]
        //[T] [T] [M] [G] [G] [Q] [N] [W] [L]
        //[Z] [H] [F] [J] [D] [Z] [S] [H] [Q]
        //[B] [V] [B] [T] [W] [V] [Z] [Z] [M]
        // 1   2   3   4   5   6   7   8   9
        // vec of size 9
        let mut stacks: Vec<Vec<u8>> = vec![Vec::new(); 9];
        //read lines in reverse and drop the first line
        for line in s.lines().rev().skip(1) {
            let chars = line.as_bytes();
            let crates = chars.chunks(4);
            for (i, crate_name) in crates.enumerate() {
                if crate_name[1] != b' ' {
                    stacks[i].push(crate_name[1]);
                }
            }
        }
        Ok(Stacks { stacks })
    }
}


impl FromStr for Move {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        //move 1 from 2 to 3
        let number = parts[1].parse::<usize>().unwrap();
        let from = parts[3].parse::<usize>().unwrap();
        let to = parts[5].parse::<usize>().unwrap();
        Ok(Move { number, from, to })
    }
}

fn part1() {
    let input_text: String = read_to_string("inputs/2022_5.txt")
        .expect("Could not read file");  
    let parts = input_text
        .split("\n\n")
        .collect::<Vec<_>>();
    let mut stacks: Stacks = parts[0]
        .parse()
        .expect("Could not parse stacks");
    let mut moves = parts[1]
        .lines()
        .map(|line| line
             .parse());
    loop {
        match moves.next() {
            Some(Ok(m)) => stacks.move_crates1(&m),
            Some(Err(_)) => panic!("Error parsing move"),
            None => break,
        }
    }
    let last_crates: Vec<u8> = stacks.stacks
        .iter()
        .map(|stack| stack[stack.len()-1])
        .collect();
    let result = from_utf8(&last_crates).expect("Could not convert to string");
    println!("part1: {:?}", result);
}

fn part2() {
    let file_text = &read_to_string("inputs/2022_5.txt");
    let file_text: &str = match file_text {
        Ok(text) => text,
        Err(_) => return,
    };
    let parts = file_text.split("\n\n").collect::<Vec<&str>>();
    let stacks: Result<Stacks, _>= parts[0].parse();
    let mut stacks: Stacks = match stacks {
        Ok(s) => s,
        Err(_) => return,
    };
    let moves: Vec<Result<Move, _>> = parts[1].lines().map(|line| line.parse()).collect();
    let moves: Vec<Move> = moves.into_iter().filter_map(|m| m.ok()).collect();
    for m in moves {
        stacks.move_crates2(&m);
    }
    let last_crates: Vec<u8> = stacks.stacks.iter().map(|stack| stack[stack.len()-1]).collect();
    println!("part2: {:?}", from_utf8(&last_crates).unwrap());
}

fn main () {
    part1();
    part2();
}
