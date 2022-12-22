use std::str::FromStr;
use std::collections::{ VecDeque, HashMap, BinaryHeap };
use std::io::Write;
use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::Ordering;
use num_complex::Complex64;


#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() ->  &'static str {
"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(test_input()), 152);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(test_input()), 301);
    }
}

struct Elf {
    name: String,
    waiting_on: Vec<String>,
    value: Option<Complex64>,
    operation: Option<Box<dyn Fn(Complex64, Complex64) -> Complex64>>,
}

impl FromStr for Elf {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(' ');
        let name = parts.next().unwrap().to_string().replace(':', "");
        let value_or_elf1 = parts.next().unwrap();
        let value_result = value_or_elf1.parse::<Complex64>();
        let value = match value_result  {
            Ok(n) => Some(n),
            Err(_) => None
        };
        if value.is_some() {
            return Ok(Elf { name, waiting_on: Vec::new(), value, operation: None });
        }
        let elf1 = value_or_elf1.to_string();
        let operation: Option<Box<dyn Fn(Complex64, Complex64) -> Complex64>> = match parts.next() {
            Some("+") => Some(Box::new(|x,y| x + y)),
            Some("-") => Some(Box::new(|x,y| x - y)),
            Some("*") => Some(Box::new(|x,y| x * y)),
            Some("/") => Some(Box::new(|x,y| x / y)),
            _ => None
        };
        let elf2 = parts.next().unwrap().to_string();
        Ok(Elf { name, waiting_on: vec![elf1, elf2], value: None, operation })
    }
}

struct Elves {
    elves: Vec<Elf>,
    elf_map: HashMap<String, usize>,
}

impl FromStr for Elves {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elves: Vec<Elf> = s.lines().map(|l| l.parse::<Elf>().unwrap()).collect();
        let elf_map = elves.iter().enumerate().map(|(i, e)| (e.name.clone(), i)).collect();
        Ok(Elves { elves, elf_map })
    }
}

impl Elves {
    fn solve_for(&mut self, elf_name: &str) -> Result<Complex64, String> {
        while self.elves[self.elf_map[elf_name]].value.is_none() {
            for i in 0..self.elves.len() {
                if self.elves[i].value.is_some() {
                    continue;
                }
                let elf1_value = self.elves[self.elf_map[&self.elves[i].waiting_on[0]]].value;
                let elf2_value = self.elves[self.elf_map[&self.elves[i].waiting_on[1]]].value;
                if let (Some(v1), Some(v2)) = (elf1_value, elf2_value) {
                    let operation = self.elves[i].operation.as_ref().unwrap();
                    let value = operation(v1, v2);
                    self.elves[i].value = Some(value);
                }
            }
        }
        Ok(self.elves[self.elf_map[elf_name]].value.unwrap())
    }

    fn compare(&mut self, elf_name: &str, value: Complex64) -> (Complex64, Complex64) {
        self.elves[self.elf_map["humn"]].value = Some(value);
        while self.elves[self.elf_map[elf_name]].value.is_none() {
            for i in 0..self.elves.len() {
                if self.elves[i].value.is_some() {
                    continue;
                }
                let elf1_value = self.elves[self.elf_map[&self.elves[i].waiting_on[0]]].value;
                let elf2_value = self.elves[self.elf_map[&self.elves[i].waiting_on[1]]].value;
                if let (Some(v1), Some(v2)) = (elf1_value, elf2_value) {
                    let operation = self.elves[i].operation.as_ref().unwrap();
                    let value = operation(v1, v2);
                    self.elves[i].value = Some(value);
                }
            }
        }
        let elf1 = self.elves[self.elf_map[elf_name]].waiting_on[0].clone();
        let elf2 = self.elves[self.elf_map[elf_name]].waiting_on[1].clone();
        let elf1_value = self.elves[self.elf_map[&elf1]].value.unwrap();
        let elf2_value = self.elves[self.elf_map[&elf2]].value.unwrap();
        (elf1_value, elf2_value)
    }
}


fn part1(input: &str) -> i64 {
    let mut elves = input.parse::<Elves>().unwrap();
    elves.solve_for("root").unwrap().re as i64
}

fn part2(input: &str) -> i64 {
    let mut elves = input.parse::<Elves>().unwrap();
    let (elf1, elf2) = elves.compare("root", Complex64::new(0.0, 1.0));
    let result = (elf2.re - elf1.re) / elf1.im;
    result.round() as i64
}

fn main() {
    let input = include_str!("../../inputs/2022_21.txt");
    let time = std::time::Instant::now();
    println!("Part 1: {}", part1(input));
    println!("Time: {}ms", time.elapsed().as_millis());
    let time = std::time::Instant::now();
    println!("Part 2: {}", part2(input));
    println!("Time: {}ms", time.elapsed().as_millis());
}


