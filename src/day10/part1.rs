use std::fs;
use regex::Regex;
use lazy_static::lazy_static;

enum Cmd {
    Noop,
    Addx(i32),
}

#[allow(dead_code)]
pub fn solve() {
    let commands = read_input(String::from("src/day10/inputs/input.txt"));
    println!("Input count: {}", commands.len());

    let (cycles, x, signal) = commands.iter().fold((0, 1, 0), exec_cmd);
    println!("Result: {} cycles, x = {}, sum of signals {}", cycles, x, signal)
}

fn exec_cmd(state: (i32, i32, i32), command: &Cmd) -> (i32, i32, i32) {
    let (cycles, x, signal) = state;

    match *command {
        Cmd::Noop => (cycles + 1, x, update_signal(cycles + 1, x, signal)),
        Cmd::Addx(value) => {
            let new_signal = update_signal(cycles + 2, x, update_signal(cycles + 1, x, signal));
            return (cycles + 2, x + value, new_signal)
        },
    }
}

fn should_update_signal(cycles: i32) -> bool {
    (cycles - 20) % 40 == 0
}

fn update_signal(cycles: i32, x: i32, signal: i32) -> i32 {
    if should_update_signal(cycles) {        
        let new_signal = signal + cycles * x;
        println!("Updating sum to {} with {}*{} on cycle {}", new_signal, cycles, x, cycles);
        new_signal
    } else {
        signal
    }
}

fn read_input(filename: String) -> Vec<Cmd> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.lines().map(read_line).collect()
}

fn read_line(line: &str) -> Cmd {
    lazy_static! {
        static ref ADDX_RE: Regex = Regex::new(r"addx (\-?\d+)").unwrap();
    }

    if line == "noop" {
        return Cmd::Noop;
    }

    if let Some(captures) = ADDX_RE.captures(line) {
        let value: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        return Cmd::Addx(value);
    }

    panic!("Unable to parse line");
}