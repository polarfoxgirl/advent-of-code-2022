use std::{fs, collections::{HashSet, HashMap}};

enum Move {
    Forward(u32),
    Left,
    Right,
}

enum Facing {
    Right,
    Down,
    Left,
    Up,
}

#[allow(dead_code)]
pub fn solve() {
    let (open_tiles, x_jumps, y_jumps, path, start) = read_input(String::from("src/day22/inputs/input.txt"));
    println!("Input count: {} open tiles, {} x jumps, {} y jumps, {} path moves, ({}, {}) start", open_tiles.len(), x_jumps.len(), y_jumps.len(), path.len(), start.0, start.1);

    let password = can_apply_path(&open_tiles, &x_jumps, &y_jumps, &path, &start);
    println!("Result: {}", password);
}

fn can_apply_path(open_tiles: &HashSet<(u32, u32)>, x_jumps: &HashMap<(u32, u32), (u32, u32)>, y_jumps: &HashMap<(u32, u32), (u32, u32)>, path: &Vec<Move>, tile: &(u32, u32)) -> u32 {
    let mut current = *tile;

    let mut direction = Facing::Right;
    for path_move in path {
        match path_move {
            Move::Forward(n) => current = do_move(open_tiles, x_jumps, y_jumps, &current, n, &direction),
            Move::Right => direction = turn_right(&direction),
            Move::Left => direction = turn_left(&direction),
        }
    }

    calc_password(&current, &direction)
}

fn do_move(open_tiles: &HashSet<(u32, u32)>, x_jumps: &HashMap<(u32, u32), (u32, u32)>, y_jumps: &HashMap<(u32, u32), (u32, u32)>, tile: &(u32, u32), n: &u32, direction: &Facing) -> (u32, u32) {
    let mut current = *tile;
    for _ in 0..*n {
        let next = move_one(x_jumps, y_jumps, &current, direction);
        if open_tiles.contains(&next) {
            current = next;
        } else {
            break;
        }
    }

    current
}

fn move_one(x_jumps: &HashMap<(u32, u32), (u32, u32)>, y_jumps: &HashMap<(u32, u32), (u32, u32)>, tile: &(u32, u32), direction: &Facing) -> (u32, u32) {
    let (x, y) = tile;

    let next = match direction {
        Facing::Right =>(*x, *y + 1),
        Facing::Down => (*x + 1, *y),
        Facing::Left => (*x, *y - 1),
        Facing::Up => (*x - 1, *y),
    };

    match direction {
        Facing::Right | Facing::Left => {
            if let Some(jump) = x_jumps.get(&next) {
                return *jump;
            }
        },
        Facing::Up | Facing::Down => {
            if let Some(jump) = y_jumps.get(&next) {
                return *jump;
            }
        }
    }

    next
}

fn turn_left(direction: &Facing) -> Facing {
    match direction {
        Facing::Right => Facing::Up,
        Facing::Down => Facing::Right,
        Facing::Left => Facing::Down,
        Facing::Up => Facing::Left,
    }
}

fn turn_right(direction: &Facing) -> Facing {
    match direction {
        Facing::Right => Facing::Down,
        Facing::Down => Facing::Left,
        Facing::Left => Facing::Up,
        Facing::Up => Facing::Right,
    }
}

fn calc_password(tile: &(u32, u32), direction: &Facing) -> u32 {
    let (x, y) = tile;
    let facing = match direction {
        Facing::Right => 0,
        Facing::Down => 1,
        Facing::Left => 2,
        Facing::Up => 3,
    };

    1000 * x + 4 * y + facing
}

fn read_input(filename: String) -> (HashSet<(u32, u32)>, HashMap<(u32, u32), (u32, u32)>, HashMap<(u32, u32), (u32, u32)>, Vec<Move>, (u32, u32)) {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    if let Some((board_text, path_text)) = text.split_once("\n\n") {
        let (open_tiles, x_jumps, y_jumps, start) = read_board(board_text);
        let path = read_path(path_text);

        return (open_tiles, x_jumps, y_jumps, path, start);
    }

    panic!("Unable to read input")
}

fn read_board(board_text: &str) -> (HashSet<(u32, u32)>, HashMap<(u32, u32), (u32, u32)>, HashMap<(u32, u32), (u32, u32)>, (u32, u32)) {
    let mut open_tiles : HashSet<(u32, u32)> = HashSet::new();
    let mut x_jumps: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
    let mut y_jumps: HashMap<(u32, u32), (u32, u32)> = HashMap::new();

    let mut min_x: HashMap<u32, u32> = HashMap::new();
    let mut max_x: HashMap<u32, u32> = HashMap::new();
    let mut min_y: HashMap<u32, u32> = HashMap::new();
    let mut max_y: HashMap<u32, u32> = HashMap::new();

    for (i, line) in board_text.lines().enumerate() {
        let x = u32::try_from(i).unwrap() + 1;
        for (j, ch) in line.chars().enumerate() {
            let y = u32::try_from(j).unwrap() + 1;
            if ch != ' ' {
                if !min_y.contains_key(&x) {
                    min_y.insert(x, y);
                }

                if !min_x.contains_key(&y) {
                    min_x.insert(y, x);
                }

                max_y.insert(x, y);
                max_x.insert(y, x);

                if ch == '.' {
                    open_tiles.insert((x.clone(), y.clone()));
                }
            }
        }
    }

    let start = (1, *min_y.get(&1).unwrap());

    for (y, min_x_value) in min_x {
        let max_x_value = max_x.remove(&y).unwrap();
        y_jumps.insert((min_x_value - 1, y), (max_x_value, y));
        y_jumps.insert((max_x_value + 1, y), (min_x_value, y));
    }

    for (x, min_y_value) in min_y {
        let max_y_value = max_y.remove(&x).unwrap();
        x_jumps.insert((x, min_y_value - 1), (x, max_y_value));
        x_jumps.insert((x, max_y_value + 1), (x, min_y_value));
    }

    (open_tiles, x_jumps, y_jumps, start)
}

fn read_path(path_text: &str) -> Vec<Move> {
    let mut moves = Vec::new();

    let mut value = 0;
    for ch in path_text.chars() {
        if ch.is_numeric() {
            value = value * 10 + ch.to_digit(10).unwrap();
        } else {
            if value > 0 {
                moves.push(Move::Forward(value));
                value = 0;
            }

            match ch {
                'R' => moves.push(Move::Right),
                'L' => moves.push(Move::Left),
                other => panic!("Unsupported character in path: {}", other),
            }
        }
    }

    moves.push(Move::Forward(value));
    moves
}