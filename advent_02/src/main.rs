use std::fs;

fn main() {
    let file_path = "~/projects/advent2023-learning/advent_02/test.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.split('\n');
    println!("Hello, world!");
}
