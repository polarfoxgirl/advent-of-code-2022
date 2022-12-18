use std::{fs, collections::HashSet};

enum Jet {
    Left,
    Right,
}

#[allow(dead_code)]
struct Rock {
    name: String,
    can_move_left: fn(&HashSet<(u32, u32)>, &(u32, u32)) -> bool,
    can_move_right: fn(&HashSet<(u32, u32)>, &(u32, u32)) -> bool,
    can_fall: fn(&HashSet<(u32, u32)>, &(u32, u32)) -> bool,
    settle: fn(&mut HashSet<(u32, u32)>, &(u32, u32)) -> (),
    height: fn(&(u32, u32)) -> u32,
}

#[allow(dead_code)]
pub fn solve() {
    let jets = read_input(String::from("src/day17/inputs/input.txt"));
    println!("Input count: {}", jets.len());

    let rock_types = init_rock_types();

    let mut rock_at_rest: HashSet<(u32, u32)> = HashSet::new();

    let mut current_height = 0;
    let mut current_jet = 0usize;

    for i in 0..1000000000000 {
        let rock = &rock_types[i % 5];
        // println!("{} falls", rock.name);

        (current_height, current_jet) = drop_rock(&jets, &mut rock_at_rest, rock, current_jet, current_height);
        // println!("After {} rock height is {} and jet is {} with {} rock volume", i, current_height, current_jet, rock_at_rest.len());
        // print_stack(&rock_at_rest, &current_height)
    }

    println!("Result: {}", current_height);
}

