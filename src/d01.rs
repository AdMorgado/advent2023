use std::cmp::Ordering;

use derive_more::*;


const RADIX: u32 = 10;

pub fn part_one(lines: &Vec<String>) {
    let mut total = 0;

    lines.iter()
        .for_each(|line| {
            let numbers: Vec<u32> = line.chars()
                .filter(|char| char.is_digit(RADIX))
                .map(|char| char.to_digit(RADIX).expect("Not a digit!"))
                .collect();

            assert!(!numbers.is_empty());
            let number = numbers.first().unwrap() * 10 + numbers.last().unwrap();
            total += number;
        });

    println!("{}", total);
}

struct PosEntry {
    number: u32, 
    pos: usize,
}


fn line_to_number(str: &String) -> u32 {
    let numbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let results = numbers.iter()
        .enumerate()
        .map(|(substrIdx, &substring)| {
            let mut stringEntries = str.match_indices(substring)
                .map(move |(pos, _)| PosEntry{number:(substrIdx+1) as u32, pos})
                .collect::<Vec<PosEntry>>();

            let digitEntries = str.chars()
                .enumerate()
                .filter_map(|(pos, char)| {
                    let number = char.to_digit(RADIX)?;
                    Some(PosEntry{number, pos})
                });

            stringEntries.extend(digitEntries);
            stringEntries
        });
    
    let mut nums: Vec<PosEntry> = results
        .flatten()
        .collect();

    nums.sort_by_key(|entry| entry.pos);
    
    assert!(!nums.is_empty());
    let first = nums.first().unwrap().number;
    let last = nums.last().unwrap().number;
    (first * 10) + last
}

pub fn part_two(lines: &Vec<String>) {
    let result: u32 = lines.iter()
        .map(line_to_number)
        .sum();

    println!("{}", result);
}


