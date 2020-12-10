use aocompute::*;

const DAY: &str = "XX";

fn part1(filepath: &str) {
    let memory: Memory = std::fs::read_to_string(filepath).unwrap().parse().unwrap();
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
}