#[allow(dead_code)]
fn print_stack(rock_at_rest: &HashSet<(u32, u32)>, max_height: &u32) -> () {
    println!();
    for x in (1..*max_height + 1).rev() {
        for y in 1..8 {
            if rock_at_rest.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn drop_rock(jets: &Vec<Jet>, rock_at_rest: &mut HashSet<(u32, u32)>, rock: &Rock, mut current_jet: usize, mut current_height: u32) -> (u32, usize) {
    let mut position = (current_height + 4, 3);
    loop {
        // Do jet push
        match jets.get(current_jet % jets.len()) {
            Some(&Jet::Left) => {
                // println!("Left jet");
                if (rock.can_move_left)(rock_at_rest, &position) {
                    position = (position.0, position.1 - 1);
                    // println!("{} moves left", rock.name);
                }
            },
            Some(&Jet::Right) => {
                // println!("Right jet");
                if (rock.can_move_right)(rock_at_rest, &position) {
                    position = (position.0, position.1 + 1);
                    // println!("{} moves right", rock.name);
                }
            },
            None => panic!("Invalid jet index"),
        }
        current_jet = current_jet + 1;

        // Fall down
        if position.0 > 1 && (rock.can_fall)(rock_at_rest, &position) {
            position = (position.0 - 1, position.1);
            // println!("{} falls down", rock.name);
        } else {
            (rock.settle)(rock_at_rest, &position);
            // println!("{} settles", rock.name);

            let rock_height = (rock.height)(&position);
            if rock_height > current_height {
                current_height = rock_height;
            }

            return (current_height, current_jet);
        }
    }
}

fn init_rock_types() -> [Rock; 5] {
    static RIGHT_WALL: u32 = 8;

    let row = Rock {
        name: String::from("row"),
        can_move_left: |rock_at_rest, (x, y)| {
            if *y > 1 {
                return !rock_at_rest.contains(&(*x, *y - 1));
            }
            false
        },
        can_move_right: |rock_at_rest, (x, y)| {
            if *y + 4 < RIGHT_WALL {
                return !rock_at_rest.contains(&(*x, *y + 4));
            }            
            false
        },
        can_fall: |rock_at_rest, (x, y)| {
            [(*x - 1, *y), (*x - 1, *y + 1), (*x - 1, *y + 2), (*x - 1, *y + 3)].iter()
                .all(|p| !rock_at_rest.contains(p))
        },
        settle: |rock_at_rest, (x, y)| {
            [(*x, *y), (*x, *y + 1), (*x, *y + 2), (*x, *y + 3)].map(
                |p| {
                    if rock_at_rest.contains(&p) {
                        panic!("Row is settling into existing rock at ({}, {})", *x, *y);
                    }
                    rock_at_rest.insert(p)
                }
            );
        },
        height: |(x, _)| {
            *x
        }
    };

    let cross = Rock {
        name: String::from("cross"),
        can_move_left: |rock_at_rest, (x, y)| {
            if *y > 1 {
                return [(*x, *y), (*x + 1, *y - 1), (*x + 2, *y)].iter()
                    .all(|p| !rock_at_rest.contains(p));
            }
            false
        },
        can_move_right: |rock_at_rest, (x, y)| {
            if *y + 3 < RIGHT_WALL {
                return [(*x, *y + 2), (*x + 1, *y + 3), (*x + 2, *y + 2)].iter()
                    .all(|p| !rock_at_rest.contains(p));
            }
            false
        },
        can_fall: |rock_at_rest, (x, y)| {
            [(*x, *y), (*x - 1, *y + 1), (*x, *y + 2)].iter()
                .all(|p| !rock_at_rest.contains(p))
        },
        settle: |rock_at_rest, (x, y)| {
            [(*x + 1, *y), (*x, *y + 1), (*x + 1, *y + 1), (*x + 2, *y + 1), (*x + 1, *y + 2)].map(
                |p| {
                    if rock_at_rest.contains(&p) {
                        panic!("Cross is settling into existing rock at ({}, {})", *x, *y);
                    }
                    rock_at_rest.insert(p)
                }
            );
        },
        height: |(x, _)| {
            *x + 2
        }
    };

    let rev_l = Rock {
        name: String::from("reverse L"),
        can_move_left: |rock_at_rest, (x, y)| {
            if *y > 1 {
                return [(*x, *y - 1), (*x + 1, *y + 1), (*x + 2, *y + 1)].iter()
                    .all(|p| !rock_at_rest.contains(p));
            }
            false
        },
        can_move_right: |rock_at_rest, (x, y)| {
            if *y + 3 < RIGHT_WALL {
                return [(*x, *y + 3), (*x + 1, *y + 3), (*x + 2, *y + 3)].iter()
                    .all(|p| !rock_at_rest.contains(p));
            }
            false
        },
        can_fall: |rock_at_rest, (x, y)| {
            [(*x - 1, *y), (*x - 1, *y + 1), (*x - 1, *y + 2)].iter()
                .all(|p| !rock_at_rest.contains(p))
        },
        settle: |rock_at_rest, (x, y)| {
            [(*x, *y), (*x, *y + 1), (*x, *y + 2), (*x + 1, *y + 2), (*x + 2, *y + 2)].map(
                |p| {
                    if rock_at_rest.contains(&p) {
                        panic!("Reverse L is settling into existing rock at ({}, {})", *x, *y);
                    }
                    rock_at_rest.insert(p)
                }
            );
        },
        height: |(x, _)| {
            *x + 2
        }
    };

    let column = Rock {
        name: String::from("column"),
        can_move_left: |rock_at_rest, (x, y)| {
            if *y > 1 {
                return [(*x, *y - 1), (*x + 1, *y - 1), (*x + 2, *y - 1), (*x + 3, *y - 1)].iter()
                    .all(|p| !rock_at_rest.contains(p));
            }
            false
        },
        can_move_right: |rock_at_rest, (x, y)| {
            if *y + 1 < RIGHT_WALL {
                return [(*x, *y + 1), (*x + 1, *y + 1), (*x + 2, *y + 1), (*x + 3, *y + 1)].iter()
                    .all(|p| !rock_at_rest.contains(p));
            }
            false
        },
        can_fall: |rock_at_rest, (x, y)| {
            !rock_at_rest.contains(&(*x - 1, *y))
        },
        settle: |rock_at_rest, (x, y)| {
            [(*x, *y), (*x + 1, *y), (*x + 2, *y), (*x + 3, *y)].map(
                |p| {
                    if rock_at_rest.contains(&p) {
                        panic!("Column is settling into existing rock at ({}, {})", *x, *y);
                    }
                    rock_at_rest.insert(p)
                }
            );
        },
        height: |(x, _)| {
            *x + 3
        }
    };

    let square = Rock {
        name: String::from("square"),
        can_move_left: |rock_at_rest, (x, y)| {
            if *y > 1 {
                return [(*x, *y - 1), (*x + 1, *y - 1)].iter()
                    .all(|p| !rock_at_rest.contains(p));
            }
            false
        },
        can_move_right: |rock_at_rest, (x, y)| {
            if *y + 2 < RIGHT_WALL {
                return [(*x, *y + 2), (*x + 1, *y + 2)].iter()
                    .all(|p| !rock_at_rest.contains(p));
            }
            false
        },
        can_fall: |rock_at_rest, (x, y)| {
            [(*x - 1, *y), (*x - 1, *y + 1)].iter()
                .all(|p| !rock_at_rest.contains(p))
        },
        settle: |rock_at_rest, (x, y)| {
            [(*x, *y), (*x + 1, *y), (*x, *y + 1), (*x + 1, *y + 1)].map(
                |p| {
                    if rock_at_rest.contains(&p) {
                        panic!("Square is settling into existing rock at ({}, {})", *x, *y);
                    }
                    rock_at_rest.insert(p)
                }
            );
        },
        height: |(x, _)| {
            *x + 1
        }
    };

    [row, cross, rev_l, column, square]
}


fn read_input(filename: String) -> Vec<Jet> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.chars().map(read_jet).collect()
}

fn read_jet(ch: char) -> Jet {
    match ch {
        '>' => Jet::Right,
        '<' => Jet::Left,
        _ => panic!("Invalid jet symbol"),
    }
}