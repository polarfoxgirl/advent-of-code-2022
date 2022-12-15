use std::{fs, collections::HashMap};
use regex::Regex;
use lazy_static::lazy_static;

#[allow(dead_code)]
pub fn solve() {
    let sensors = read_input(String::from("src/day15/inputs/input.txt"));
    println!("Input count: {}", sensors.len());

    if let Some((x, y)) = try_find_beacon(&sensors, 0, 4000000) {
        let freq = u64::try_from(x).unwrap() * 4000000 + u64::try_from(y).unwrap();
        println!("Result: ({}, {}) with frequency {}", x, y, freq);
    } else {
        println!("Can't find beacon");
    }
}

fn try_find_beacon(sensors: &Vec<((i32, i32), (i32, i32))>, min_xy: i32, max_xy: i32) -> Option<(i32, i32)> {
    let mut x_ranges_map: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

    for x in min_xy..(max_xy + 1) {
        let y_ranges = get_y_ranges(sensors, &x);
        if has_full_coverage(&y_ranges, &min_xy, &max_xy) {
            continue;
        }

        for y in min_xy..(max_xy + 1) {
            if !inside_ranges(&y_ranges, &y) {
                let x_ranges = x_ranges_map
                    .entry(y)
                    .or_insert_with(|| get_x_ranges(sensors, &y));
                if !inside_ranges(x_ranges, &x) {
                    return Some((x, y));
                }
            }
        }
    }

    None
}

fn has_full_coverage(ranges: &Vec<(i32, i32)>, min_xy: &i32, max_xy: &i32) -> bool {
    ranges.iter()
        .any(|(r1, r2)| (*r1 <= *min_xy) && (*max_xy <= *r2))
}

fn get_x_ranges(sensors: &Vec<((i32, i32), (i32, i32))>, y: &i32) -> Vec<(i32, i32)> {
    sensors.iter()
        .filter_map(|s| get_x_sensor_range(y, s))
        .fold(Vec::new(), merge_in_range)
}

fn get_y_ranges(sensors: &Vec<((i32, i32), (i32, i32))>, x: &i32) -> Vec<(i32, i32)> {
    sensors.iter()
        .filter_map(|s| get_y_sensor_range(x, s))
        .fold(Vec::new(), merge_in_range)
}

fn get_x_sensor_range(row_y: &i32, sensor: &((i32, i32), (i32, i32))) -> Option<(i32, i32)> {
    let ((s_x, s_y), (b_x, b_y)) = sensor;

    let mnht_distance = (s_x - b_x).abs() + (s_y - b_y).abs();

    // Solving for x in (s_y - row_y).abs() + (s_x - x).abs() == mnht_distance
    let y_distance = (s_y - row_y).abs();

    // No solution
    if y_distance > mnht_distance {
        return None;
    }

    let x1 = y_distance + s_x - mnht_distance;
    let x2 =  mnht_distance + s_x - y_distance;

    if x1 <= x2 {
        return Some((x1, x2));
    } else {
        return Some((x2, x1));
    }
}

fn get_y_sensor_range(column_x: &i32, sensor: &((i32, i32), (i32, i32))) -> Option<(i32, i32)> {
    let ((s_x, s_y), (b_x, b_y)) = sensor;

    let mnht_distance = (s_x - b_x).abs() + (s_y - b_y).abs();

    // Solving for y in (s_y - y).abs() + (s_x - column_x).abs() == mnht_distance
    let x_distance = (s_x - column_x).abs();

    // No solution
    if x_distance > mnht_distance {
        return None;
    }

    let y1 = x_distance + s_y - mnht_distance;
    let y2 =  mnht_distance + s_y - x_distance;

    if y1 <= y2 {
        return Some((y1, y2));
    } else {
        return Some((y2, y1));
    }
}

fn merge_in_range(ranges: Vec<(i32, i32)>, merge_range: (i32, i32)) -> Vec<(i32, i32)> {

    let mut result = Vec::new();
    let mut new_ranges = Vec::new();

    for range in ranges {
        match try_merge_ranges(&range, &merge_range) {
            None => {
                result.push(range);
            },
            Some(new_range) => {
                new_ranges.push(new_range);
            },
        }
    }

    if new_ranges.len() > 0 {
        let final_range = new_ranges.into_iter()
            .reduce(|r1, r2| try_merge_ranges(&r1, &r2).unwrap())
            .unwrap();

        result.push(final_range);
    } else {
        result.push(merge_range);
    }

    result
}

fn try_merge_ranges(range1: &(i32, i32), range2: &(i32, i32)) -> Option<(i32, i32)> {
    let (x1, y1) = range1;
    let (x2, y2) = range2;

    if *x1 <= *x2 {
        if *y1 >= *y2 {
            return Some((*x1, *y1));
        }

        if *x2 - 1 <= *y1 {
            return Some((*x1, *y2));
        }
    }

    if *x2 <= *x1 {
        if *y2 >= *y1 {
            return Some((*x2, *y2));
        }

        if *x1 - 1 <= *y2 {
            return Some((*x2, *y1));
        }
    }

    None
}

fn inside_ranges(ranges: &Vec<(i32, i32)>, value: &i32) -> bool {
    ranges.iter()
        .any(|(r1, r2)| (*r1 <= *value) && (*value <= *r2))
}

fn read_input(filename: String) -> Vec<((i32, i32), (i32, i32))> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.lines().map(read_line).collect()
}

fn read_line(line: &str) -> ((i32, i32), (i32, i32)) {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"Sensor at x=(\-?\d+), y=(\-?\d+): closest beacon is at x=(\-?\d+), y=(\-?\d+)").unwrap();
    }

    if let Some(captures) = LINE_RE.captures(line) {
        let s_x: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let s_y: i32 = captures.get(2).unwrap().as_str().parse().unwrap();

        let b_x: i32 = captures.get(3).unwrap().as_str().parse().unwrap();
        let b_y: i32 = captures.get(4).unwrap().as_str().parse().unwrap();

        return ((s_x, s_y), (b_x, b_y));
    }

    panic!("Unable to parse line");
}