use std::str::FromStr;
use std::collections::{ VecDeque, HashMap, BinaryHeap };
use std::io::Write;
use itertools::Itertools;
use rayon::prelude::*;


#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() ->  &'static str {
"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(test_input()), 33);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(test_input()), 3472);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cost {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Blueprint {
    ore: Cost,
    clay: Cost,
    obsidian: Cost,
    geode: Cost
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Blueprints(Vec<Blueprint>);

impl FromStr for Blueprints {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blueprints = Vec::new();
        for line in s.trim().lines() {
            let mut words = line.split_whitespace();
            let id = words.nth(1).unwrap().replace(':',"").parse::<i32>().unwrap();

            let ore_robot_ore_cost = words.nth(4).unwrap().parse::<i32>().unwrap();
            let ore_robot_cost = Cost { ore: ore_robot_ore_cost, clay: 0, obsidian: 0, geode: 0 };

            let clay_robot_ore_cost = words.nth(5).unwrap().parse::<i32>().unwrap();
            let clay_robot_cost = Cost { ore: clay_robot_ore_cost, clay: 0, obsidian: 0, geode: 0 };
            
            let obsidian_robot_ore_cost = words.nth(5).unwrap().parse::<i32>().unwrap();
            let obsidian_robot_clay_cost = words.nth(2).unwrap().parse::<i32>().unwrap();
            let obsidian_robot_cost = Cost { ore: obsidian_robot_ore_cost, clay: obsidian_robot_clay_cost, obsidian: 0, geode: 0 };

            let geode_robot_ore_cost = words.nth(5).unwrap().parse::<i32>().unwrap();
            let geode_robot_obsidian_cost = words.nth(2).unwrap().parse::<i32>().unwrap();
            let geode_robot_cost = Cost { ore: geode_robot_ore_cost, clay: 0, obsidian: geode_robot_obsidian_cost, geode: 0 };

            blueprints.push(Blueprint { ore: ore_robot_cost, clay:clay_robot_cost, obsidian:obsidian_robot_cost, geode:geode_robot_cost });
        }
        Ok(Blueprints(blueprints))
    }
}

impl Blueprints {
    fn first_three(&self, time_limit: i32) -> i64 {
        let mut total = 1;
        for blueprint in self.0.iter().take(3) {
            total *= blueprint.max_geode(time_limit);
        }
        total
    }

