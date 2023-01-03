use std::{fs, ops::{Sub, Add}};
use regex::Regex;
use lazy_static::lazy_static;
use memoize::memoize;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Resources {
    ore: u16,
    clay: u16,
    obsidian: u16,
}

impl Add for Resources {
    type Output = Resources;

    fn add(self, other: Resources) -> Resources {
        Resources { 
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
        }
    }
}

impl Sub for Resources {
    type Output = Resources;

    fn sub(self, other: Resources) -> Resources {
        Resources { 
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Blueprint {
    number: u16,
    ore_robot: Resources,
    clay_robot: Resources,
    obsidian_robot: Resources,
    geode_robot: Resources,
}

#[allow(dead_code)]
pub fn solve() {
    let mut blueprints = read_input(String::from("src/day19/inputs/test.txt"));
    println!("Input count: {}", blueprints.len());

    let first = optimize_blueprint(blueprints.pop().unwrap());
    println!("First blueprint max: {}", first);

    // let result: u16 = blueprints.iter()
    //     .map(|b| b.number * optimize_blueprint(b))
    //     .sum();
    // println!("Result: {}", result);
}

fn optimize_blueprint(blueprint: Blueprint) -> u16 {
    let resources = Resources { ore: 0, clay: 0, obsidian: 0 };
    let robots = (1, 0, 0, 0);
    let geodes = 0;
    let minutes = 24;

    get_max(blueprint, resources, robots, geodes, minutes)
}

#[memoize]
fn get_max(blueprint: Blueprint, resources: Resources, robots: (u16, u16, u16, u16), geodes: u16, minutes: u16) -> u16 {
    if minutes == 0 {
        return geodes;
    }

    let next_minutes = minutes - 1;
    let (new_resources, new_geods) = produce(robots);
    
    get_robot_options(blueprint, resources, robots).into_iter()
        .map(|(updated_robots, updated_resorces)| {
            get_max(blueprint, updated_resorces + new_resources, updated_robots, geodes + new_geods, next_minutes)
        })
        .max()
        .unwrap()
}

fn produce(robots: (u16, u16, u16, u16)) -> (Resources, u16) {
    let (ore_robots, clay_robots, obsidian_robots, geode_robots) = robots;

    (Resources {
        ore: ore_robots,
        clay: clay_robots,
        obsidian: obsidian_robots,
    }, geode_robots)
}

fn get_robot_options(blueprint: Blueprint, resources: Resources, robots: (u16, u16, u16, u16)) -> Vec<((u16, u16, u16, u16), Resources)> {
    let mut results = Vec::new();
    let (ore_robots, clay_robots, obsidian_robots, geode_robots) = robots;

    if has_enough(&resources, &blueprint.geode_robot) {
        results.push((
            (ore_robots, clay_robots, obsidian_robots, geode_robots + 1), 
            resources - blueprint.geode_robot,
        ));

        return results;
    }

    results.push((robots, resources));

    if has_enough(&resources, &blueprint.ore_robot) {
        results.push((
            (ore_robots + 1, clay_robots, obsidian_robots, geode_robots), 
            resources - blueprint.ore_robot,
        ));
    }

    if has_enough(&resources, &blueprint.clay_robot) {
        results.push((
            (ore_robots, clay_robots + 1, obsidian_robots, geode_robots), 
            resources - blueprint.clay_robot,
        ));
    }

    if has_enough(&resources, &blueprint.obsidian_robot) {
        results.push((
            (ore_robots, clay_robots, obsidian_robots + 1, geode_robots), 
            resources - blueprint.obsidian_robot,
        ));
    }

    results
}

fn has_enough(existing: &Resources, required: &Resources) -> bool {
    (existing.ore >= required.ore) && (existing.clay >= required.clay) && (existing.obsidian >= required.obsidian)
}

fn read_input(filename: String) -> Vec<Blueprint> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.lines().map(read_line).collect()
}

fn read_line(line: &str) -> Blueprint {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    }

    if let Some(captures) = LINE_RE.captures(line) {
        let parse_u16 = |i| {
            captures.get(i).unwrap().as_str().parse().unwrap()
        };

        return Blueprint {
            number: parse_u16(1),
            ore_robot: Resources { ore: parse_u16(2), clay: 0, obsidian: 0 },
            clay_robot: Resources { ore: parse_u16(3), clay: 0, obsidian: 0 },
            obsidian_robot: Resources { ore: parse_u16(4), clay: parse_u16(5), obsidian: 0 },
            geode_robot: Resources { ore: 0, clay: parse_u16(6), obsidian: parse_u16(7) },
        };
    }

    panic!("Unable to parse line");
}