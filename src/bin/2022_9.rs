use std::fs::read_to_string;
use std::str::FromStr;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;
    fn test_input() -> &'static str {
    "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"}
    fn test_input2() -> &'static str {
        "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
    }

    #[test]
    fn part1_test() {
        let input = test_input();
        assert_eq!(part1(input), 13);
    }
    #[test]
    fn part2_test() {
        let input = test_input2();
        assert_eq!(part2(input), 36);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn is_adjacent(&self, pos: &Position) -> bool {
        (self.x == pos.x && (self.y - pos.y).abs() == 1) ||
        (self.y == pos.y && (self.x - pos.x).abs() == 1) ||
        (self.x - pos.x).abs() == 1 && (self.y - pos.y).abs() == 1
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Rope {
    knots: Vec<Position>,
    visited: Vec<Position>,
}

impl Rope {
    fn new(number_of_knots: usize) -> Self {
        Rope {
            knots: vec![Position { x: 0, y: 0 }; number_of_knots],
            visited: vec![],
        }
    }

    fn move_head(&mut self, rope_move: &Move) {
        for _ in 0..rope_move.distance {
            match rope_move.direction {
                Direction::Up => self.knots[0].y += 1,
                Direction::Down => self.knots[0].y -= 1,
                Direction::Left => self.knots[0].x -= 1,
                Direction::Right => self.knots[0].x += 1,
            }

            //iterate over knots in a 2 element window
            let length = self.knots.len().clone();
            for i in 0..length - 1 {
                if !self.knots[i].is_adjacent(&self.knots[i+1]) {
                        if self.knots[i].y > self.knots[i+1].y {
                            self.knots[i+1].y += 1;
                        } else if self.knots[i].y < self.knots[i+1].y {
                            self.knots[i+1].y -= 1;
                        }
                        if self.knots[i].x > self.knots[i+1].x {
                            self.knots[i+1].x += 1;
                        } else if self.knots[i].x < self.knots[i+1].x {
                            self.knots[i+1].x -= 1;
                        }
                }
            }
            self.visited.push(self.knots.last().unwrap().clone());
        }
     }


    fn unique_visited(&self) -> usize {
        let mut set = HashSet::new();
        for pos in &self.visited {
            set.insert(pos);
        }
        set.len()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
struct Move {
    direction: Direction,
    distance: i32,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dist) = s.split_at(1);
        let dist = dist.trim().parse::<i32>().unwrap();
        let direction = match dir {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(format!("Unknown direction: {}", dir)),
        };
        Ok(Move { direction, distance: dist })
    }
}

fn part1(input: &str) -> usize {
   let moves = input.lines().map(|line| line.parse::<Move>().unwrap()).collect::<Vec<_>>();
   let mut rope = Rope::new(2);
   for rope_move in moves {
       rope.move_head(&rope_move);
   }
   rope.unique_visited()
  
}
fn part2(input: &str) -> usize {
   let moves = input.lines().map(|line| line.parse::<Move>().unwrap()).collect::<Vec<_>>();
   let mut rope = Rope::new(10);
   for rope_move in moves {
       rope.move_head(&rope_move);
   }
   rope.unique_visited()
}

fn main() {
    let input = read_to_string("inputs/2022_9.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

