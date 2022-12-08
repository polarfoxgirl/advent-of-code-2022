use std::fs;
use regex::Regex;

#[allow(dead_code)]
pub fn solve() {
    let (mut stacks, moves) = read_input(String::from("src/day05/inputs/input.txt"));
    println!("Input count: {} stacks, {} moves", stacks.len(), moves.len());

    let result = get_stack_tops(&stacks);
    println!("Setup: {}", result);

    for instructions in moves {
        apply_move(&mut stacks, &instructions);
    }

    let result = get_stack_tops(&stacks);
    println!("Result: {}", result);
}

fn apply_move(stacks: &mut Vec<Vec<char>>, instructions: &(usize, usize, usize)) -> () {
    let (move_count, source, target) = instructions;
    let mut temp = Vec::with_capacity(*move_count);

    if let Some(source_stack) = stacks.get_mut(*source - 1) {
        for _ in 0..*move_count {
            if let Some(ch) = source_stack.pop() {
                temp.push(ch);
            } else {
                panic!("not enough crates in stack")
            }
        }
    } else {
        panic!("Invalid source stack")
    }

    if let Some(target_stack) = stacks.get_mut(*target - 1) {
        for _ in 0..*move_count {
           target_stack.append(&mut temp);
        }
    } else {
        panic!("Invalid target stack")
    }
}

fn get_stack_tops(stacks: &Vec<Vec<char>>) -> String {
    String::from_iter(stacks.iter().map(|s| s.last().map_or(' ', |ch| *ch)))
}

fn read_input(filename: String) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    let split = text.split_once("\n\n");
    if let Some((crates_text, moves_text)) = split {
        (parse_crates(crates_text), parse_moves(moves_text))
    } else {
        panic!("Unable to separate crates from moves")
    }
}

fn parse_crates(crates_text: &str) -> Vec<Vec<char>> {
    let crate_rows: Vec<Vec<Option<char>>> = crates_text
        .lines()
        .filter_map(parse_crates_line)
        .collect();

    let stack_count = crate_rows.first().map_or_else(|| 0, |r| r.len());

    let mut stacks: Vec<Vec<char>> = (0..stack_count)
        .map(|_| Vec::new())
        .collect();
    for row in crate_rows {
        for (i, maybe_crate) in row.iter().enumerate() {
            if let Some(ch) = maybe_crate {
                if let Some(stack) = stacks.get_mut(i) {
                    stack.push(*ch)
                } else {
                    panic!("Unbalanced crate stack")
                }
            }
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    stacks
}

fn parse_crates_line(line: &str) -> Option<Vec<Option<char>>> {
    if line.starts_with(" 1") {
        return None;
    }

    let crates: Vec<Option<char>> = line
        .chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|chunk| {
            if let Some(first_ch) = chunk.get(0) {
                if '[' == *first_ch {
                    if let Some(crate_ch) = chunk.get(1) {
                        return Some(crate_ch.clone());
                    } else {
                        panic!("Malformed crate format")
                    }
                }

                return None;
            }
            
            panic!("Crates in unexpected format")
        })
        .collect();
    
    Some(crates)
}

fn parse_moves(moves_text: &str) -> Vec<(usize, usize, usize)> {
    let moves_re = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();

    moves_text
        .lines()
        .map(|x| parse_moves_line(&moves_re, x))
        .collect()
}

fn parse_moves_line(re: &Regex, line: &str) -> (usize, usize, usize) {
    let captures = re.captures(line).unwrap();

    (get_usize(&captures, 1), get_usize(&captures, 2), get_usize(&captures, 3))
}

fn get_usize(captures: &regex::Captures, index: usize) -> usize {
    let capture = captures.get(index).unwrap();
    capture.as_str().parse().unwrap()
}