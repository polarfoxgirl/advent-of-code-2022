use std::fs;
use std::vec::IntoIter;
use regex::{Regex, Match};
use lazy_static::lazy_static;

enum Line {
    LsCmd,
    CdCmd(String),
    File(u32, String),
    Dir(String),
}

#[allow(dead_code)]
pub fn solve() {
    let lines = read_input(String::from("src/day07/inputs/input.txt"));
    println!("Input count: {}", lines.len());

    let mut lines_iter = lines.into_iter();
    if let Some(Line::CdCmd(root)) = lines_iter.next() {
        if "/" != root {
            panic!("Unexpected root")
        }

        let (total_size, dirs) = process_dir(&mut lines_iter);
        let required_space = 30000000 - (70000000 - total_size);

        let maybe_result = dirs.iter()
            .filter(|x| **x >= required_space)
            .min();

        match maybe_result {
            Some(result) => println!("Result: {}", result),
            None => panic!("No result found"),
        }
    } else {
        panic!("Expected cd as first command");
    }
}

fn process_dir(lines: &mut IntoIter<Line>) -> (u32, Vec<u32>) {
    if let Some(ls_line) = lines.next() {
        match ls_line {
            Line::LsCmd => (),
            _ => panic!("Expecting ls as first command inside new directory"),
        }
    } else {
        panic!("Expected ls but got EOF")
    }

    let mut total_size = 0;
    let mut subdirs = Vec::new();
    loop {
        if let Some(line) = lines.next() {
            match line {
                Line::CdCmd(dir_name) => {
                    if ".." == dir_name {
                        break;
                    }

                    let (subdir_size, mut subsubdirs) = process_dir(lines);
                    total_size = total_size + subdir_size;
                    subdirs.append(&mut subsubdirs);
                    subdirs.push(subdir_size);
                },
                Line::File(file_size, _) => total_size = total_size + file_size,
                Line::Dir(_) => (),
                Line::LsCmd => panic!("Unexpected ls command"),
            }

        } else {
            break;
        }
    }

    (total_size, subdirs)
}

fn read_input(filename: String) -> Vec<Line> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Line {
    lazy_static! {
        static ref CD_RE: Regex = Regex::new(r"\$ cd ([./\w]+)").unwrap();
        static ref FILE_RE: Regex  = Regex::new(r"(\d+) (\w+)").unwrap();
        static ref DIR_RE: Regex  = Regex::new(r"dir (\w+)").unwrap();
    }

    if "$ ls" == line {
        return Line::LsCmd;
    }

    if let Some(captures) = CD_RE.captures(line) {
        return Line::CdCmd(String::from(captures.get(1).unwrap().as_str()));
    }

    if let Some(captures) = FILE_RE.captures(line) {
        return Line::File(
            parse_u32(&captures.get(1).unwrap()),
            String::from(captures.get(2).unwrap().as_str())
        );
    }

    if let Some(captures) = DIR_RE.captures(line) {
        return Line::Dir(String::from(captures.get(1).unwrap().as_str()));
    }

    panic!("Unable to parse line: {}", line);
}

fn parse_u32(capture: &Match) -> u32 {
    capture.as_str().parse().unwrap()
}