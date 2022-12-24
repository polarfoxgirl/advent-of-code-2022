use std::fs;

#[allow(dead_code)]
pub fn solve() {
    let mut list = read_input(String::from("src/day20/inputs/input.txt"));
    println!("Input count: {}", list.len());

    for _ in 0..10 {
        mix_list(&mut list);
    }

    let (x, y, z) = get_coordinates(&list);
    println!("Found coordinates ({}, {}, {}) with sum of {}", x, y, z, x + y + z);
}

fn mix_list(list: &mut Vec<(usize, i64)>) -> () {
    let length = list.len();
    let int_length = i64::try_from(length).unwrap();

    let mut next = 0;
    let mut pos = 0;
    while next < length {
        let item_pos = find_item_pos(list, &next, &pos);
        pos = item_pos.1;
        let item = item_pos.0;

        let target_pos = calc_target_pos(item, &pos, &int_length);
        move_item(list, &pos,  &target_pos);

        next = next + 1;
    }
}

fn find_item_pos<'a>(list: &'a Vec<(usize, i64)>, next: &usize, start_pos: &usize) -> (&'a i64, usize) {
    let mut pos = *start_pos;
    loop {
        if let Some((maybe_next, item)) = list.get(pos) {
            if *maybe_next == *next {
                return (item, pos);
            }
        }

        pos = (pos + 1) % list.len();
    }
}

fn calc_target_pos(item: &i64, pos: &usize, int_length: &i64) -> usize {
    let mut int_value = (i64::try_from(*pos).unwrap() + *item) % (*int_length - 1);

    if int_value < 0 {
        int_value = int_value + *int_length - 1;
        if int_value < 0 {
            panic!("Value still negative {}", int_value);
        }
    }
    
    match usize::try_from(int_value) {
        Ok(value) => value,
        Err(_) => panic!("Unable to convert value {}", int_value),
    }
}

fn move_item(list: &mut Vec<(usize, i64)>, pos: &usize, target_pos: &usize) -> () {
    if *pos == *target_pos {
        return;
    }

    let value = list.remove(*pos);
    if *target_pos == 0 {
        list.push(value);
    } else {
        list.insert(*target_pos, value);
    }
}

fn get_coordinates(list: &Vec<(usize, i64)>) -> (i64, i64, i64) {
    let zero_pos = find_zero_pos(list);

    let x = list.get((zero_pos + 1000) % list.len()).unwrap().1;
    let y = list.get((zero_pos + 2000) % list.len()).unwrap().1;
    let z = list.get((zero_pos + 3000) % list.len()).unwrap().1;
    (x, y, z)
}

fn find_zero_pos(list: &Vec<(usize, i64)>) -> usize {
    let mut pos = 0;
    loop {
        if let Some((_, item)) = list.get(pos) {
            if *item == 0 {
                return pos;
            }
        }

        pos = (pos + 1) % list.len();
    }
}

fn read_input(filename: String) -> Vec<(usize, i64)> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.lines()
        .map(|s| s.parse().unwrap())
        .map(|v: i64| v * 811589153)
        .enumerate()
        .collect()
}