    fn quality(&self, time_limit: i32) -> i64 {
        let mut quality = 0;
        for (i, blueprint) in self.0.iter().enumerate() {
            let geode_count = blueprint.max_geode(time_limit);
            quality += (i as i64+1) * geode_count
        }
        quality
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
    time: i32,
}

impl Blueprint {
    fn max_geode(&self, time_limit: i32) -> i64 {
        let mut stack = Vec::with_capacity(1000);
        let starting_state = State { ore: 0, clay: 0, obsidian: 0, geode: 0, ore_robots: 1, clay_robots: 0, obsidian_robots: 0, geode_robots: 0, time: time_limit };
        stack.push(starting_state);
        let mut best: i64 = 0;

        while let Some(state) = stack.pop() {
            let possible_best: i32 = state.geode + state.geode_robots * state.time + (1..(state.time)).sum::<i32>();
            if possible_best < best as i32 {
                continue;
            }
            let time_to_next_ore_robot = 0.max((self.ore.ore - state.ore + state.ore_robots - 1) / state.ore_robots) +1;
            let time_to_next_clay_robot = match state.ore_robots {
                0 => i32::MAX,
                _ => 0.max((self.clay.ore - state.ore + state.ore_robots - 1) / state.ore_robots) + 1
            };
            let time_to_next_obsidian_robot = match state.ore_robots {
                0 => i32::MAX,
                _ => match state.clay_robots {
                    0 => i32::MAX,
                    _ => {
                        let time_to_ore = 0.max((self.obsidian.ore - state.ore + state.ore_robots - 1) / state.ore_robots) + 1;
                        let time_to_clay = 0.max((self.obsidian.clay - state.clay + state.clay_robots-1) / state.clay_robots) + 1;
                        time_to_ore.max(time_to_clay)
                    }
                }
            };
            let time_to_next_geode_robot = match state.ore_robots {
                0 => i32::MAX,
                _ => match state.obsidian_robots {
                    0 => i32::MAX,
                    _ => {
                        let time_to_ore = 0.max((self.geode.ore - state.ore + state.ore_robots-1) / state.ore_robots) + 1;
                        let time_to_obsidian = 0.max((self.geode.obsidian - state.obsidian+ state.obsidian_robots-1) / state.obsidian_robots) + 1;
                        time_to_ore.max(time_to_obsidian)
                    }
                }
            };
            if time_to_next_ore_robot <= state.time {
                let new_state = State { ore: state.ore + state.ore_robots*time_to_next_ore_robot - self.ore.ore,
                                        clay: state.clay + state.clay_robots*time_to_next_ore_robot - self.ore.clay,
                                        obsidian: state.obsidian + state.obsidian_robots*time_to_next_ore_robot - self.ore.obsidian,
                                        geode: state.geode + state.geode_robots*time_to_next_ore_robot - self.ore.geode,
                                        ore_robots: state.ore_robots + 1,
                                        clay_robots: state.clay_robots,
                                        obsidian_robots: state.obsidian_robots,
                                        geode_robots: state.geode_robots,
                                        time: state.time - time_to_next_ore_robot };
                stack.push(new_state);
            }
            if time_to_next_clay_robot <= state.time {
                let new_state = State { ore: state.ore + state.ore_robots*time_to_next_clay_robot - self.clay.ore,
                                        clay: state.clay + state.clay_robots*time_to_next_clay_robot - self.clay.clay,
                                        obsidian: state.obsidian + state.obsidian_robots*time_to_next_clay_robot - self.clay.obsidian,
                                        geode: state.geode + state.geode_robots*time_to_next_clay_robot - self.clay.geode,
                                        ore_robots: state.ore_robots,
                                        clay_robots: state.clay_robots + 1,
                                        obsidian_robots: state.obsidian_robots,
                                        geode_robots: state.geode_robots,
                                        time: state.time - time_to_next_clay_robot };
                stack.push(new_state);
            }
            if time_to_next_obsidian_robot <= state.time {
                let new_state = State { ore: state.ore + state.ore_robots*time_to_next_obsidian_robot - self.obsidian.ore,
										 clay: state.clay + state.clay_robots*time_to_next_obsidian_robot - self.obsidian.clay,
										 obsidian: state.obsidian + state.obsidian_robots*time_to_next_obsidian_robot - self.obsidian.obsidian,
										 geode: state.geode + state.geode_robots*time_to_next_obsidian_robot - self.obsidian.geode,
										 ore_robots: state.ore_robots,
										 clay_robots: state.clay_robots,
										 obsidian_robots: state.obsidian_robots + 1,
										 geode_robots: state.geode_robots,
										 time: state.time - time_to_next_obsidian_robot };
                stack.push(new_state);
            }
            if time_to_next_geode_robot <= state.time {
                let new_state = State { ore: state.ore + state.ore_robots*time_to_next_geode_robot - self.geode.ore,
										 clay: state.clay + state.clay_robots*time_to_next_geode_robot - self.geode.clay,
										 obsidian: state.obsidian + state.obsidian_robots*time_to_next_geode_robot - self.geode.obsidian,
										 geode: state.geode + state.geode_robots*time_to_next_geode_robot - self.geode.geode,
										 ore_robots: state.ore_robots,
										 clay_robots: state.clay_robots,
										 obsidian_robots: state.obsidian_robots,
										 geode_robots: state.geode_robots + 1,
										 time: state.time - time_to_next_geode_robot  };
                stack.push(new_state);
            }
            if time_to_next_ore_robot >= state.time && time_to_next_clay_robot >= state.time && time_to_next_obsidian_robot >= state.time && time_to_next_geode_robot >= state.time {
                best = best.max(state.geode as i64 + state.geode_robots as i64 * (state.time as i64));
                continue;
            }
        }
        best
    }
}

fn part1(input: &str) -> i64 {
    let blueprints = input.parse::<Blueprints>().unwrap();
    blueprints.quality(24)
}

fn part2(input: &str) -> i64 {
    let blueprints = input.parse::<Blueprints>().unwrap();
    blueprints.first_three(32)
}

fn main() {
    let input = include_str!("../../inputs/2022_19.txt");
    let time = std::time::Instant::now();
    println!("Part 1: {}", part1(input));
    println!("Time: {}ms", time.elapsed().as_millis());
    let time = std::time::Instant::now();
    println!("Part 2: {}", part2(input));
    println!("Time: {}ms", time.elapsed().as_millis());
}
