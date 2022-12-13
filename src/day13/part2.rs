use std::{fs, cmp::Ordering};

enum ListItem {
    Int(u32),
    List(Vec<ListItem>),
}

#[allow(dead_code)]
pub fn solve() {
    let mut packets = read_input(String::from("src/day13/inputs/input.txt"));

    packets.push(ListItem::List(vec![ListItem::Int(2)]));
    packets.push(ListItem::List(vec![ListItem::Int(6)]));

    packets.sort_by(|x, y| {
        match check_order(x, y) {
            None => Ordering::Equal,
            Some(is_less) => {
                if is_less { 
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        }
    });

    let first_divider = ListItem::List(vec![ListItem::Int(2)]);
    let second_divider = ListItem::List(vec![ListItem::Int(6)]);

    let result = packets.iter()
        .enumerate()
        .filter(|(_, item)| check_order(&first_divider, *item).is_none() ||  check_order(&second_divider, *item).is_none())
        .map(|(index, _)| index + 1)
        .reduce(|acc, x| acc * x)
        .unwrap();
    
    println!("Result: {}", result);
}

#[allow(dead_code)]
fn check_order(item1: &ListItem, item2: &ListItem) -> Option<bool> {
    match item1 {
        ListItem::Int(int1) => {
            match item2 {
                ListItem::Int(int2) => {
                    if *int1 == *int2 {
                        return None;
                    } else {
                        return Some(*int1 < *int2); 
                    }
                }
                ListItem::List(list2) => check_list_order(&vec![ListItem::Int(*int1)], list2),
            }
        },
        ListItem::List(list1) => {
            match item2 {
                ListItem::Int(int2) => check_list_order(list1, &vec![ListItem::Int(*int2)]),
                ListItem::List(list2) => check_list_order(list1, list2),
            }
        }
    }
}

fn check_list_order(list1: &Vec<ListItem>, list2: &Vec<ListItem>) -> Option<bool> {
    for (i, item1) in list1.iter().enumerate() {
        match list2.get(i) {
            None => return Some(false),
            Some(item2) => {
                if let Some(result) = check_order(item1, item2) {
                    return Some(result);
                }
            }
        }
    }
    
    if list1.len() < list2.len() {
        return Some(true);
    }

    return None;
}

fn read_input(filename: String) -> Vec<ListItem> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.lines()
        .filter(|s| !s.is_empty())
        .map(read_line)
        .collect()
}

fn read_line(line: &str) -> ListItem {
    let chars = line.chars().collect();
    let (result, final_pos) = read_item(&chars, 0);

    if final_pos != line.len() {
        panic!("Failed to read line: {}", line);
    }

    result
}

fn read_item(chars: &Vec<char>, pos: usize) -> (ListItem, usize) {
    match chars.get(pos) {
        None => panic!("Unexpected end of line"),
        Some(ch) => {
            if *ch == '[' {
                let (list, pos) = read_list(chars, pos + 1);
                return (ListItem::List(list), pos);
            }

            if ch.is_numeric() {
                let (int, pos) = read_int(chars, pos);
                return (ListItem::Int(int), pos);
            }
            
            panic!("Unexpected character in item: {}", ch);
        }
    }
}

fn read_list(chars: &Vec<char>, start_pos: usize) -> (Vec<ListItem>, usize) {
    let mut pos = start_pos;
    let mut list = Vec::new();
    loop {        
        match chars.get(pos) {
            None => panic!("Unexpected end of line"),
            Some(ch) => {
                match ch {
                    ',' => pos = pos + 1,
                    ']' => return (list, pos + 1),
                    _ => {
                        let (item, new_pos) = read_item(chars, pos);
                        list.push(item);
                        pos = new_pos;
                    },
                }
            }
        }
    }
}

fn read_int(chars: &Vec<char>, start_pos: usize) -> (u32, usize) {
    let mut pos = start_pos;
    let mut val = 0;
    loop {
        match chars.get(pos) {
            None => panic!("Unexpected end of line"),
            Some(ch) => {
                if let Some(x) = ch.to_digit(10) {
                    val = val * 10 + x;
                    pos = pos + 1;
                } else {
                    return (val, pos);
                }
            }
        }
    }
}