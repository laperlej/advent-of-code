use std::str::FromStr;
use std::cmp::Ordering;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::sequence::delimited;
use nom::multi::separated_list0;
use nom::branch::alt;

#[cfg(test)]
mod test{
    use super::*;
    fn test_input() -> &'static str {
"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"}

    #[test]
    fn part1_test() {
        assert_eq!(part1(test_input()), 13);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(test_input()), 140);
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Value {
    Int(i64),
    List(Vec<Value>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.partial_cmp(b),
            (Value::List(a), Value::List(b)) => a.partial_cmp(b),
            (Value::Int(a), Value::List(_)) => Value::List(vec![Value::Int(*a)]).partial_cmp(other),
            (Value::List(_), Value::Int(b)) => self.partial_cmp(&Value::List(vec![Value::Int(*b)])),
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Packet {
    content: Value,
}


impl Packet {
    fn parse_list(input: &str) -> nom::IResult<&str, Value> {
        let parsed = delimited(
                char('['),
                separated_list0(
                    char(','),
                    Packet::parse_value),
                char(']'))(input);
        match parsed {
            Ok((i, v)) => Ok((i, Value::List(v))),
            Err(e) => Err(e),
        }
    }
    
    fn parse_number(input: &str) -> nom::IResult<&str, Value> {
        let parsed = digit1(input);
        match parsed {
            Ok((i, v)) => Ok((i, Value::Int(v.parse().unwrap()))),
            Err(e) => Err(e),
        }
    }

    fn parse_value(input: &str) -> nom::IResult<&str, Value> {
        alt((
            Packet::parse_number,
            Packet::parse_list,
        ))(input)
    }
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(());
        }
        let (input, value) = Packet::parse_value(s).unwrap();
        if !input.is_empty() {
            return Err(());
        }
        Ok(Packet{content: value})
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone)]
struct Pair {
    first: Packet,
    second: Packet,
}

impl FromStr for Pair {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        Ok(Pair {
            first: lines.next().unwrap().parse().unwrap(),
            second: lines.next().unwrap().parse().unwrap(),
        })
    }
}

impl Pair { 
    fn is_right_order(&self) -> bool {
        self.first < self.second
    }
}

fn part1(input: &str) -> usize {
    let pairs: Vec<Pair> = input.split("\n\n").map(|s| s.parse().unwrap()).collect();
    let mut total = 0;
    for (i, pair) in pairs.iter().enumerate() {
        if pair.is_right_order(){
            total += i+1;
        }
    }
    total
}

fn part2(input: &str) -> usize {
    let mut packets: Vec<Packet> = input.lines().flat_map(|s| s.parse()).collect();
    let divider1 = "[[2]]".parse::<Packet>().unwrap();
    let divider2 = "[[6]]".parse::<Packet>().unwrap();
    packets.push(divider1.clone());
    packets.push(divider2.clone());
    packets.sort();
    let mut total = 1;
    for (i, packet) in packets.iter().enumerate() {
        if packet == &divider1 {
            total *= i+1;
        }
        if packet == &divider2 {
            total *= i+1;
        }
    }
    total
}

fn main() {
    let input = include_str!("../../inputs/2022_13.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

