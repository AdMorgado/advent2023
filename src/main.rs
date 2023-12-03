use std::{fs::File, io::{BufReader, BufRead}};

mod d01;

fn main() {

    // Day 1
    let file = File::open("inputs/d01").expect("Could not open file!");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines()
        .map(|line| line.expect("Failure to read line!"))
        .collect();
    
    d01::part_one(&lines);
    d01::part_two(&lines);

    // Day 2

}
