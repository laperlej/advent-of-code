use ndarray::prelude::*;
use std::fs::read_to_string;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;
    fn test_input() -> &'static str {
        "30373
25512
65332
33549
35390"
    }

    #[test]
    fn part1_test() {
        let input = test_input();
        assert_eq!(part1(input), 21);
    }
    #[test]
    fn part2_test() {
        let input = test_input();
        assert_eq!(part2(input), 8);
    }
}

struct Forest {
    //2d array of trees
    trees: Array2<u8>,
}

impl FromStr for Forest {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees: Vec<Vec<u8>> = s.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>()).collect();
        Ok(Forest {
            trees: Array2::from_shape_vec((trees.len(), trees[0].len()), trees.concat()).unwrap(),
        })
    }
}

impl Forest {
    fn visible_trees(&self) -> usize {
        let mut visible_grid: Array2<u32> = Array2::zeros(self.trees.dim());
        let dims = self.trees.dim();
        let outer_ring = (dims.0 + dims.1 - 2) * 2;
        for left_row in 1..dims.0-1 {
            let mut previous_height = self.trees[[left_row, 0]];
            for index in 1..dims.0-1 {
                if self.trees[[left_row, index]] > previous_height {
                    visible_grid[[left_row, index]] = 1;
                    previous_height = previous_height.max(self.trees[[left_row, index]]);
                }
            }
        }
        for right_row in 1..dims.0-1 {
            let mut previous_height = self.trees[[right_row, dims.0-1]];
            for index in (1..dims.0-1).rev() {
                if self.trees[[right_row, index]] > previous_height {
                  visible_grid[[right_row, index]] = 1;
                  previous_height = previous_height.max(self.trees[[right_row, index]]);
                }
            }
        }
        for top_column in 1..dims.1-1 {
            let mut previous_height = self.trees[[0, top_column]];
            for index in 1..dims.1-1 {
                if self.trees[[index, top_column]] > previous_height {
                  visible_grid[[index, top_column]] = 1;
                  previous_height = previous_height.max(self.trees[[index, top_column]]);
                }
            }
        }
        for bottom_column in 1..dims.1-1 {
            let mut previous_height = self.trees[[dims.1-1, bottom_column]];
            for index in (1..dims.1-1).rev() {
                if self.trees[[index, bottom_column]] > previous_height {
                  visible_grid[[index, bottom_column]] = 1;
                  previous_height = previous_height.max(self.trees[[index, bottom_column]]);
                }
            }
        }
        println!("{:?}", visible_grid);
        
        outer_ring + visible_grid.iter().sum::<u32>() as usize
    }

    fn scenic_score_helper(&self, row: usize, column: usize) -> usize {
        let mut left_scenic_score = 0;
        let mut right_scenic_score = 0;
        let mut top_scenic_score = 0;
        let mut bottom_scenic_score = 0;
        //check left
        for cur_column in (0..column).rev() {
            if self.trees[[row, cur_column]] >= self.trees[[row, column]] {
                left_scenic_score += 1;
                break;
            } else {
                left_scenic_score += 1;
            }
        }
        //check right
        for cur_column in column+1..self.trees.dim().1 {
            if self.trees[[row, cur_column]] >= self.trees[[row, column]] {
                right_scenic_score += 1;
                break;
            } else {
                right_scenic_score += 1;
            }
        }
        //check up
        for cur_row in (0..row).rev() {
            if self.trees[[cur_row, column]] >= self.trees[[row, column]] {
                top_scenic_score += 1;
                break;
            } else {
                top_scenic_score += 1;
            }
        }
        //check down
        for cur_row in row+1..self.trees.dim().0 {
            if self.trees[[cur_row, column]] >= self.trees[[row, column]] {
                bottom_scenic_score += 1;
                break;
            } else {
                bottom_scenic_score += 1;
            }
        }
        left_scenic_score * right_scenic_score * top_scenic_score * bottom_scenic_score
    }

    fn scenic_score(&self) -> usize {
        let mut score_grid: Array2<usize> = Array2::zeros(self.trees.dim());
        for row in 0..self.trees.dim().0 {
            for column in 0..self.trees.dim().1 {
                score_grid[[row, column]] = self.scenic_score_helper(row, column);
            }
        }
        score_grid.iter().max().unwrap().clone() as usize
    }
}



fn part1(input: &str) -> usize {
    let forest = input.parse::<Forest>().unwrap();
    forest.visible_trees()
}
fn part2(input: &str) -> usize {
    let forest = input.parse::<Forest>().unwrap();
    forest.scenic_score()
}

fn main() {
    let input = read_to_string("inputs/2022_8.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
