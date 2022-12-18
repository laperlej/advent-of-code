use std::str::FromStr;
use std::fmt::Display;
use std::collections::{ VecDeque, HashMap };
use std::io::Write;
use ndarray::prelude::*;
use itertools::Itertools;


#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() ->  &'static str {
        ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(test_input()), 3068);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(test_input()), 1514285714288);
    }
   
}

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Block {
    position: Position,
    shape: Vec<(usize, usize)>,
}

impl Block {
    fn new(position: Position, shape: usize) -> Self {
        let shapes = vec![
        vec![(0,0), (1,0), (2,0), (3,0)],
        vec![(0,1), (1,0), (1,1), (1,2), (2,1)],
        vec![(0,0), (1,0), (2,0), (2,1), (2,2)],
        vec![(0,0), (0,1), (0,2), (0,3)],
        vec![(0,0), (1,0), (0,1), (1,1)]];
        let i = shape % shapes.len();
        Self { position, shape: shapes[i].clone() }
    }
}

#[derive(Debug, Clone)]
struct Chamber {
    map: Array2<u8>,
    block: Block,
    block_no: usize,
    max_height: usize,
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        let mut map = self.map.clone();
        for (x, y) in &self.block.shape {
            map[[self.block.position.x + x, self.block.position.y + y]] = b'@';
        }
        let start = 20.max(self.max_height) - 20;
        let end = 20.max(self.max_height) + 10;
        for j in (start..end).rev() {
            for i in 0..map.shape()[0] {
                s.push(map[[i,j]] as char);
            }
            s.push('\n');
        }
        write!(f, "{s}")
    }
}

impl Chamber {
    fn new() -> Self {
        //map
        let height = 200000000;
        let width = 9;
        let mut map = Array2::from_elem((width,height), b'.');
        for i in 1..width-1 {
            map[[i, 0]] = b'-';
        }
        for i in 1..height {
            map[[0,i]] = b'|';
            map[[width-1,i]] = b'|';
        }
        map[[0,0]] = b'+';
        map[[width-1,0]] = b'+';
        //Block
        let max_height = 0;
        let position = Position { x: 3, y: max_height + 4 };
        let block_no = 0;
        let block = Block::new(position, block_no);

        Chamber { map, block, block_no, max_height }
    }

    fn next_block(&mut self) {
        self.block_no += 1;
        self.block = Block::new(Position { x: 3, y: self.max_height+4 }, self.block_no);
    }

    fn will_colide(&self, x: i64, y: i64) -> bool {
        for (i, j) in &self.block.shape {
            let new_x = self.block.position.x as i64 + x + *i as i64;
            let new_y = self.block.position.y as i64 + y + *j as i64;
            if self.map[[new_x as usize, new_y as usize]] != b'.'{
                return true
            }
        }
        false
    }


    fn next_move(&mut self, direction: char) {
        let (x, y): (i64, i64) = match direction {
            '>' => (1, 0),
            '<' => (-1, 0),
            _ => (0, 0),
        };

        if !self.will_colide(x, y) {
            self.block.position.x = (self.block.position.x as i64 + x) as usize;
            self.block.position.y = (self.block.position.y as i64 + y) as usize;
        }
    }
    
    fn fall_down(&mut self) {
        if !self.will_colide(0, -1) {
            self.block.position.y -= 1;
        } else {
            for (i, j) in &self.block.shape {
                self.map[[self.block.position.x + i, self.block.position.y + j]] = b'#';
                self.max_height = std::cmp::max(self.max_height, self.block.position.y + j);
            }
            self.next_block();
        }
    }
}




fn part1(input: &str) -> i64 {
    let mut chamber = Chamber::new();
    let moves = input.trim().chars().collect::<VecDeque<_>>();
    let mut i = 0;
    let mut previous = 0;
    while chamber.block_no < 2022 {
        let move_no = i % moves.len();
        chamber.next_move(moves[move_no]);
        chamber.fall_down();
        i += 1;
    }
    chamber.max_height as i64
}


fn part2(input: &str) -> i64 {
    let mut chamber = Chamber::new();
    let moves = input.trim().chars().collect::<VecDeque<_>>();
    let mut i = 0;
    let mut previous_height = 0;
    let mut previous_no = 0;
    let mut max_height_adj = 0;
    while chamber.block_no < 1000000000000 {
        if i % moves.len() == 0 {
            if chamber.max_height-previous_height == 2649 {
                while chamber.block_no < 1000000000000-1705 { 
                    chamber.block_no += 1705;
                    max_height_adj += 2649;
                }
            }
            previous_height = chamber.max_height;
            previous_no = chamber.block_no;
        }

        let move_no = i % moves.len();
        chamber.next_move(moves[move_no]);
        chamber.fall_down();
        i += 1;
    }
    chamber.max_height as i64 + max_height_adj
}

fn main() {
    let input = include_str!("../../inputs/2022_17.txt");
    let time = std::time::Instant::now();
    println!("Part 1: {}", part1(input));
    println!("Time: {}ms", time.elapsed().as_millis());
    let time = std::time::Instant::now();
    println!("Part 2: {}", part2(input));
    println!("Time: {}ms", time.elapsed().as_millis());
}

