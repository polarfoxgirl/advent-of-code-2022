use std::{fs, collections::HashMap};
use regex::Regex;
use lazy_static::lazy_static;

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

enum Monkey {
    Number(u64),
    Op(String, Op, String),
}

#[allow(dead_code)]
pub fn solve() {
    let monkeys = read_input(String::from("src/day21/inputs/input.txt"));
    println!("Input count: {}", monkeys.len());

    let result = resolve_monkey(&monkeys, &String::from("root"));
    println!("Result: {}", result);
}

fn resolve_monkey(monkeys: &HashMap<String, Monkey>, name: &String) -> u64 {
    if let Some(monkey) = monkeys.get(name) {
        match monkey {
            Monkey::Number(x) => return *x,
            Monkey::Op(left, op, right) => {
                let left = resolve_monkey(monkeys, left);
                let right = resolve_monkey(monkeys, right);
                return apply_op(&left, &right, op);
            }
        }
    }

    panic!("Unknown monkey: {}", name);
}

fn apply_op(left: &u64, right: &u64, op: &Op) -> u64 {
    match op {
        Op::Add => *left + *right,
        Op::Sub => *left - *right,
        Op::Mul => *left * *right,
        Op::Div => *left / *right,
    }
}

fn read_input(filename: String) -> HashMap<String, Monkey> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.lines().map(read_line).collect()
}

fn read_line(line: &str) -> (String, Monkey) {
    lazy_static! {
        static ref NUMBER_RE: Regex = Regex::new(r"(\w+): (\d+)").unwrap();
        static ref OP_RE: Regex = Regex::new(r"(\w+): (\w+) ([+\-*/]) (\w+)").unwrap();
    }

    if let Some(captures) = NUMBER_RE.captures(line) {
        return (
            String::from(captures.get(1).unwrap().as_str()),
            Monkey::Number(captures.get(2).unwrap().as_str().parse().unwrap())
        );
    }

    if let Some(captures) = OP_RE.captures(line) {
        let op = get_op(captures.get(3).unwrap().as_str());
        return (
            String::from(captures.get(1).unwrap().as_str()),
            Monkey::Op(String::from(captures.get(2).unwrap().as_str()), op, String::from(captures.get(4).unwrap().as_str()))
        );
    }

    panic!("Unable to parse line: {}", line);
}

fn get_op(op_str: &str) -> Op {
    match op_str {
        "+" => Op::Add,
        "-" => Op::Sub,
        "*" => Op::Mul,
        "/" => Op::Div,
        other => panic!("Unrecorgnozed op: {}", other),
    }
}