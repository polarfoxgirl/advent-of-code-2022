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

    let mut pixels = Vec::new();
    let (cycles, x) = commands.iter().fold((0, 1), |acc, cmd| exec_cmd(&mut pixels, acc, cmd));

    println!("Result: {} cycles, x = {}", cycles, x);
    print_pixels(&pixels);
}

fn exec_cmd(pixels: &mut Vec<bool>, state: (i32, i32), command: &Cmd) -> (i32, i32) {
    let (cycles, x) = state;
    update_pixels(pixels, cycles + 1, x);

    match *command {
        Cmd::Noop => (cycles + 1, x),
        Cmd::Addx(value) => {
            update_pixels(pixels, cycles + 2, x);
            return (cycles + 2, x + value)
        },
    }
}

fn update_pixels(pixels: &mut Vec<bool>, cycles: i32, x: i32) -> () {
    let pos = (cycles - 1) % 40;
    pixels.push((pos - x).abs() < 2);

}

fn print_pixels(pixels: &Vec<bool>) -> () {
    for (i, pixel) in pixels.iter().enumerate() {
        if *pixel {
            print!("#");
        } else {
            print!(".");
        }

        if (i + 1) % 40 == 0 {
            println!();
        }
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