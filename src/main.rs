use std::fs;
mod day02;
use day02::{p1::solve, p2::solve as solve2};

fn main() {
    let base = String::from("src/day02/");
    let data =
        fs::read_to_string(base + "input.txt").expect("Something went wrong reading the file");

    let contents: Vec<&str> = data.split_terminator("\n").collect();

    match solve2(contents) {
        Ok(answer) => println!("Answer: {}", answer),
        Err(problem) => println!("We got a problem: {}", problem),
    }
}
