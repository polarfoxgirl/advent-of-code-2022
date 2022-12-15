use std::{fs, collections::HashSet};
use regex::Regex;
use lazy_static::lazy_static;

#[allow(dead_code)]
pub fn solve() {
    let sensors = read_input(String::from("src/day15/inputs/input.txt"));
    println!("Input count: {}", sensors.len());

    let row_y = 2000000;
    let ranges: Vec<(i32, i32)> = sensors.iter()
        .filter_map(|s| get_sensor_range(&row_y, s))
        .fold(Vec::new(), merge_in_range);
    println!("Got {} ranges", ranges.len());

    let beacons: HashSet<i32> = sensors.iter()
        .filter(|((_, _), (_, y))| *y == row_y)
        .map(|((_, _), (x, _))| *x)
        .collect();

    let total_coverage = ranges.iter()
        .map(|(x, y)| *y + 1 - *x)
        .sum::<i32>();
    
    println!("Result: {} - {} = {}", total_coverage, beacons.len(), total_coverage - i32::try_from(beacons.len()).unwrap());
}

fn get_sensor_range(row_y: &i32, sensor: &((i32, i32), (i32, i32))) -> Option<(i32, i32)> {
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