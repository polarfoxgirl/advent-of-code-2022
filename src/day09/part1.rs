use std::{fs, collections::HashSet};
use regex::Regex;
use lazy_static::lazy_static;

enum Cmd {
    Right,
    Left,
    Up,
    Down,
}

#[allow(dead_code)]
pub fn solve() {
    let commands = read_input(String::from("src/day09/inputs/input.txt"));
    println!("Input count: {}", commands.len());

    let mut visited = HashSet::new();
    visited.insert((0, 0));

    let ((h_x, h_y), (t_x, t_y)) = commands.iter()
        .fold(((0, 0), (0, 0)), |acc, cmd| apply_move(&mut visited, cmd, acc));

    println!("Result: visited {} to get to (({}, {}), ({}, {}))", visited.len(), h_x, h_y, t_x, t_y)
}

fn apply_move(visited: &mut HashSet<(i32, i32)>, instruction: &(Cmd, u32), head_tail: ((i32, i32), (i32, i32))) -> ((i32, i32), (i32, i32)) {
    let (cmd, count) = instruction;

    (0..*count).fold(head_tail, |acc, _| apply_step(visited, cmd,  acc))
}

fn apply_step(visited: &mut HashSet<(i32, i32)>, cmd: &Cmd, head_tail: ((i32, i32), (i32, i32))) -> ((i32, i32), (i32, i32)) {
    let ((h_x, h_y), tail) = head_tail;

    let new_head = match *cmd {
        Cmd::Right => (h_x + 1, h_y),
        Cmd::Left => (h_x - 1, h_y),
        Cmd::Up => (h_x, h_y + 1),
        Cmd::Down => (h_x, h_y - 1),
    };

    let new_tail = move_tail(&new_head, &tail);
    visited.insert(new_tail);
    (new_head, new_tail)
}

fn move_tail(new_head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    let (h_x, h_y) = new_head;
    let (t_x, t_y) = tail;

    if (*h_x - *t_x).abs() > 2 || (*h_y - *t_y).abs() > 2 {
        panic!("Invalid position (({}, {}), ({}, {}))", h_x, h_y, t_x, t_y)
    }

    // Same column
    if *h_x == *t_x {
        if *h_y > *t_y + 1 {
            return (*t_x, *t_y + 1);
        }
        if *h_y < *t_y - 1 {
            return (*t_x, *t_y - 1);
        }
    }

    // Same row
    if *h_y == *t_y {
        if *h_x > *t_x + 1 {
            return (*t_x + 1, *t_y);
        }
        if *h_x < *t_x - 1 {
            return (*t_x - 1, *t_y);
        }
    }

    // Diagonals
    if *h_x > *t_x {
        if *h_y > *t_y {
            if *h_x > *t_x + 1 || *h_y > *t_y + 1 {
                return (*t_x + 1, *t_y + 1);
            }
        }
        if *h_y < *t_y {
            if *h_x > *t_x + 1 || *h_y < *t_y - 1 {
                return (*t_x + 1, *t_y - 1);
            }
        }
    }
    if *h_x < *t_x {
        if *h_y > *t_y {
            if *h_x < *t_x - 1 || *h_y > *t_y + 1 {
                return (*t_x - 1, *t_y + 1);
            }
        }
        if *h_y < *t_y {
            if *h_x < *t_x - 1 || *h_y < *t_y - 1 {
                return (*t_x - 1, *t_y - 1);
            }
        }
    }

    return *tail;
}


fn read_input(filename: String) -> Vec<(Cmd, u32)> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.lines().map(read_line).collect()
}

fn read_line(line: &str) -> (Cmd, u32) {
    lazy_static! {
        static ref CMD_RE: Regex = Regex::new(r"([RLUD]) (\d+)").unwrap();
    }

    if let Some(captures) = CMD_RE.captures(line) {
        let count: u32 = captures.get(2).unwrap().as_str().parse().unwrap();

        let cmd = match captures.get(1).unwrap().as_str() {
            "R" => Cmd::Right,
            "L" => Cmd::Left,
            "U" => Cmd::Up,
            "D" => Cmd::Down,
            _ => panic!("Unrecognized command")
        };

        return (cmd, count);
    }

    panic!("Unable to parse line");
}