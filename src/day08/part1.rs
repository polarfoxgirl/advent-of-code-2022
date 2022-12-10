use std::{fs, collections::HashSet};

#[allow(dead_code)]
pub fn solve() {
    let tree_rows = read_input(String::from("src/day08/inputs/input.txt"));
    println!("Input count: {}", tree_rows.len());

    let grid_width = tree_rows.get(0).map_or(0, Vec::len);

    let mut visible = HashSet::new();

    // Visibile from the left
    for (i, row) in tree_rows.iter().enumerate() {
        let mut previous_max = 0;
        for (j, tree) in row.iter().enumerate() {
            if j == 0 || *tree > previous_max {
                visible.insert((i, j));
                previous_max = *tree;
            }
        }
    }

    // Visibile from the right
    for (i, row) in tree_rows.iter().enumerate() {
        let mut previous_max = 0;
        for (j, tree) in row.iter().enumerate().rev() {
            if j == grid_width - 1 || *tree > previous_max {
                visible.insert((i, j));
                previous_max = *tree;
            }
        }
    }

    // Visible from the top
    for j in 0..grid_width {
        let mut previous_max = 0;
        for (i, row) in tree_rows.iter().enumerate() {
            if let Some(tree) = row.get(j) {
                if i == 0 || *tree > previous_max {
                    visible.insert((i, j));
                    previous_max = *tree;
                }
            } else {
                panic!("Index out of bounds: {}", j)
            }
        }
    }

    // Visible from the bottom
    for j in 0..grid_width {
        let mut previous_max = 0;
        for (i, row) in tree_rows.iter().enumerate().rev() {
            if let Some(tree) = row.get(j) {
                if i == grid_width - 1 || *tree > previous_max {
                    visible.insert((i, j));
                    previous_max = *tree;
                }
            } else {
                panic!("Index out of bounds: {}", j)
            }
        }
    }

    println!("Result: {}", visible.len())
}


fn read_input(filename: String) -> Vec<Vec<u32>> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.lines().map(read_line).collect()
}

fn read_line(line: &str) -> Vec<u32> {
    line.chars().filter_map(|ch| ch.to_digit(10)).collect()
}