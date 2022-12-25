use std::fs;

#[allow(dead_code)]
pub fn solve() {
    let numbers = read_input(String::from("src/day25/inputs/input.txt"));
    println!("Input count: {}", numbers.len());

    let result_int: i64 = numbers.iter()
        .map(from_snafu)
        .sum();
    let result: String = to_snafu(&result_int).into_iter().collect();

    println!("Result: {}", result);
}

fn from_snafu(number: &String) -> i64 {
    number.chars()
        .fold(0, |acc, ch| acc * 5 + from_snafu_digit(&ch))
}

fn from_snafu_digit(ch: &char) -> i64 {
    match ch {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        other => panic!("Invalid SNAFU digit: {}", other),
    }
}

fn to_snafu(value: &i64) -> Vec<char> {
    let remainder = *value % 5;
    let (ch, has_carryover) = to_snafu_digit(&remainder);

    let mut next = *value / 5;
    if has_carryover {
        next = next + 1;
    }

    if next > 0 {
        let mut result = to_snafu(&next);
        result.push(ch);
        return result;
    }

    vec![ch]
}

fn to_snafu_digit(value: &i64) -> (char, bool) {
    match value {
        0 => ('0', false),
        1 => ('1', false),
        2 => ('2', false),
        3 => ('=', true),
        4 => ('-', true),
        other => panic!("Invalid value for SNAFU digit: {}", other),
    }
} 

fn read_input(filename: String) -> Vec<String> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.lines()
        .map(String::from)
        .collect()
}