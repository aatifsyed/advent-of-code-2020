use itertools::Itertools;
use petgraph::{dot::Dot, graphmap::GraphMap, Directed, IntoWeightedEdge};
use regex::Regex;
use std::{fs, io, path};

#[macro_use]
extern crate lazy_static;

/// GraphMap requires Copy.
/// This can't be done for Strings.
/// Use slices, and read in the file as a String.
/// Hold references to parts of that string for the lifetime of the program.
#[derive(Eq, Hash, Debug, Copy, Clone, PartialEq, Ord, PartialOrd)]
struct Bag<'b>(&'b str);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Contains(usize);

#[derive(Debug, Eq, PartialEq)]
struct Edge<'e> {
    from: Bag<'e>,
    to: Bag<'e>,
    label: Contains,
}

impl<'e> IntoWeightedEdge<Contains> for Edge<'e> {
    type NodeId = Bag<'e>;
    fn into_weighted_edge(self) -> (Self::NodeId, Self::NodeId, Contains) {
        (self.from, self.to, self.label)
    }
}

/// Each line is parsed as an Edges.
#[derive(Debug, Eq, PartialEq)]
struct Edges<'s> {
    from: Bag<'s>,
    tos: Vec<(Bag<'s>, Contains)>,
}

impl<'s> Iterator for Edges<'s> {
    type Item = Edge<'s>;
    fn next(&mut self) -> Option<Edge<'s>> {
        match self.tos.pop() {
            Some((to, label)) => Some(Edge {
                from: self.from,
                to,
                label,
            }),
            None => None,
        }
    }
}

lazy_static! {
    static ref BAG_REGEX: Regex = Regex::new(r"((?P<number>\d) )?(?P<description>\w+ \w+) bag")
        .expect("Couldn't compile bag regex");
}

impl<'s> From<&'s str> for Edges<'s> {
    fn from(line: &'s str) -> Self {
        let mut caps = BAG_REGEX.captures_iter(line);

        // Pull out the first capture group. It is the A node
        let (head, tail) = (caps.next().expect("No matches!"), caps);

        let head = head.name("description").expect("Invalid head!").as_str();
        let head = Bag(head);

        // Pull out our vector of B nodes and weights, if they exist.
        let tail = tail
            .filter_map(|cap| {
                let description = cap.name("description").expect("No description!").as_str();
                // Only found this bug later on...
                // Let's hope there are no orphaned nodes!
                match description {
                    "no other" => None,
                    _ => {
                        let number = cap
                            .name("number")
                            .expect("No number!")
                            .as_str()
                            .parse()
                            .expect("Couldn't parse number!");
                        Some((Bag(description), Contains(number)))
                    }
                }
            })
            .collect();
        Edges {
            from: head,
            tos: tail,
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

const DAY: &str = "07";

fn part1(filepath: &str) {
    let file = fs::read_to_string(filepath).unwrap();
    for (enumeration, edges) in file.lines().map(|line| Edges::from(line)).enumerate() {
        let g: GraphMap<_, _, Directed> = GraphMap::from_edges(edges);
        g.to_file(format!("{}.dot", enumeration)).unwrap();
    }
}

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
    fn parse_line_into_edges() {
        let s = String::from("light red bags contain 1 bright white bag, 2 muted yellow bags.");
        let e = Edges::from(&s[..]);
        assert_eq!(
            e,
            Edges {
                from: Bag("light red"),
                tos: vec![
                    (Bag("bright white"), Contains(1)),
                    (Bag("muted yellow"), Contains(2))
                ]
            }
        );
    }
    #[test]
    fn iter_edges() {
        let s = String::from("light red bags contain 1 bright white bag, 2 muted yellow bags.");
        let mut e = Edges::from(&s[..]);
        assert_eq!(
            e.next(),
            Some(Edge {
                from: Bag("light red"),
                to: Bag("muted yellow"),
                label: Contains(2)
            })
        );
        assert_eq!(
            e.next(),
            Some(Edge {
                from: Bag("light red"),
                to: Bag("bright white"),
                label: Contains(1)
            })
        );
        assert_eq!(e.next(), None);
    }
    #[test]
    fn parse_line_into_graph() {
        let s = String::from("light red bags contain 1 bright white bag, 2 muted yellow bags.");
        let e = Edges::from(&s[..]);
        let g: GraphMap<_, _, Directed> = GraphMap::from_edges(e);
        assert_eq!(
            g.edge_weight(Bag("light red"), Bag("muted yellow")),
            Some(&Contains(2))
        );
        assert_eq!(
            g.edge_weight(Bag("light red"), Bag("bright white")),
            Some(&Contains(1))
        );
        assert_eq!(g.node_count(), 3);
        assert_eq!(g.edge_count(), 2);
    }
}
