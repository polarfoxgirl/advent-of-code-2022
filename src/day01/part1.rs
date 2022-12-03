use std::fs;

#[allow(dead_code)]
pub fn solve() {
    let elf_items = read_input(String::from("src/day01/inputs/input.txt"));
    println!("Input count: {}", elf_items.len());

    let max_sum: i32 = elf_items.iter().map(|items| items.iter().sum()).max().unwrap_or_default();
    println!("Result: {}", max_sum)
}


fn read_input(filename: String) -> Vec<Vec<i32>> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.split("\n\n").map(|e| e.split("\n").map(|x| x.parse().unwrap()).collect()).collect()
}