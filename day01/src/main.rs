use fileutils;

fn main() {
    let nums = fileutils::numbers_from_file("inputs/day01.txt");
    println!("{:?}", nums);
}
