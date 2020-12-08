use itertools::Itertools;
use petgraph::{dot::Dot, graphmap::GraphMap, Directed};
use regex::Regex;
use regexutils::ExtractCaptured;
use std::str::FromStr;
use std::{fs, io, path};

#[macro_use]
extern crate lazy_static;

#[derive(Eq, Hash, Debug, Copy, Clone, PartialEq, Ord, PartialOrd)]
struct Bag<'a>(&'a str);

#[derive(Debug, Copy, Clone)]
struct Contains(usize);

struct Edge<N, E> {
    a: N,
    b: N,
    weight: E,
}

struct Edges<N, E> {
    a: N,
    v: Vec<(N, E)>,
}

/// A single line
impl Edges<Bag<'_>, Contains> {
    fn decompose(&self) -> Vec<Edge<Bag<'_>, Contains>> {
        self.v
            .iter()
            .map(|tuple| Edge {
                a: self.a,
                b: tuple.0,
                weight: tuple.1,
            })
            .collect()
    }
}

impl FromStr for Edges<Bag<'_>, Contains> {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut caps = BAG_REGEX.captures_iter(s);

        // Pull out the first capture group. It is the A node
        let (head, tail) = (caps.next().expect("No matches!"), caps);

        let head = &head.extract_captured::<String>("description")[..];
        let head = Bag(head);

        // Pull out our vector of B nodes and weights, if they exist.
        let tail = tail
            .map(|cap| {
                let description = &cap.extract_captured::<String>("description")[..];
                let number = cap.extract_captured::<usize>("number");
                (Bag(description), Contains(number))
            })
            .collect();
        Ok(Edges { a: head, v: tail })
    }
}

trait ToFile {
    fn to_file(&self, filename: impl AsRef<path::Path>) -> Result<(), io::Error>;
}

impl ToFile for GraphMap<Bag<'_>, Contains, Directed> {
    fn to_file(&self, filename: impl AsRef<path::Path>) -> Result<(), io::Error> {
        let dot = Dot::new(&self);
        fs::write(filename, format!("{:?}", dot))
    }
}

lazy_static! {
    static ref BAG_REGEX: Regex = Regex::new(r"(P<number>\d )?(P<description>\w+ \w+) bag")
        .expect("Couldn't compile bag regex");
}

const DAY: &str = "07";

fn part1(filepath: &str) {}

fn part2(filepath: &str) {}

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
        assert_eq!(part1(&format!("../inputs/day{}.txt", DAY)), ());
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&format!("../inputs/day{}.txt", DAY)), ());
    }
    #[test]
    fn visualize_graph() {
        let mut g = GraphMap::<Bag, Contains, Directed>::new();
        g.add_node(Bag("yellow"));
        g.add_edge(Bag("yellow"), Bag("red"), Contains(3));
        g.add_edge(Bag("purple"), Bag("red"), Contains(4));
        g.to_file("g.dot").unwrap();
    }
}
