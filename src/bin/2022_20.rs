use std::str::FromStr;
use std::collections::{ VecDeque, HashMap, BinaryHeap };
use std::io::Write;
use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::Ordering;


#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() ->  &'static str {
"1
2
-3
3
-2
0
4"
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(test_input()), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(test_input()), 1623178306);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Num {
    value: i64,
    index: i64
}

struct NumList {
    nums: Vec<Num>,
    decryption_key: i64,
    offset: i64
}

impl FromStr for NumList {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = Vec::new();
        let decryption_key = 811589153;
        for (i, line) in s.trim().lines().enumerate() {
            let value = line.parse::<i64>().unwrap();
            nums.push(Num { value, index: i as i64 });
        }
        Ok(NumList { nums, decryption_key, offset: 0 })
    }
}

impl NumList {
    fn apply_decryption_key(&mut self, decryption_key: i64) {
        for num in &mut self.nums {
            num.value *= decryption_key;
        }
    }

    fn find_by_original_index(&self, index: i64) -> (usize, &Num) {
        self.nums.iter().find_position(|n| n.index == index).unwrap()
    }

    fn swap(&mut self, i: i64, j: i64) {
        let i = (i + self.nums.len() as i64 *4) % self.nums.len() as i64;
        let j = (j + self.nums.len() as i64 *4) % self.nums.len() as i64;
        self.nums.swap(i as usize, j as usize);
    }

    fn rotate_right(&mut self, n: i64) {
        let n = (n + self.nums.len() as i64 *4) % self.nums.len() as i64;
        self.nums.rotate_right(n as usize);
    }

    fn rotate_left(&mut self, n: i64) {
        let n = (n + self.nums.len() as i64 *4) % self.nums.len() as i64;
        self.nums.rotate_left(n as usize);
    }

    fn decrypt(&mut self, mixes: i64) {
        let mut i = 0;
        let mut nb_mixes = mixes;
        while nb_mixes > 0 {
            while i < self.nums.len() {
                let (j, num) = self.find_by_original_index(i as i64);
                let value = num.value;
                match num.value.cmp(&0) {
                    Ordering::Less => {
                        for k in 0..value.abs()%(self.nums.len()-1) as i64 {
                            self.swap(j as i64 - k, j as i64 - k - 1);
                        }
                    },
                    Ordering::Equal => {},
                    Ordering::Greater => {
                        for k in 0..value.abs()%(self.nums.len()-1) as i64 {
                            self.swap(j as i64 + k, j as i64 + k + 1);
                        }
                    }
                }
                i+=1;
            }
            println!("{:?}", self.nums);
            nb_mixes -= 1;
            i = 0;
        }
    }

    fn get(&self, index: i64) -> i64 {
        let (zero_index, _) = self.nums.iter().find_position(|n| n.value == 0).unwrap();
        let i = (zero_index as i64 + index + self.nums.len() as i64) % self.nums.len() as i64;
        self.nums[i as usize].value
    }

    fn sum_of_coordinates(&self) -> i64 {
        self.get(1000) + self.get(2000) + self.get(3000)
    }
}

/*
0,1,2,3,4,5,6
0,1,2,4,3,5,6
0,1,2,4,5,3,6
0,1,2,4,5,6,3
3,1,2,4,5,6,0
1,3,2,4,5,6,0
1,2,3,4,5,6,0
1,2,4,3,5,6,0


811589153, 1623178306, -2434767459, 2434767459, -1623178306, 0, 3246356612
811589153%7, 1623178306%7, -2434767459%7, 2434767459%7, -1623178306%7, 0, 3246356612%7

 4, 1,-5, 5,-1, 0, 2
 1,-5, 5,-1, 4, 0, 2
-5, 1, 5,-1, 4, 0, 2

*/

fn part1(input: &str) -> i64 {
    let mut nums = input.parse::<NumList>().unwrap();
    nums.decrypt(1);
    nums.sum_of_coordinates()
}

fn part2(input: &str) -> i64 {
    let mut nums = input.parse::<NumList>().unwrap();
    nums.apply_decryption_key(811589153);
    nums.decrypt(10);
    nums.sum_of_coordinates()
}

fn main() {
    let input = include_str!("../../inputs/2022_20.txt");
    let time = std::time::Instant::now();
    println!("Part 1: {}", part1(input));
    println!("Time: {}ms", time.elapsed().as_millis());
    let time = std::time::Instant::now();
    println!("Part 2: {}", part2(input));
    println!("Time: {}ms", time.elapsed().as_millis());
}

