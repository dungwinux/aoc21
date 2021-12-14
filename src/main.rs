use std::fs;
mod day14;
use day14::{p1, p2};

fn run(contents: Vec<&str>, solver: &dyn Fn(Vec<&str>) -> Result<i64, Box<dyn std::error::Error>>) {
    match solver(contents) {
        Ok(answer) => println!("Answer: {}", answer),
        Err(problem) => println!("We got a problem: {}", problem),
    }
}

fn main() {
    let base = String::from("src/day14/");
    let data =
        fs::read_to_string(base + "input.txt").expect("Something went wrong reading the file");

    let contents: Vec<&str> = data.split_terminator("\n").collect();
    run(contents.clone(), &p1::solve);
    run(contents, &p2::solve);
}
