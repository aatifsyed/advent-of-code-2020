use petgraph::{dot::Dot, graphmap::GraphMap, Directed};
use regex::Regex;
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
    fn from_line(line: &str) -> Self {
        BAG_REGEX_A.is_match(line);
        Edges {
            a: Bag("Hello"),
            v: vec![],
        }
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
    static ref BAG_REGEX_A: Regex = Regex::new(r"^(\w+ \w+) bags contain(,? \d+ \w+ \w+ bags?)*")
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
