use std::{fs, collections::HashSet};

#[allow(dead_code)]
pub fn solve() {
    let datastream = read_input(String::from("src/day06/inputs/input.txt"));
    println!("Input count: {}", datastream.len());

    let result = find_marker(&datastream);
    println!("Result: {}", result)
}

fn find_marker(datastream: &String) -> u16 {
    let mut count: u16 = 0;

    let mut buffer: [Option<char>; 4] = [None; 4];
    let mut pos: usize = 0;

    for new_ch in datastream.chars() {
        count = count + 1;
        buffer[pos] = Some(new_ch);
        pos = (pos + 1) % 4;


        let unique_chs = buffer.iter().filter_map(|x| *x).collect::<HashSet<char>>().len();

        if 4 == unique_chs {
            return count;
        }        
    }

    panic!("Unable to find marker")
}


fn read_input(filename: String) -> String {
    println!("Reading file {}", filename);
    fs::read_to_string(filename).expect("Failed to read input")
}