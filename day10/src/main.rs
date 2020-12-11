use fileutils;
use itertools::Itertools;
use std::collections::HashMap;

const DAY: &str = "10";

fn build_input(filepath: &str) -> Vec<isize> {
    let mut input = fileutils::numbers_from_file(filepath);

    input.sort();

    let max_joltage_adapter = input[input.len() - 1];
    let device_voltage = max_joltage_adapter + 3;

    input.push(device_voltage);
    input.insert(0, 0); // Outlet
    input
}

fn part1(filepath: &str) -> isize {
    let input = build_input(filepath);
    let mut distribution = HashMap::new();
    for (a, b) in input.iter().tuple_windows() {
        let diff = b - a;
        let counter = distribution.entry(diff).or_insert(0);
        *counter += 1
    }
    distribution.get(&1).unwrap() * distribution.get(&3).unwrap()
}

#[derive(Debug, Copy, Clone, Hash)]
struct Edge();

fn part2(filepath: &str) -> usize {
    let input = build_input(filepath);
    let device_voltage = input[input.len() - 1];
    let mut graph = petgraph::graphmap::GraphMap::<isize, isize, petgraph::Directed>::new();
    for i in &input {
        graph.add_node(*i);
        println!("added node {}", i);
    }
    for i in &input {
        for n in 1..4 {
            if graph.contains_node(i + n) {
                graph.add_edge(*i, i + n, 0);
                println!("added edge between {} and {}", i, i + n);
            }
        }
    }
    petgraph::algo::all_simple_paths::<Vec<_>, _>(&graph, 0, device_voltage, 0, None).count()
}

fn main() {
    let filepath = format!("inputs/day{}.txt", DAY);
    println!("part 1: {:?}", part1(&filepath));
    println!("part 2: {:?}", part2(&filepath));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(&format!("../inputs/day{}.txt", DAY)), 1984);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&format!("../inputs/day{}.txt", DAY)), 0);
    }
}
