use ndarray::prelude::*;
use std::fmt;
use std::str::{FromStr, from_utf8};
use std::io::Write;
use std::io;
use std::collections::HashSet;

#[cfg(test)]
mod test {
    use super::*;
    fn test_input() -> &'static str {
"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"
    }
    
    #[test]
    fn part1_test() {
        assert_eq!(part1(test_input(), 10), 26);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(test_input(), 20), 56000011);
    }
}

struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Position { x, y }
    }

    fn manhattan_distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

struct Beacon {
    position: Position,
}

struct Sensor {
    position: Position,
    beacon: Beacon,
    radius: i64,
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
        let captures = re.captures(s).unwrap();
        let sensor_x = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let sensor_y = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let beacon_x = captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let beacon_y = captures.get(4).unwrap().as_str().parse::<i64>().unwrap();
        let sensor_position = Position::new(sensor_x, sensor_y);
        let beacon_position = Position::new(beacon_x, beacon_y);
        let radius = sensor_position.manhattan_distance(&beacon_position);
        Ok(Sensor {
            position: sensor_position,
            beacon: Beacon {
                position: beacon_position,
            },
            radius,
        })
    }
}

impl Sensor {
    fn covered_positions_on_line(&self, y: i64) -> Vec<i64> {
        let mut positions = Vec::new();
        let y_diff = (self.position.y - y).abs();
        if y_diff > self.radius {
            return positions;
        }
        let number_of_positions = 1 + (self.radius - y_diff) * 2;
        let first_position = self.position.x - number_of_positions / 2;
        for i in 0..number_of_positions {
            positions.push(first_position + i);
        }
        positions
    }
}

fn covered_positions_on_line(sensors: &Vec<Sensor>, y: i64) -> usize {
    let mut positions = HashSet::new();
    for sensor in sensors {
        let covered_positions = sensor.covered_positions_on_line(y);
        positions.extend(covered_positions);
    }
    for sensor in sensors {
        if sensor.position.y == y {
            positions.remove(&sensor.position.x);
        }
        if sensor.beacon.position.y == y {
            positions.remove(&sensor.beacon.position.x);
        }
    }
    positions.len()
}

fn beacon_position(sensors: &Vec<Sensor>, max: i64) -> Position {
    for sensor in sensors {
        let distance = sensor.radius + 1;
        for i in 0..distance {
            let a = Position::new(sensor.position.x + i, sensor.position.y + distance - i);
            let b = Position::new(sensor.position.x - i, sensor.position.y + distance - i);
            let c = Position::new(sensor.position.x + i, sensor.position.y - distance + i);
            let d = Position::new(sensor.position.x - i, sensor.position.y - distance + i);
            if !is_covered(&a, sensors, &max) {
                return a
            }
            if !is_covered(&b, sensors, &max) {
                return b
            }
            if !is_covered(&c, sensors, &max) {
                return c
            }
            if !is_covered(&d, sensors, &max) {
                return d
            }
        }
    }
    panic!("No beacon position found");
}

fn is_covered(position: &Position, sensors: &Vec<Sensor>, max: &i64) -> bool {
    if position.y > *max || position.x > *max || position.y < 0 || position.x < 0 {
        return true;
    }
    for sensor in sensors {
        let distance_to_position = sensor.position.manhattan_distance(position);
        if distance_to_position <= sensor.radius {
            return true;
        }
    }
    false
}

fn part1(input: &str, y: i64) -> usize {
    let sensors = input.lines().map(|line| line.parse::<Sensor>().unwrap()).collect::<Vec<_>>();
    covered_positions_on_line(&sensors, y)
}

fn part2(input: &str, max: i64) -> i64 {
    let sensors = input.lines().map(|line| line.parse::<Sensor>().unwrap()).collect::<Vec<_>>();
    let position = beacon_position(&sensors, max);
    position.x * 4000000 + position.y
}

fn main() {
    let input = include_str!("../../inputs/2022_15.txt");
    let time = std::time::Instant::now();
    let part1_answer = part1(input, 2000000);
    println!("Part 1: {} in: {}ms", part1_answer, time.elapsed().as_millis());
    let time = std::time::Instant::now();
    let part2_answer = part2(input, 4000000);
    println!("Part 2: {} in: {}ms", part2_answer, time.elapsed().as_millis());
}

