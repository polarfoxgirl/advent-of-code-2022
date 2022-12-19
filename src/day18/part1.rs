use std::{fs, collections::HashSet};

#[allow(dead_code)]
pub fn solve() {
    let blocks = read_input(String::from("src/day18/inputs/input.txt"));
    println!("Input count: {}", blocks.len());

    let result: usize = blocks.iter()
        .map(|b| {
            get_neighbors(&b).iter()
                .filter(|n| !blocks.contains(*n))
                .count()
        })
        .sum();

    println!("Result: {}", result);
}

fn get_neighbors(block: &(i16,i16,i16)) -> [(i16,i16,i16); 6] {
    let (x, y, z) = block;
    [
        (*x - 1, *y, *z),
        (*x + 1, *y, *z),
        (*x, *y - 1, *z),
        (*x, *y + 1, *z),
        (*x, *y, *z - 1),
        (*x, *y, *z + 1),
    ]
}

fn read_input(filename: String) -> HashSet<(i16,i16,i16)> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.lines()
        .map(read_line)
        .collect()
}

fn read_line(line: &str) -> (i16,i16,i16) {
    if let Some((first, tail)) = line.split_once(",") {
        if let Some((second, third)) = tail.split_once(",") {
            return (first.parse().unwrap(), second.parse().unwrap(), third.parse().unwrap());
        }
    }

    panic!("Unable to parse line");
}