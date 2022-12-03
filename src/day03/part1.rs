use std::{fs, collections::HashSet};

#[allow(dead_code)]
pub fn solve() {
    let elf_rucksacks = read_input(String::from("src/day03/inputs/input.txt"));
    println!("Input count: {}", elf_rucksacks.len());

    let result: u32 = elf_rucksacks
        .iter()
        .map(|(f, s)| find_outlier(f, s))
        .sum();

    println!("Result: {}", result)
}

fn find_outlier(first_comp: &HashSet<u8>, second_comp: &HashSet<u8>) -> u32 {
    let intersection: Vec<&u8> = first_comp.intersection(second_comp).collect();

    if intersection.len() != 1 {
        panic!("Can't find a singular outlier")
    }

    (*intersection[0]).into()
}


fn read_input(filename: String) -> Vec<(HashSet<u8>, HashSet<u8>)> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.split("\n").map(parse_line).collect()
}

fn parse_line(line: &str) -> (HashSet<u8>, HashSet<u8>) {
    if line.len() % 2 != 0 {
        panic!("Can't split rucksack into equal compartments")
    }

    let rucksack_len = line.len() / 2;

    let first_comp = line[0..rucksack_len].chars().map(get_priority).collect();
    let second_comp = line[rucksack_len..line.len()].chars().map(get_priority).collect();

    (first_comp, second_comp)
}

fn get_priority(ch: char) -> u8 {
    if ch.is_ascii_lowercase() {
        1 + (ch as u8) - ('a' as u8)
    } else if ch.is_ascii_uppercase() {
        27 + (ch as u8) - ('A' as u8)
    } else {
        panic!("Unsupported char value in rucksack")
    }
}