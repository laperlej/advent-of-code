use std::str::FromStr;
use std::cmp::Ordering;
use ndarray::prelude::*;
use std::collections::{HashMap, VecDeque, BinaryHeap};

#[cfg(test)]
mod test{
    use super::*;
    fn test_input() -> &'static str {
"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"}

    #[test]
    fn part1_test() {
        assert_eq!(part1(test_input()), 31);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(test_input()), 29);
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new() -> Position {
        Position {
            x: 0,
            y: 0,
        }
    }
}

impl Position {
    fn distance(&self, p: &Position) -> i64{
        ((self.x as i64 - p.x as i64).abs() + (self.y as i64 - p.y as i64).abs()) as i64
    }
}

struct HeightMap{ 
    map: Array2<u8>,
    start: Position,
    end: Position,
}

impl HeightMap {
    fn get_successors(&self, node: &Position) -> Vec<Position> {
        let mut successors = Vec::new();
        if node.x != 0 {
            let next = Position{x:node.x-1, y:node.y};
            if self.map[[next.x, next.y]] <= self.map[[node.x, node.y]] + 1 {
                successors.push(next);
            }
        }
        if node.y != 0 {
            let next = Position{x:node.x, y:node.y-1};
            if self.map[[next.x, next.y]] <= self.map[[node.x, node.y]] + 1 {
                successors.push(next);
            }
        }
        if node.x < self.map.dim().0-1 {
            let next = Position{x:node.x+1, y:node.y};
            if self.map[[next.x, next.y]] <= self.map[[node.x, node.y]] + 1 {
                successors.push(next);
            }
        }
        if node.y < self.map.dim().1-1 {
            let next = Position{x:node.x, y:node.y+1};
            if self.map[[next.x, next.y]] <= self.map[[node.x, node.y]] + 1 {
                successors.push(next);
            }
        }
        successors
    }

    fn get_successors2(&self, node: &Position) -> Vec<Position> {
        let mut successors = Vec::new();
        if node.x != 0 {
            let next = Position{x:node.x-1, y:node.y};
            if self.map[[next.x, next.y]] + 1 >= self.map[[node.x, node.y]] {
                successors.push(next);
            }
        }
        if node.y != 0 {
            let next = Position{x:node.x, y:node.y-1};
            if self.map[[next.x, next.y]] + 1 >= self.map[[node.x, node.y]] {
                successors.push(next);
            }
        }
        if node.x < self.map.dim().0-1 {
            let next = Position{x:node.x+1, y:node.y};
            if self.map[[next.x, next.y]] + 1>= self.map[[node.x, node.y]] {
                successors.push(next);
            }
        }
        if node.y < self.map.dim().1-1 {
            let next = Position{x:node.x, y:node.y+1};
            if self.map[[next.x, next.y]] +1 >= self.map[[node.x, node.y]] {
                successors.push(next);
            }
        }
        successors
    }

    fn search(&self, start: &Position, end: &Position) -> i64 {
        let mut visited = HashMap::new();
        let mut heap: BinaryHeap<Node> = BinaryHeap::new();
        heap.push(Node{position:start.clone(), depth: 0, min_distance: start.distance(end)});

        while let Some(node) = heap.pop() {
            let successors = self.get_successors(&node.position);
            for next_node in successors {
                if next_node == *end {
                    return node.depth+1;
                }
                if !visited.contains_key(&next_node) {
                    heap.push(Node{position:next_node.clone(), depth:node.depth+1 , min_distance: node.depth+1+next_node.distance(end)});
                }
            }
            visited.insert(node.position, true);
        }
        -1
    }

    fn search2(&self, start: &Position) -> i64 {
        //minimum steps to reach map[x][y] == 0
        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();
        queue.push_back((start.clone(), 0));

        while !queue.is_empty() {
            let (node, depth) = queue.pop_front().unwrap();
            let successors = self.get_successors2(&node);
            for next_node in successors {
                if self.map[[next_node.x, next_node.y]] == 0 {
                    return depth+1;
                }
                if !visited.contains_key(&next_node) {
                    visited.insert(next_node.clone(), true);
                    queue.push_back((next_node.clone(), depth+1));
                }
            }
        }
        -1
    }

    fn nb_steps(&self) -> i64 {
        self.search(&self.start, &self.end)
    }

    fn nb_steps2(&self) -> i64 {
        self.search2(&self.end)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    position: Position,
    depth: i64,
    min_distance: i64,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.min_distance.cmp(&self.min_distance)
    }
}

impl FromStr for HeightMap {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = Position::new();
        let mut end = Position::new();
        let lines: Vec<Vec<u8>> = s.lines().map(|l| l.as_bytes().to_vec()).collect();
        let mut map = Array2::zeros((lines.len(), lines[0].len()));
        for i in 0..lines.len() {
            for j in 0..lines[i].len() {
                match lines[i][j] as char {
                    'S' => {
                        start.x = i;
                        start.y = j;
                        map[[i,j]] = 0;
                    }
                    'E' => {
                        end.x = i;
                        end.y = j;
                        map[[i,j]] = 25;
                    }
                    c => {
                        map[[i,j]] = (c as u8) - 97
                    }
                }
            }

        }
        Ok(HeightMap {
            map,
            start,
            end,
        })
    }
}

fn part1(input: &str) -> i64 {
    let map: HeightMap = input.parse().unwrap();
    map.nb_steps()
}

fn part2(input: &str) -> i64 {
    let map: HeightMap = input.parse().unwrap();
    map.nb_steps2()
}

fn main() {
    let input = include_str!("../../inputs/2022_12.txt");
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
