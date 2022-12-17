use std::str::FromStr;
use std::iter::FromIterator;
use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;
    fn test_input() -> &'static str {
        "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(test_input()), 10605);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(test_input()), 2713310158);
    }

}

#[derive(Debug, Clone)]
struct Monkeys {
    monkeys: Vec<Monkey>,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: (char, String),
    test: i64,
    if_true: usize,
    if_false: usize,
    count: usize,
}

impl Monkeys {
    fn play_round(&mut self, is_part1: bool) {
        let modulo = self.monkeys.iter().map(|m| m.test).product();
        for i in 0..self.monkeys.len() { 
            let transfers: Vec<(usize, i64)> = self.monkeys[i].play(is_part1, modulo);
            for (monkey_index, worry_level) in transfers.iter() {
                self.monkeys[*monkey_index].items.push(*worry_level)
            }

        }
    }

    fn result(&self) -> i64 {
        let mut counts: Vec<usize> = self.monkeys.iter().map(|m| m.count).collect();
        counts.sort();
        counts.iter().rev().take(2).product::<usize>() as i64
    }
}

impl Monkey {
    fn play(&mut self, is_part1: bool, modulo: i64) -> Vec<(usize, i64)> {
        let mut transfers = Vec::new();
        for item in self.items.iter() {
            self.count += 1;
            let mut new_item = match self.operation.0 {
                '*' => {
                    match self.operation.1.as_str() {
                        "old" => {
                            item * item
                        }
                        value => {
                            item * value.parse::<i64>().unwrap()
                        }
                    }
                }
                '+' => {
                    match self.operation.1.as_str() {
                        "old" => {
                            item + item
                        }
                        value => {
                            item + value.parse::<i64>().unwrap()
                        }
                    }
                }
                _ => panic!("WTF invalid operator")
            };
            if is_part1 {
                new_item /= 3;
            } else {
                new_item %= modulo;
            }
            if new_item % self.test == 0 {
                transfers.push((self.if_true, new_item))
            } else {
                transfers.push((self.if_false, new_item))
            }
        }
        self.items = Vec::new();
        transfers
    }
}


impl FromIterator<Monkey> for Monkeys {
    fn from_iter<I: IntoIterator<Item = Monkey>>(iter: I) -> Self {
        let mut monkeys = Vec::new();
        for monkey in iter {
            monkeys.push(monkey);
        }
        Monkeys { monkeys }
    }
}

impl FromStr for Monkeys {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split("\n\n")
          .map(|line| line.parse::<Monkey>())
          .collect::<Result<Monkeys, _>>()
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn  from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let items_re = Regex::new(r"(\d+)").unwrap();
        let items: Vec<i64> = items_re.find_iter(lines[1]).map(|n| n.as_str().parse::<i64>().unwrap()).collect();
        let mut operation_split = lines[2].split(' ').rev().take(2);
        let operand: String = operation_split.next().unwrap().to_string();
        let operation: char = operation_split.next().unwrap().chars().next().unwrap();
        let test: i64 = lines[3].split(' ').last().unwrap().parse().unwrap();
        let if_true: usize = lines[4].split(' ').last().unwrap().parse().unwrap();     
        let if_false: usize = lines[5].split(' ').last().unwrap().parse().unwrap();   

        Ok(Monkey {
            items,
            operation: (operation, operand),
            test,
            if_true,
            if_false,
            count: 0,
        })
    }
}

fn part1(input: &str) -> i64 {
    let mut monkeys: Monkeys = input.parse().unwrap();
    for _ in 0..20 {
        monkeys.play_round(true)
    }
    monkeys.result()
}

fn part2(input: &str) -> i64 {
    let mut monkeys: Monkeys = input.parse().unwrap();
    for _ in 0..10000 {
        monkeys.play_round(false)
    }
    monkeys.result()
}

fn main() {
    let input = include_str!("../../inputs/2022_11.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
} 
