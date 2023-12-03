use std::{fs::File, io::{BufReader, Read}};


fn get_input(filename: &String) -> String {
    let file = File::open(filename).expect("Could not open file!");
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    reader.read_to_string(&mut input).expect("Could not read file!");
    input
}

pub trait Day {
    fn part_one(input: &String);
    fn part_two(input: &String);

    fn perform(&self, filename: &String) {
        // Get input
        let input = get_input(filename);
        Self::part_one(&input);
        Self::part_two(&input);
    }
}

