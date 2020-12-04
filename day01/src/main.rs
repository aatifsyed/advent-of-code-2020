// Accept filename due to https://github.com/rust-lang/cargo/issues/8340
fn part1(filepath: &str) -> isize {
    let nums = fileutils::numbers_from_file(filepath);

    // https://doc.rust-lang.org/rust-by-example/flow_control/loop/nested.html
    for a in &nums {
        for b in &nums {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    panic!("Couldn't find a match!")
}

fn part2(filepath: &str) -> isize {
    let nums = fileutils::numbers_from_file(filepath);

    // https://doc.rust-lang.org/rust-by-example/flow_control/loop/nested.html
    for a in &nums {
        for b in &nums {
            for c in &nums {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    panic!("Couldn't find a match!")
}

fn main() {
    println!("part 1: {}", part1("inputs/day01.txt"));
    println!("part 2: {}", part2("inputs/day01.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1("../inputs/day01.txt"), 158916);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("../inputs/day01.txt"), 165795564);
    }
}
