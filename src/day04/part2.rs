use std::fs;
use regex::{Regex, Match};

#[allow(dead_code)]
pub fn solve() {
    let assignment_pairs = read_input(String::from("src/day04/inputs/input.txt"));
    println!("Input count: {}", assignment_pairs.len());

    let result_count = assignment_pairs.iter()
        .filter(|(fr, sr)| has_overlap(fr, sr))
        .count();
    println!("Result: {}", result_count)
}

fn has_overlap(first_range: &(u32, u32), second_range: &(u32, u32)) -> bool {
    let (first_start, first_end) = first_range;
    let (second_start, second_end) = second_range;

    if *first_end < *second_start {
        return false;
    }

    if *first_start > *second_end {
        return false;
    }

    return true;
}

fn read_input(filename: String) -> Vec<((u32, u32), (u32, u32))> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    let re = Regex::new(r"(\d+)\-(\d+),(\d+)\-(\d+)").unwrap();
    text.split("\n").map(|x| parse_line(&re, x)).collect()
}

fn parse_line(re: &Regex, line: &str) -> ((u32, u32), (u32, u32)) {
    let captures = re.captures(line).unwrap();

    let first_range: (u32, u32) = (parse_i32(&captures.get(1).unwrap()), parse_i32(&captures.get(2).unwrap()));
    let second_range: (u32, u32) = (parse_i32(&captures.get(3).unwrap()), parse_i32(&captures.get(4).unwrap()));

    (first_range, second_range)
}

fn parse_i32(capture: &Match) -> u32 {
    capture.as_str().parse().unwrap()
}