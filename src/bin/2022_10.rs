use std::fs::read_to_string;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;
    fn test_input() -> &'static str {
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
    }

    fn example_input() -> &'static str {
        "noop
addx 3
addx -5"
    }

    fn test2_expected() -> &'static str {
        "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."}

    #[test]
    fn example_test() {
        let input = example_input();
        let mut cpu = Cpu::new();
        for line in input.lines() {
            let instruction = line.parse::<Instruction>().unwrap();
            cpu.execute(&instruction);
        }
        assert_eq!(cpu.value_at_cycle[0], 1);
        assert_eq!(cpu.value_at_cycle[1], 1);
        assert_eq!(cpu.value_at_cycle[2], 1);
        assert_eq!(cpu.value_at_cycle[3], 4);
        assert_eq!(cpu.value_at_cycle[4], 4);
    }
    #[test]
    fn part1_test() {
        let input = test_input();
        assert_eq!(part1(input), 13140);
    }
    fn part2_test() {
        let input = test_input();
        assert_eq!(part2(input), test2_expected());
    }
}

struct Instruction{
    instruction: String,
    value: isize,
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(" ");
        let instruction = tokens.next().unwrap().to_string();
        let value = match tokens.next() {
            Some(v) => v.parse().unwrap(),
            None => 0,
        };
        Ok(Instruction {
            instruction,
            value,
        })
    }
}

struct CRT {
    pixels: Vec<Vec<char>>,
}    

impl CRT {
    fn new() -> CRT {
        CRT {
            pixels: vec![vec!['.'; 40]; 6],
        }
    }
    fn draw(&mut self, cpu: &Cpu) -> String {
        for cycle in 0..40*6 {
            let col = cycle % 40;
            let row = cycle / 40;
            let position = cpu.value_at_cycle[cycle];
            if (position - col as isize).abs() <= 1 {
                self.pixels[row][col] = '#';
            }
        }
        let lines: Vec<String> = self.pixels.iter().map(|line| line.iter().collect()).collect();
        lines.join("\n")
    }
}


struct Cpu {
    register: isize,
    cycle: usize,
    value_at_cycle: Vec<isize>
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            register: 1,
            cycle: 0,
            value_at_cycle: vec![],
        }
    }
    fn execute(&mut self, instruction: &Instruction) {
        match instruction.instruction.as_str() {
            "addx" => {
                self.value_at_cycle.push(self.register);
                self.cycle += 1;
                self.value_at_cycle.push(self.register);
                self.cycle += 1;
                self.register += instruction.value;

            },
            "noop" => {
                self.value_at_cycle.push(self.register);
                self.cycle += 1;
            },
            _ => panic!("Unknown instruction {}", instruction.instruction),
        }
    }
}



fn part2(s: &str) -> String {
    let mut cpu = Cpu::new();
    for line in s.lines() {
        let instruction = line.parse::<Instruction>().unwrap();
        cpu.execute(&instruction);
    }
    let mut crt = CRT::new();
    crt.draw(&cpu)
}

fn part1(s: &str) -> isize {
    let mut cpu = Cpu::new();
    for line in s.lines() {
        let instruction = line.parse::<Instruction>().unwrap();
        cpu.execute(&instruction);
    }
    let mut total = 0;
    for cycle in (20..cpu.value_at_cycle.len()).step_by(40) {
        total += cpu.value_at_cycle[cycle-1] * cycle as isize;
    }
    total
}

fn main() {
    let input = read_to_string("inputs/2022_10.txt").expect("Could not read input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2:\n{}", part2(&input));
}
