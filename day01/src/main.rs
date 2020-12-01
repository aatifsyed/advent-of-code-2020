use fileutils;

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

fn main() {
    println!("{}", part1("inputs/day01.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1("../inputs/day01.txt"), 158916);
    }
}
