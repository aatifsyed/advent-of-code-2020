use petgraph::{dot::Dot, graphmap::GraphMap, Directed};
use std::{fs, io, path};

#[derive(Eq, Hash, Debug, Copy, Clone, PartialEq, Ord, PartialOrd)]
struct Bag<'a>(&'a str);

#[derive(Debug)]
struct Contains(usize);

struct Edge<'a, N, E> {
    a: &'a N,
    b: &'a E,
}

trait AddEdge<'a, N, E> {
    fn add_edge(e: &Edge<'a, N, E>) -> Option<Edge<'a, N, E>>;
}

impl<'a> AddEdge<'a, Bag<'_>, Contains> for GraphMap<Bag<'_>, Contains, Directed> {
    fn add_edge(e: &Edge<'a, Bag<'_>, Contains>) -> Option<Edge<'a, Bag<'a>, Contains>> {
        Some(*e)
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
        g.add_node(Bag { 0: "yellow" });
        g.add_edge(Bag { 0: "yellow" }, Bag { 0: "red" }, Contains { 0: 3 });
        g.add_edge(Bag { 0: "blue" }, Bag { 0: "red" }, Contains { 0: 2 });
        g.to_file("g.dot").unwrap();
    }
}
