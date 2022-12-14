use std::{fs, collections::HashSet};

#[allow(dead_code)]
pub fn solve() {
    let rockways = read_input(String::from("src/day14/inputs/input.txt"));
    println!("Input count: {}", rockways.len());

    let rock = get_rock(&rockways);
    let floor = rock.iter().map(|(_, y)| *y).max().unwrap() + 2;

    let mut sand_at_rest: HashSet<(u32, u32)> = HashSet::new();
    loop {
        match drop_sand(&floor, &rock, &sand_at_rest) {
            None => break,
            Some(sand) => {
                sand_at_rest.insert(sand);
            },
        }
    }

    println!("Result: {}", sand_at_rest.len());
}

fn drop_sand(floor: &u32, rock: &HashSet<(u32,u32)>, sand_at_rest: &HashSet<(u32, u32)>) -> Option<(u32, u32)> {
    let mut sand = (500, 0);
    if sand_at_rest.contains(&sand) {
        return None;
    }

    loop {
        match move_sand(floor, rock, sand_at_rest, &sand) {
            None => return Some(sand),
            Some(new_sand) => {
                sand = new_sand;
            }
        }
    }
}

fn move_sand(floor: &u32, rock: &HashSet<(u32,u32)>, sand_at_rest: &HashSet<(u32, u32)>, sand: &(u32, u32)) -> Option<(u32, u32)> {
    if sand.1 + 1 == *floor {
        return None;
    }

    let down = (sand.0, sand.1 + 1);
    if !rock.contains(&down) && !sand_at_rest.contains(&down) {
        return Some(down);
    }

    let down_left = (sand.0 - 1, sand.1 + 1);
    if !rock.contains(&down_left) && !sand_at_rest.contains(&down_left) {
        return Some(down_left);
    }

    let down_right = (sand.0 + 1, sand.1 + 1);
    if !rock.contains(&down_right) && !sand_at_rest.contains(&down_right) {
        return Some(down_right);
    }

    None
}

fn get_rock(rockways: &Vec<Vec<(u32,u32)>>) -> HashSet<(u32,u32)> {
    let mut rock: HashSet<(u32,u32)> = HashSet::new();
    for rockway in rockways {
        let mut previous = None;
        for (x, y) in rockway {            
            if let Some((x_prev, y_prev)) = previous {

                // Horizontal line
                if *x == x_prev {
                    if *y > y_prev {
                        for j in 1..(*y - y_prev) {
                            rock.insert((*x, y_prev + j));
                        }
                    }
                    if *y < y_prev {
                        for j in 1..(y_prev - *y) {
                            rock.insert((*x, *y + j));
                        }
                    }
                }

                // Vertical line
                if *y == y_prev {
                    if *x > x_prev {
                        for i in 1..(*x- x_prev) {
                            rock.insert((x_prev + i, *y));
                        }
                    }
                    if *x < x_prev {
                        for i in 1..(x_prev - *x) {
                            rock.insert((*x + i, *y));
                        }
                    }
                }
            }

            rock.insert((*x, *y));
            previous = Some((*x, *y));
        }
    }

    rock
}

fn read_input(filename: String) -> Vec<Vec<(u32,u32)>> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.lines().map(read_line).collect()
}

fn read_line(line: &str) -> Vec<(u32,u32)> {
    line.split(" -> ").map(|s| {
        match s.split_once(",") {
            None => panic!("Invalid coordinate format"),
            Some((x_str, y_str)) => (x_str.parse().unwrap(), y_str.parse().unwrap())
        }
    }).collect()
}