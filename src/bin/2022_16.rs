use std::str::FromStr;
use std::collections::{ VecDeque, HashMap };
use std::io::Write;
use ndarray::prelude::*;
use itertools::Itertools;
use rayon::prelude::*;


#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() ->  &'static str {
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(test_input()), 1651);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(test_input()), 1707);
    }
}

#[derive(Debug, Clone)]
struct Valves {
    valves: HashMap<String, Valve>,
}

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: i64,
    tunnels: Vec<String>,
}

impl FromStr for Valves {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut valves = HashMap::new();
        for line in s.replace("valves","valve").lines() {
            let mut parts = line.split(" has flow rate=");
            let name = parts.next().unwrap().split(' ').last().unwrap().to_string();
            let mut parts = parts.next().unwrap().split(';');
            let flow_rate = parts.next().unwrap().parse().unwrap();
            let parts = parts.next().unwrap().split("valve ");
            let tunnels = parts.last().unwrap().split(", ").map(|s| s.to_string()).collect();
            valves.insert(name.clone(), Valve { name, flow_rate, tunnels });
        }
        Ok(Valves { valves })
    }
}

struct Graph {
    adj_matrix : Array2<i64>,
    flow_rates : Array1<i64>,
}

fn find_distance(start: String, end: String, valves: &Valves) -> i64 {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    queue.push_back((start.clone(), 0));
    visited.insert(start, true);
    while let Some((valve_name, depth)) = queue.pop_front() {
        if *valve_name == end {
            return depth;
        }
        visited.insert(valve_name.clone(), true);
        for tunnel in valves.valves.get(&valve_name).unwrap().tunnels.iter() {
            if !visited.contains_key(tunnel) {
                queue.push_back((tunnel.clone(), depth+1));
            }
        }
    }
    panic!("No path found");
}

impl Graph {
    fn new(valves: &Valves) -> Self {
        let mut non_0_valves = valves.valves.iter().filter(|(n,v)| *n == "AA" || v.flow_rate > 0).collect::<Vec<_>>();
        non_0_valves.sort_by_key(|(n,_)| *n);
        let mut adj_matrix = Array2::zeros((non_0_valves.len(), non_0_valves.len()));
        for i in 0..non_0_valves.len() {
            for j in 0..non_0_valves.len() {
                adj_matrix[[i,j]] = find_distance(non_0_valves[i].1.name.clone(), non_0_valves[j].1.name.clone(), valves);
            }
            
        }
        let flow_rates = Array1::from_iter(non_0_valves.iter().map(|(_,v)| v.flow_rate));
        Graph { adj_matrix, flow_rates }
    }

    fn best_flow_helper(&self, visited: u16, time: i64) -> i64 {
        let mut best_flow = 0;
        let mut stack = Vec::with_capacity(100);
        stack.push((visited, 0, time, 0));
        while let Some((visited, node, time, flow)) = stack.pop() {
            for i in 0..self.adj_matrix.nrows() {
                if  visited & (1<<i) == 0 { 
                    let mut new_visited = visited;
                    new_visited |= 1<<i;
                    let new_time = time - 1 - self.adj_matrix[[node, i]];
                    if new_time > 0 {
                        stack.push((new_visited, i, new_time, flow + self.flow_rates[i]*new_time));
                    }
                    best_flow = best_flow.max(flow);
                }
            }
        }
        best_flow
    }

    fn best_flow(&self) -> i64 {
        let visited = 0;
        self.best_flow_helper(visited, 30)
    }

    fn best_elephant_flow(&self) -> i64 {
        (0..u16::MAX/2).into_par_iter()
            .step_by(2)
            .filter(|v| v.count_ones() == 7)
            .map(|visited| { self.best_flow_helper(visited, 26) + self.best_flow_helper(!visited ^ 1, 26)
            })
            .max().unwrap()
    }
}

fn part1(input: &str) -> i64 {
    let valves: Valves = input.parse().unwrap();
    let graph = Graph::new(&valves);
    graph.best_flow()
}

fn part2(input: &str) -> i64 {
    let valves: Valves = input.parse().unwrap();
    let graph = Graph::new(&valves);
    graph.best_elephant_flow()
}

fn main() {
    let input = include_str!("../../inputs/2022_16.txt");
    let time = std::time::Instant::now();
    println!("Part 1: {}", part1(input));
    println!("Time: {}ms", time.elapsed().as_millis());
    let time = std::time::Instant::now();
    println!("Part 2: {}", part2(input));
    println!("Time: {}ms", time.elapsed().as_millis());
}
