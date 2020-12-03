use fileutils;
use std::collections::HashMap;

struct Direction {
    right: usize,
    down: usize,
}

#[derive(Debug)]
struct Position {
    h: usize,
    w: usize,
}

impl Position {
    fn update(&mut self, direction: &Direction, grid: &fileutils::RectangularCharGrid) {
        self.h += direction.down;
        self.w += direction.right;
        // Infinite, repeated width
        if self.w >= grid.width {
            self.w -= grid.width;
        }
    }
}

fn count_encounters(
    grid: fileutils::RectangularCharGrid,
    direction: Direction,
) -> HashMap<char, usize> {
    let mut seen = HashMap::new();
    let mut pos = Position { h: 0, w: 0 };
    pos.update(&direction, &grid);
    while pos.h < grid.height {
        let index = pos.h * grid.width + pos.w;
        let current_char = grid.buffer[index];
        *seen.entry(current_char).or_insert(0) += 1;
        println!("counted {} at {:?}", current_char, pos);
        pos.update(&direction, &grid);
    }
    seen
}

fn part1(filepath: &str) -> usize {
    let grid = fileutils::RectangularCharGrid::from_file(filepath);
    *count_encounters(grid, Direction { right: 3, down: 1 })
        .get(&'#')
        .expect("No Trees!")
}

fn part2(filepath: &str) {}

fn main() {
    println!("part 1: {}", part1("inputs/day03.txt"));
    // println!("part 2: {}", part2("inputs/day03.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_example() {
        let m = fileutils::RectangularCharGrid::from_file("../inputs/examples/day03.txt");
        println!("{:?}", m);
        println!("{}", m);
    }

    #[test]
    fn traverse() {
        let m = fileutils::RectangularCharGrid::from_file("../inputs/examples/day03.txt");
        println!("{:?}", count_encounters(m, Direction { right: 3, down: 1 }));
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
