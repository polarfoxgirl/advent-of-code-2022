use std::{fs, collections::{VecDeque, HashMap, hash_map::Entry}};

#[allow(dead_code)]
pub fn solve() {
    let (elevations, end, max_y) = read_input(String::from("src/day12/inputs/input.txt"));
    println!("Input count: {} with ({}, {})", elevations.len(), end.0, end.1);

    let max_x = elevations.len() / max_y;
    println!("Max values are {} and {}", max_x, max_y);

    let mut distances = HashMap::new();
    distances.insert(end, 0);
    let mut queue = VecDeque::from([end]);

    loop {
        if queue.is_empty() {
            break;
        }

        let position = queue.pop_front().unwrap();
        let current_elevation =  elevations.get(&position).unwrap();
        let current_distance = *distances.get(&position).unwrap();

        for neighbor in get_neighbors(&max_x, &max_y, &position) {
            if let Entry::Vacant(ve) = distances.entry(neighbor) {
                let elevation = elevations.get(&neighbor).unwrap();

                if *current_elevation <= *elevation || *current_elevation == *elevation + 1 {
                    ve.insert(current_distance + 1);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    let result = elevations.iter()
        .filter(|(_, v)| **v == 0)
        .filter_map(|(k, _)| distances.get(k))
        .min()
        .unwrap();
    println!("Shortest path is {}", result);
}

fn get_neighbors(max_x: &usize, max_y: &usize, position: &(usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = position;
    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    if *x > 0 {
        neighbors.push((*x - 1, *y));
    }

    if *x < *max_x - 1 {
        neighbors.push((*x + 1, *y));
    }

    if *y > 0 {
        neighbors.push((*x, *y - 1));
    }

    if *y < *max_y - 1 {
        neighbors.push((*x, *y + 1));
    }

    neighbors
}

fn read_input(filename: String) -> (HashMap<(usize, usize), u32>, (usize, usize), usize) {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    let mut elevations: HashMap<(usize, usize), u32> = HashMap::new();
    let mut end = None;
    let mut max_y = None;

    for (i, line) in text.lines().enumerate() {
        if max_y.is_none() {
            max_y = Some(line.len());
        }

        for (j, ch) in line.chars().enumerate() {
            if ch == 'S' {
                elevations.insert((i, j), 0);
            } else if ch == 'E' {
                elevations.insert((i, j), 25);
                end = Some((i, j))
            } else {
                elevations.insert((i, j), ch.to_digit(36).unwrap() - 10);
            }
        }
    }

    (elevations, end.unwrap(), max_y.unwrap())
}