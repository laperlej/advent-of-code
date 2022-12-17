use ndarray::prelude::*;
use std::fmt;
use std::str::{FromStr, from_utf8};
use std::io::Write;

#[cfg(test)]
mod test {
    use super::*;
    fn test_input() -> &'static str {
"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
    }
    
    #[test]
    fn part1_test() {
        assert_eq!(part1(test_input()), 24);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(test_input()), 93);
    }
}

struct Scan {
    lines: Vec<Line>,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

impl FromStr for Scan {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<Vec<Point>> = s.lines().map(|line| line.split(" -> ").map(|point| point.parse().unwrap()).collect()).collect();
        let lines: Vec<Line> = points.iter().flat_map(|vp| vp.windows(2).map(|wp| Line{start: wp[0].clone(), end: wp[1].clone()})).collect();
        let min_x = lines.iter().map(|line| line.start.x.max(line.end.x)).min().unwrap();
        let max_x = lines.iter().map(|line| line.start.x.max(line.end.x)).max().unwrap();
        let min_y = lines.iter().map(|line| line.start.y.max(line.end.y)).min().unwrap();
        let max_y = lines.iter().map(|line| line.start.y.max(line.end.y)).max().unwrap();
        Ok(Scan{lines, min_x, max_x, min_y, max_y})
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone)]
struct Line {
    start:Point,
    end:Point,
}
 
#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');
        let x = iter.next().unwrap().parse().unwrap();
        let y = iter.next().unwrap().parse().unwrap();
        Ok(Point {x,y})
    }
}

struct Map {
    map: Array2<u8>,
    offset_x: usize,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rows = self.map.rows();
        for row in rows {
            writeln!(f, "{}", from_utf8(&row.to_vec()).unwrap())?;
        }
        Ok(())
    }
}

impl Map {
    fn new(scan: &Scan, has_floor: bool) -> Map {
        let mut map = Array2::from_elem((scan.max_y+3, 1000), b'.');
        
        if has_floor{
            for x in 0..map.dim().1 {
                let y = map.dim().0-1;
                map[[y, x]] = b'#';
            }
        }
        
        map[[0,500]] = b'+';
        for line in scan.lines.iter() {
            if line.start.x == line.end.x {
                for y in line.start.y..=line.end.y {
                    map[[y, line.start.x]] = b'#';
                }
                for y in line.end.y..=line.start.y {
                    map[[y, line.start.x]] = b'#';
                }
            } else if line.start.y == line.end.y {
                for x in line.start.x..=line.end.x {
                    map[[line.start.y, x]] = b'#';
                }
                for x in line.end.x..=line.start.x {
                    map[[line.start.y, x]] = b'#';
                }
            } else {
                panic!("Not a line");
            }
        }
        Map {map, offset_x:0}
    }

    fn add_sand(&mut self, x: usize, y: usize) -> bool {
        let mut s = Point {x,y};
        while s.y < self.map.dim().0-1 {
            if self.map[[s.y+1, s.x]] == b'.' {
                s.y += 1;
                continue;
            }
            if self.map[[s.y+1, s.x-1]] == b'.' {
                s.x -= 1;
                s.y += 1;
                continue;
            }
            if self.map[[s.y+1, s.x+1]] == b'.' {
                s.x += 1;
                s.y += 1;
                continue;
            }
            self.map[[s.y, s.x]] = b'o';
            if s.x == 500 && s.y == 0 {
                return false;
            }
            return true
        }
        false
    }

    fn fill(&mut self) {
        while self.add_sand(500-self.offset_x, 0) {}

    }

    fn count_sand(&self) -> usize {
        self.map.iter().filter(|&&c| c == b'o').count()
    }
}

fn part1(input: &str) -> usize {
    let lines = input.parse::<Scan>().unwrap();
    let mut map = Map::new(&lines, false);
    map.fill();
    map.count_sand()
}

fn part2(input: &str) -> usize {
    let lines = input.parse::<Scan>().unwrap();
    let mut map = Map::new(&lines, true);
    map.fill();
    map.count_sand()
}

fn main() {
    let input = include_str!("../../inputs/2022_14.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
