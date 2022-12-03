use std::{fs, collections::HashSet};

#[allow(dead_code)]
pub fn solve() {
    let elf_rucksacks = read_input(String::from("src/day03/inputs/input.txt"));
    println!("Input count: {}", elf_rucksacks.len());

    if elf_rucksacks.len() % 3 != 0 {
        panic!("Number of elves is not divisible by 3")
    }

    let result: u32 = elf_rucksacks.chunks_exact(3).map(find_badge).sum();
    println!("Result: {}", result)
}

fn find_badge(rucksacks: &[HashSet<u8>]) -> u32 {
    let first_overlap: HashSet<u8> = rucksacks[0].intersection(&rucksacks[1]).map(|x| *x).collect();
    let overlap : Vec<&u8> = rucksacks[2].intersection(&first_overlap).collect();

    if overlap.len() != 1 {
        panic!("Can't find singular badge")
    }
    if let Some(&value) = overlap.get(0) {
        (*value).into()
    } else {
        panic!("Unreachable")
    }
}


fn read_input(filename: String) -> Vec<HashSet<u8>> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.split("\n").map(|line| line.chars().map(get_priority).collect()).collect()
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