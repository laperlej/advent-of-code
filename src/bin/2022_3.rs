use std::str::FromStr;
use std::fs::read_to_string;


#[derive(Debug, Clone)]
struct RuckSack1 {
    left: Vec<u8>,
    right: Vec<u8>
}

impl FromStr for RuckSack1 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mid = s.len() / 2;
        Ok(RuckSack1::new(s[..mid].as_bytes().to_vec(), s[mid..].as_bytes().to_vec()).unwrap())
    }
}

impl RuckSack1 {
    fn new (left: Vec<u8>, right: Vec<u8>) -> Result<RuckSack1, String> {
        let left = ascii_to_priority(&left)?;
        let right = ascii_to_priority(&right)?;
        Ok(RuckSack1 {left, right})
    }
    fn get_matching_item (&self) -> usize {
        let mut left: Vec<u8> = self.left.clone();
        let mut right: Vec<u8>= self.right.clone();
        left.sort();
        right.sort();
        let mut i = 0;
        let mut j = 0;
        while i < left.len() && j < right.len() {
            if left[i] == right[j] {
                return left[i].into();
            }
            if left[i] < right[j] {
                i += 1;
            } else {
                j += 1;
            }
        }
        0
    }
}

#[derive(Debug, Clone)]
struct RuckSack2 {
    elf1: Vec<u8>,
    elf2: Vec<u8>,
    elf3: Vec<u8>
}

impl FromStr for RuckSack2 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elves: Vec<Vec<u8>> = s.split_whitespace().map(|s| s.as_bytes().to_vec()).collect();
        if elves.len() != 3 {
            return Err(());
        }
        Ok(RuckSack2::new(&elves[0], &elves[1], &elves[2]).unwrap())
    }
}

impl RuckSack2 {
    fn new (elf1: &Vec<u8>, elf2: &Vec<u8>, elf3: &Vec<u8>) -> Result<RuckSack2, String> {
        let elf1 = ascii_to_priority(&elf1)?;
        let elf2 = ascii_to_priority(&elf2)?;
        let elf3 = ascii_to_priority(&elf3)?;
        Ok(RuckSack2 {elf1, elf2, elf3})
    }
    fn get_matching_item (&self) -> usize {
        let mut elf1: Vec<u8> = self.elf1.clone();
        let mut elf2: Vec<u8>= self.elf2.clone();
        let mut elf3: Vec<u8>= self.elf3.clone();
        elf1.sort();
        elf2.sort();
        elf3.sort();
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        while i < elf1.len() && j < elf2.len() && k < elf3.len() {
            if elf1[i] == elf2[j] && elf2[j] == elf3[k] {
                return elf1[i].into();
            }
            if elf1[i] < elf2[j] {
                i += 1;
            } else if elf2[j] < elf3[k] {
                j += 1;
            } else {
                k += 1;
            }
        }
        0
    }
}

fn ascii_to_priority (items: &Vec<u8>) -> Result<Vec<u8>, String> {
    let mut priority: Vec<u8> = Vec::new();
    for i in items {
        if i >= &97 {
            priority.push(i - 96);
        } else if i >= &65 {
            priority.push(i - 64 + 26);
        } else {
            return Err(format!("invalid item: {}", i));
        }
    }
    Ok(priority)
}


fn main () {
    let rucksacks: Vec<RuckSack1>  = read_to_string("inputs/2022_3.txt")
        .unwrap()
        .lines()
        .map(|s| s.parse::<RuckSack1>().unwrap())
        .collect();
    let priorities: Vec<usize>= rucksacks.iter().map(|r| r.get_matching_item()).collect();
    let total: usize = priorities.iter().sum();
    println!("total: {}", total);

    //read every 3 lines into a RuckSack2
    let rucksacks2: Vec<RuckSack2>  = read_to_string("inputs/2022_3.txt")
        .unwrap()
        .lines().filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|s| s.join(" ").parse::<RuckSack2>().unwrap())
        .collect();
    let priorities2: Vec<usize>= rucksacks2.iter().map(|r| r.get_matching_item()).collect();
    let total2: usize = priorities2.iter().sum();
    println!("total2: {}", total2);
}
