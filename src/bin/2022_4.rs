use std::str::FromStr;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Pair {
    left: Assignment,
    right: Assignment
}

#[derive(Debug, Clone)]
struct Assignment {
    min: usize,
    max: usize,
}

impl FromStr for Pair {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");
        let left: Assignment = parts.next().unwrap().parse().unwrap();
        let right: Assignment = parts.next().unwrap().parse().unwrap();
        Ok(Pair{left, right})
    }
}

impl FromStr for Assignment {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');
        let min = split.next().unwrap().parse::<usize>().unwrap();
        let max = split.next().unwrap().parse::<usize>().unwrap();
        Ok(Assignment {min, max})
    }
}

impl Assignment {
    fn contains(&self, other: &Assignment) -> bool {
        self.min <= other.min && self.max >= other.max
    }
    fn overlap(&self, other: &Assignment) -> bool {
        self.min <= other.max && self.max >= other.min
    }
}

fn main () {
    let pairs: Vec<Pair>  = read_to_string("inputs/2022_4.txt")
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|s| s.parse::<Pair>().unwrap())
        .collect();
    let mut contain_count = 0;
    let mut overlap_count = 0;
    for pair in pairs {
        if pair.left.contains(&pair.right) || pair.right.contains(&pair.left) {
            contain_count += 1;
        }
        if pair.left.overlap(&pair.right) {
            overlap_count += 1;
        }
    }
    println!("{}", contain_count);
    println!("{}", overlap_count);
}
