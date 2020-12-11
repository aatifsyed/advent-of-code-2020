use fileutils;
use itertools::Itertools;
use std::collections::HashSet;

const DAY: &str = "09";

fn part1(filepath: &str) -> isize {
    let input = fileutils::numbers_from_file(filepath);
    let width = 25;
    for i in width..input.len() {
        let slice = &input[i - width..i];
        let candidate = input[i];
        let mults = valid_sums(slice, 2);
        if !mults.contains(&candidate) {
            return candidate;
        }
    }
    panic!("Not found!");
}

fn possible_slice_indices<T>(vec: &Vec<T>) -> Vec<(usize, usize)> {
    let indices = 0..vec.len();
    indices
        .into_iter()
        .combinations(2)
        .map(|pair| {
            let tup = (*pair.iter().min().unwrap(), *pair.iter().max().unwrap());
            tup
        })
        .collect()
}

fn part2(filepath: &str) -> isize {
    let input = fileutils::numbers_from_file(filepath);
    let target = part1(filepath);
    for (from, to) in possible_slice_indices(&input) {
        // println!("trying {} {}", from, to);
        let slice = input[from..to].iter();
        if target == slice.clone().sum() {
            return slice.clone().min().unwrap() + slice.max().unwrap();
        }
    }
    panic!("Not found!")
}

fn main() {
    let filepath = format!("inputs/day{}.txt", DAY);
    println!("part 1: {:?}", part1(&filepath));
    println!("part 2: {:?}", part2(&filepath));
}

fn valid_sums(over: &[isize], len: usize) -> HashSet<isize> {
    let combos = over.into_iter().combinations(len);
    combos.map(|vec| vec.into_iter().sum()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(&format!("../inputs/day{}.txt", DAY)), 26134589);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&format!("../inputs/day{}.txt", DAY)), 3535124);
    }
    #[test]
    fn test_example_1() {
        let filepath = "../inputs/examples/day09.txt";
        let input = fileutils::numbers_from_file(filepath);
        let width = 5;
        let mut result = None;
        for i in width..input.len() {
            let slice = &input[i - width..i];
            let candidate = input[i];
            let mults = valid_sums(slice, 2);
            // print!("{:?} @ {:?}: {:?}", candidate, i, slice);
            if !mults.contains(&candidate) {
                result = Some(candidate)
            }
            // println!(" ({:?})", result);
        }
        assert_eq!(result, Some(127))
    }
    #[test]
    fn test_combinations() {
        let mults = valid_sums(&vec![1, 2, 3][..], 2);
        assert!(mults.contains(&2) && mults.contains(&3) && mults.contains(&6) && mults.len() == 3)
    }
    #[test]
    fn test_possible_slice_indices() {
        println!("{:?}", possible_slice_indices(&vec![1, 2, 3]))
    }
}
