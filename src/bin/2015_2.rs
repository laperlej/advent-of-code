use std::str::FromStr;
use rayon::prelude::*;
use futures::executor::block_on;
use futures;

#[derive(Debug)]
struct BoxDimension {
    length: i64,
    width: i64,
    height: i64
}

impl FromStr for BoxDimension {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("x").collect();
        let length = parts[0].parse().unwrap();
        let width = parts[1].parse().unwrap();
        let height = parts[2].parse().unwrap();
        Ok(BoxDimension { length, width, height })
    }
}

fn parse_file(filename: String) -> Vec<BoxDimension> {
    let input = std::fs::read_to_string(filename).expect("Error reading input file");
    //read line by line, check for empty lines and store in a vector if not empty
    let numbers: Vec<BoxDimension> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().expect("Error parsing input"))
        .collect();
    numbers
}

async fn total_paper_needed(dims: &Vec<BoxDimension>) -> i64 {
    let mut total = 0;
    for dim in dims {
        let l = dim.length;
        let w = dim.width;
        let h = dim.height;
        let lw = l * w;
        let wh = w * h;
        let hl = h * l;
        let min = lw.min(wh).min(hl);
        total += 2 * lw + 2 * wh + 2 * hl + min;
    }
    total
}

async fn total_ribbon_needed(dims: &Vec<BoxDimension>) -> i64 {
    dims.par_iter()
        .map(|dim| {
            let mut ordered_dim = vec![dim.length, dim.width, dim.height];
            ordered_dim.sort();
            ordered_dim[0]*2 + ordered_dim[1]*2 + ordered_dim[0]*ordered_dim[1]*ordered_dim[2]
        })
        .sum()
}

async fn solve_aoc() {
    let box_dimentions = parse_file("../../inputs/2015_2.txt".to_string());

    let total_paper_future = total_paper_needed(&box_dimentions);
    let total_ribbon_future = total_ribbon_needed(&box_dimentions);
    let (total_paper, total_ribbon) = futures::join!(total_paper_future, total_ribbon_future);
    println!("Total paper needed: {}", total_paper);
    println!("Total ribbon needed: {}", total_ribbon);
}

fn main() {
    block_on(solve_aoc());
}
