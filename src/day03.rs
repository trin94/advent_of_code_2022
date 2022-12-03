use std::ffi::c_uint;

use crate::files;

const LOWER_CASE_OFFSET: u32 = 96;
const UPPER_CASE_OFFSET: u32 = 38;


pub fn priority_of(char: char) -> u32 {
    if char.is_lowercase() {
        char as c_uint - LOWER_CASE_OFFSET
    } else {
        char as c_uint - UPPER_CASE_OFFSET
    }
}

pub fn find_same_char(first: &str, last: &str) -> char {
    let first_chars = first.chars();
    for f in first_chars {
        for l in last.chars() {
            if f == l { return f; }
        }
    }
    panic!("Could not find same char in '{}' and '{}'", first, last)
}

pub fn solve() {
    let file = "resources/day03.txt";
    let lines = files::parse_lines_from(file);

    let sum: u32 = lines.iter()
        .map(|line| {
            let compartment = line.split_at(line.len() / 2);
            let first = compartment.0;
            let last = compartment.1;
            let same_char = find_same_char(first, last);
            priority_of(same_char)
        })
        .sum();

    println!("Sum of the priorities: {}", sum)
}