use std::fs;

#[allow(dead_code)]
pub fn solve() {
    let elf_items = read_input(String::from("src/day01/inputs/input.txt"));
    println!("Input count: {}", elf_items.len());

    let mut elf_loads: Vec<i32> = elf_items.iter()
        .map(|items| items.iter().sum())
        .collect();

    elf_loads.sort();

    let elf_count = elf_loads.len();
    if elf_count < 3 {
        println!("Not enough elfs!")
    } else {
        let result: i32 = elf_loads[elf_count - 3..elf_count].iter().sum();
        println!("Result: {}", result)
    }
}


fn read_input(filename: String) -> Vec<Vec<i32>> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.split("\n\n").map(|e| e.split("\n").map(|x| x.parse().unwrap()).collect()).collect()
}