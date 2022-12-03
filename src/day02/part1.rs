use std::fs;
use regex::Regex;

#[allow(dead_code)]
pub fn solve() {
    let rounds = read_input(String::from("src/day02/inputs/input.txt"));
    println!("Input count: {}", rounds.len());

    let total_score: i32 = rounds.iter()
        .map(|(o, r)| calc_score(o, r))
        .sum();

    println!("Result: {}", total_score)
}

#[derive(PartialEq)]
enum RpsAction {
    Rock,
    Paper,
    Scissors,
}

fn calc_score(opponent: &RpsAction, response: &RpsAction) -> i32 {
    let action_score = match response {
        RpsAction::Rock => 1,
        RpsAction::Paper => 2,
        RpsAction::Scissors => 3,
    };
    
    let win_score;
    if *opponent == *response {
        win_score = 3;
    } else if wins_over(response) == *opponent {
        win_score = 6;
    } else {
        win_score = 0;
    }

    action_score + win_score
}

fn wins_over(action: &RpsAction) -> RpsAction {
    match action {
        RpsAction::Rock => RpsAction::Scissors,
        RpsAction::Paper => RpsAction::Rock,
        RpsAction::Scissors => RpsAction::Paper,
    }
}

fn read_input(filename: String) -> Vec<(RpsAction, RpsAction)> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    let re = Regex::new(r"([ABC]) ([XYZ])").unwrap();
    text.split("\n").map(|x| parse_line(&re, x)).collect()
}

fn parse_line(re: &Regex, line: &str) -> (RpsAction, RpsAction) {
    let captures = re.captures(line).unwrap();

    let opponent = match captures.get(1).unwrap().as_str() {
        "A" => RpsAction::Rock,
        "B" => RpsAction::Paper,
        "C" => RpsAction::Scissors,
        _ => panic!("Unable to parse command")
    };

    let response = match captures.get(2).unwrap().as_str() {
        "X" => RpsAction::Rock,
        "Y" => RpsAction::Paper,
        "Z" => RpsAction::Scissors,
        _ => panic!("Unable to parse command")
    };

    (opponent, response)
}