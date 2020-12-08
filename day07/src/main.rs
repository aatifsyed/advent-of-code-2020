use petgraph::{dot::Dot, graphmap::GraphMap, Directed};
use std::{fs, io, path};

#[derive(Eq, Hash, Debug, Copy, Clone, PartialEq, Ord, PartialOrd)]
struct Bag(&str);

macro_rules! bag {
    ($s:expr) => {
        Bag($s.to_string())
    };
}

#[derive(Debug)]
struct Contains(usize);

struct Edge<N, E> {
    a: N,
    b: N,
    weight: E,
}

trait AddEdge<N, E> {
    fn add_edge(&mut self, e: Edge<N, E>) -> Option<E>;
}

impl AddEdge<Bag, Contains> for GraphMap<Bag, Contains, Directed> {
    fn add_edge(&mut self, e: Edge<Bag, Contains>) -> Option<Contains> {
        self.add_edge(e.a, e.b, e.weight)
    }
}

trait ToFile {
    fn to_file(&self, filename: impl AsRef<path::Path>) -> Result<(), io::Error>;
}

impl ToFile for GraphMap<Bag, Contains, Directed> {
    fn to_file(&self, filename: impl AsRef<path::Path>) -> Result<(), io::Error> {
        let dot = Dot::new(&self);
        fs::write(filename, format!("{:?}", dot))
    }
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
        g.add_node(bag!("yellow"));
        g.add_edge(bag!("yellow"), bag!("red"), Contains(3));
        g.add_edge(bag!("purple"), bag!("red"), Contains(3));
        g.to_file("g.dot").unwrap();
    }
}
