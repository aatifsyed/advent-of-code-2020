use fileutils;

fn part1(filepath: &str) {}

fn part2(filepath: &str) {}

fn main() {
    // println!("part 1: {}", part1("inputs/day03.txt"));
    // println!("part 2: {}", part2("inputs/day03.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_example() {
        let m = fileutils::RectangularCharMap::from_file("../inputs/examples/day03.txt");
        println!("{:?}", m);
        println!("{}", m);
    }
    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1("../inputs/day03.txt"), foo);
    // }
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2("../inputs/day03.txt"), foo);
    // }
}
