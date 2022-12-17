use std::{fs, collections::HashMap};
use regex::Regex;
use lazy_static::lazy_static;
use im::HashSet;

struct Valve {
    flow_rate: u32,
    tunnels: Vec<String>,
}

#[allow(dead_code)]
pub fn solve() {
    let valves = read_input(String::from("src/day16/inputs/test.txt"));
    println!("Input count: {}", valves.len());

    let start_valve = String::from("AA");
    let result = visit_valve(&valves, &start_valve, &0, &0, &0, &HashSet::new());
    println!("Result: {}", result);
}

fn visit_valve(valves: &HashMap<String, Valve>, valve_name: &String, minutes: &u32, total_flow: &u32, flow: &u32, open_valves: &HashSet<String>) -> u32 {
    if *minutes == 30 {
        return *total_flow;
    }

    let valve = valves.get(valve_name).unwrap();

    let updated_minutes = *minutes + 1;
    let updated_total  = *total_flow + *flow;

    if !open_valves.contains(valve_name) {
        let updated_open_valves = open_valves.update(valve_name.clone());
        let updated_flow = *flow + valve.flow_rate;

        return visit_valve(valves, valve_name, &updated_minutes, &updated_total, &updated_flow, &updated_open_valves);
    }

    valve.tunnels
        .iter()
        .map(|next_valve| visit_valve(
            valves,
            next_valve,
            &updated_minutes,
            &updated_total,
            flow,
            open_valves,
        ))
        .max()
        .unwrap()

}

fn read_input(filename: String) -> HashMap<String, Valve> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.lines()
        .map(read_line)
        .collect()
}

fn read_line(line: &str) -> (String, Valve) {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)").unwrap();
    }

    if let Some(captures) = LINE_RE.captures(line) {
        let name = String::from(captures.get(1).unwrap().as_str());
        let valve = Valve {
            flow_rate: captures.get(2).unwrap().as_str().parse().unwrap(),
            tunnels: captures.get(3).unwrap().as_str().split(", ").map(String::from).collect(),
        };

        return (name, valve);
    }

    panic!("Unable to parse line");
}