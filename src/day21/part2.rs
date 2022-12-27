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
    let mut monkeys = read_input(String::from("src/day21/inputs/input.txt"));
    println!("Input count: {}", monkeys.len());

    monkeys.remove(&String::from("humn"));
    if let Monkey::Op(left, _, right) = monkeys.remove(&String::from("root")).unwrap() {
        let result = resolve_root_eq(&monkeys, &left, &right);
        println!("Result: {}", result);
    }
}

fn resolve_root_eq(monkeys: &HashMap<String, Monkey>, left: &String, right: &String) -> u64 {
    let mut known_monkeys = HashMap::new();

    let maybe_left_value = try_resolve_monkey(&monkeys, &mut known_monkeys, left);
    let maybe_right_value = try_resolve_monkey(&monkeys, &mut known_monkeys, right);

    if let Some(left_value) = maybe_left_value {
        return reverse_resolve(&monkeys, &known_monkeys, right, &left_value);
    }

    if let Some(right_value) = maybe_right_value {
        return reverse_resolve(&monkeys, &known_monkeys, left, &right_value);
    }

    panic!("Human dependency in both equality branches");
}

fn try_resolve_monkey(monkeys: &HashMap<String, Monkey>, known_monkeys: &mut HashMap<String, u64>, name: &String) -> Option<u64> {
    if let Some(monkey) = monkeys.get(name) {
        match monkey {
            Monkey::Number(x) => {
                known_monkeys.insert(name.clone(), *x);
                return Some(*x);
            },
            Monkey::Op(left, op, right) => {
                let maybe_left = try_resolve_monkey(monkeys, known_monkeys, left);
                let maybe_right = try_resolve_monkey(monkeys, known_monkeys, right);

                if let Some(left) = maybe_left {
                    if let Some(right) = maybe_right {
                        let value = apply_op(&left, &right, op);
                        known_monkeys.insert(name.clone(), value);
                        return Some(value);
                    }
                }
            }
        }
    }

    None
}

fn reverse_resolve(monkeys: &HashMap<String, Monkey>, known_monkeys: &HashMap<String, u64>, name: &String, result: &u64) -> u64 {
    lazy_static! {
        static ref HUMN: String = String::from("humn");
    }
    if *name == *HUMN {
        return *result;
    }

    match monkeys.get(name).unwrap() {
        Monkey::Number(_) => panic!("Unexpected unresolved number monkey"),
        Monkey::Op(left, op, right) => {

            if let Some(left_value) = known_monkeys.get(left) {
                let right_value = reverse_op_right(left_value, op, result);
                return reverse_resolve(monkeys, known_monkeys, right, &right_value);
            }

            if let Some(right_value) = known_monkeys.get(right) {
                let left_value = reverse_op_left(right_value, op, result);
                return reverse_resolve(monkeys, known_monkeys, left, &left_value);
            }

            panic!("Unknown values on both sides for monkey: {}", name);
        }
    }
}

fn apply_op(left: &u64, right: &u64, op: &Op) -> u64 {
    match op {
        Op::Add => *left + *right,
        Op::Sub => *left - *right,
        Op::Mul => *left * *right,
        Op::Div => *left / *right,
    }
}

fn reverse_op_right(left: &u64, op:&Op, result: &u64) -> u64 {
    match op {
        Op::Add => *result - *left,
        Op::Sub => *left - *result,
        Op::Mul => *result / *left,
        Op::Div => *left / *result,
    }
}

fn reverse_op_left(right: &u64, op:&Op, result: &u64) -> u64 {
    match op {
        Op::Add => *result - *right,
        Op::Sub => *result + *right,
        Op::Mul => *result / *right,
        Op::Div => *result * *right,
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