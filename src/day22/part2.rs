use std::{fs, collections::{HashSet, HashMap}};

enum Move {
    Forward(u32),
    Left,
    Right,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

#[allow(dead_code)]
pub fn solve() {
    let (open_tiles, jumps, path, start, jump_types, walls) = read_input(String::from("src/day22/inputs/input.txt"));
    println!("Input count: {} open tiles, {} jumps, {} path moves, ({}, {}) start", open_tiles.len(), jumps.len(), path.len(), start.0, start.1);

    let password = apply_path(&open_tiles, &jumps, &path, &start, &jump_types, &walls);
    println!("Result: {}", password);
}

fn apply_path(open_tiles: &HashSet<(u32, u32)>, jumps: &HashMap<(u32, u32, Facing), (u32, u32, Facing)>, path: &Vec<Move>, start: &(u32, u32, Facing), jump_types: &HashMap<(u32, u32, Facing), String>, walls: &HashSet<(u32, u32)>) -> u32 {
    let mut current = *start;

    for path_move in path {
        match path_move {
            Move::Forward(n) => current = do_move(open_tiles, jumps, current, n, jump_types, walls),
            Move::Right => current = turn_right(current),
            Move::Left => current = turn_left(current),
        }
    }

    calc_password(current)
}

fn do_move(open_tiles: &HashSet<(u32, u32)>, jumps: &HashMap<(u32, u32, Facing), (u32, u32, Facing)>, state: (u32, u32, Facing), n: &u32, jump_types: &HashMap<(u32, u32, Facing), String>, walls: &HashSet<(u32, u32)>) -> (u32, u32, Facing) {
    let mut current = state;
    for _ in 0..*n {
        let next = move_one(jumps, &current, &jump_types);
        if open_tiles.contains(&(next.0, next.1)) {
            current = next;
        } else if walls.contains(&(next.0, next.1)){
            break;
        } else {
            panic!("Position ({}, {}, {}) is out of bounds", next.0, next.1, print(&next.2));
        }
    }

    current
}

fn move_one(jumps: &HashMap<(u32, u32, Facing), (u32, u32, Facing)>, state: &(u32, u32, Facing), jump_types: &HashMap<(u32, u32, Facing), String>) -> (u32, u32, Facing) {
    let (x, y, direction) = state;

    let next = match direction {
        Facing::Right =>(*x, *y + 1, Facing::Right),
        Facing::Down => (*x + 1, *y, Facing::Down),
        Facing::Left => (*x, *y - 1, Facing::Left),
        Facing::Up => (*x - 1, *y, Facing::Up),
    };

    if let Some(jump_result) = jumps.get(&next) {
        println!("Attempting jump ({}, {}, {}) -> ({}, {}, {}) of type {}", next.0, next.1, print(&next.2), jump_result.0, jump_result.1, print(&jump_result.2), jump_types.get(&next).unwrap());
        // println!("Attempting jump type {}",jump_types.get(&next).unwrap());
        return *jump_result;
    }

    next
}

fn print(direction: &Facing) -> &str {
    match direction {
        Facing::Right => "Right",
        Facing::Down => "Down",
        Facing::Left => "Left",
        Facing::Up => "Up",
    }
}

fn turn_left(state: (u32, u32, Facing)) -> (u32, u32, Facing) {
    let (x, y, direction) = state;
    match direction {
        Facing::Right => (x, y, Facing::Up),
        Facing::Down => (x, y, Facing::Right),
        Facing::Left => (x, y, Facing::Down),
        Facing::Up => (x, y, Facing::Left),
    }
}

fn turn_right(state: (u32, u32, Facing)) -> (u32, u32, Facing) {
    let (x, y, direction) = state;
    match direction {
        Facing::Right => (x, y, Facing::Down),
        Facing::Down => (x, y, Facing::Left),
        Facing::Left => (x, y, Facing::Up),
        Facing::Up => (x, y, Facing::Right),
    }
}

fn calc_password(state: (u32, u32, Facing)) -> u32 {
    let (x, y, direction) = state;
    let facing = match direction {
        Facing::Right => 0,
        Facing::Down => 1,
        Facing::Left => 2,
        Facing::Up => 3,
    };

    println!("Final position: ({}, {}, {})", x, y, print(&direction));
    1000 * x + 4 * y + facing
}

fn read_input(filename: String) -> (HashSet<(u32, u32)>, HashMap<(u32, u32, Facing), (u32, u32, Facing)>, Vec<Move>, (u32, u32, Facing), HashMap<(u32, u32, Facing), String>, HashSet<(u32, u32)>) {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    if let Some((board_text, path_text)) = text.split_once("\n\n") {
        let (open_tiles, jumps, start, jump_types, walls) = read_board(board_text);
        let path = read_path(path_text);

        return (open_tiles, jumps, path, start, jump_types, walls);
    }

    panic!("Unable to read input")
}

fn read_board(board_text: &str) -> (HashSet<(u32, u32)>, HashMap<(u32, u32, Facing), (u32, u32, Facing)>, (u32, u32, Facing), HashMap<(u32, u32, Facing), String>, HashSet<(u32, u32)>) {
    let mut open_tiles: HashSet<(u32, u32)> = HashSet::new();
    let mut walls: HashSet<(u32, u32)> = HashSet::new();

    let mut max_x = 0;
    for (i, line) in board_text.lines().enumerate() {
        let x = u32::try_from(i).unwrap() + 1;
        for (j, ch) in line.chars().enumerate() {
            let y = u32::try_from(j).unwrap() + 1;
            if ch == '.' {
                open_tiles.insert((x.clone(), y.clone()));
            }

            if ch == '#' {
                walls.insert((x.clone(), y.clone()));
            }
        }
        max_x = x;
    }

    // let n = max_x / 3;
    let n = max_x / 4;
    println!("n is {}", n);

    let start = (1, 2 * n + 1, Facing::Right);
    let (jumps, jump_types) = build_jumps(n);

    (open_tiles, jumps, start, jump_types, walls)
}

fn build_jumps(n: u32) -> (HashMap<(u32, u32, Facing), (u32, u32, Facing)>, HashMap<(u32, u32, Facing), String>) {
    let mut jumps: HashMap<(u32, u32, Facing), (u32, u32, Facing)> = HashMap::new();
    let mut jump_types: HashMap<(u32, u32, Facing), String> = HashMap::new();

    // 1-4 edge
    for (x1, x4) in (1..(n + 1)).zip(((2*n + 1)..(3*n + 1)).rev()) {
        jumps.insert((x1, n, Facing::Left), (x4, 1, Facing::Right));
        jump_types.insert((x1, n, Facing::Left), "1-4".to_owned());

        jumps.insert((x4, 0, Facing::Left), (x1, n + 1, Facing::Right));
        jump_types.insert((x4, 0, Facing::Left), "4-1".to_owned());
    }

    // 1-6 edge
    for (y1, x6) in ((n + 1)..(2*n + 1)).zip((3*n + 1)..(4*n + 1)) {
        jumps.insert((0, y1, Facing::Up), (x6, 1, Facing::Right));
        jump_types.insert((0, y1, Facing::Up), "1-6".to_owned());

        jumps.insert((x6, 0, Facing::Left), (1, y1, Facing::Down));
        jump_types.insert((x6, 0, Facing::Left), "6-1".to_owned());
    }

    // 2-3 edge
    for (y2, x3) in ((2*n + 1)..(3*n + 1)).zip((n + 1)..(2*n + 1)) {
        jumps.insert((n + 1, y2, Facing::Down), (x3, 2*n, Facing::Left));
        jump_types.insert((n + 1, y2, Facing::Down), "2-3".to_owned());

        jumps.insert((x3, 2*n + 1, Facing::Right), (n, y2, Facing::Up));
        jump_types.insert((x3, 2*n + 1, Facing::Right), "3-2".to_owned());
    }

    // 2-5 edge
    for (x2, x5) in (1..(n + 1)).zip(((2*n + 1)..(3*n + 1)).rev()) {
        jumps.insert((x2, 3*n + 1, Facing::Right), (x5, 2*n, Facing::Left));
        jump_types.insert((x2, 3*n + 1, Facing::Right), "2-5".to_owned());

        jumps.insert((x5, 2*n + 1, Facing::Right), (x2, 3*n, Facing::Left));
        jump_types.insert((x5, 2*n + 1, Facing::Right), "5-2".to_owned());
    }

    // 2-6 edge
    for (y2, y6) in ((2*n + 1)..(3*n + 1)).zip(1..(n+1)) {
        jumps.insert((0, y2, Facing::Up), (4*n, y6, Facing::Up));
        jump_types.insert((0, y2, Facing::Up), "2-6".to_owned());

        jumps.insert((4*n + 1, y6, Facing::Down), (1, y2, Facing::Down));
        jump_types.insert((4*n + 1, y6, Facing::Down), "6-2".to_owned());
    }

    // 3-4 edge
    for (x3, y4) in ((n + 1)..(2*n + 1)).zip(1..(n + 1)) {
        jumps.insert((x3, n, Facing::Left), (2*n + 1, y4, Facing::Down));
        jump_types.insert((x3, n, Facing::Left), "3-4".to_owned());

        jumps.insert((2*n, y4, Facing::Up), (x3, n + 1, Facing::Right));
        jump_types.insert((2*n, y4, Facing::Up), "4-3".to_owned());
    }

    // 5-6 edge
    for (y5, x6) in ((n + 1)..(2*n + 1)).zip((3*n + 1)..(4*n + 1)) {
        jumps.insert((3*n + 1, y5, Facing::Down), (x6, n, Facing::Left));
        jump_types.insert((3*n + 1, y5, Facing::Down), "5-6".to_owned());

        jumps.insert((x6, n + 1, Facing::Right), (3*n, y5, Facing::Up));
        jump_types.insert((x6, n + 1, Facing::Right), "6-5".to_owned());
    }

    (jumps, jump_types)
}

#[allow(dead_code)]
fn build_jumps_test(n: u32) -> (HashMap<(u32, u32, Facing), (u32, u32, Facing)>, HashMap<(u32, u32, Facing), String>) {
    let mut jumps: HashMap<(u32, u32, Facing), (u32, u32, Facing)> = HashMap::new();
    let mut jump_types: HashMap<(u32, u32, Facing), String> = HashMap::new();

    // 1-2 edge (fixed bug didn't affect anything)
    for (y1, y2) in ((2 * n + 1)..(3 * n + 1)).zip((1..(n + 1)).rev()) {
        jumps.insert((0, y1, Facing::Up), (n + 1, y2, Facing::Down));
        jump_types.insert((0, y1, Facing::Up), "1-2".to_owned()); // !

        jumps.insert((n, y2, Facing::Up), (1, y1, Facing::Down));
        jump_types.insert((n, y2, Facing::Up), "2-1".to_owned()); // !
    }

    // 1-3 edge ++
    for (x1, y3) in (1..(n + 1)).zip((n + 1)..(2 * n + 1)) {
        jumps.insert((x1, 2 * n, Facing::Left), (n + 1, y3, Facing::Down));
        jump_types.insert((x1, 2 * n, Facing::Left), "1-3".to_owned()); // !

        jumps.insert((n, y3, Facing::Up), (x1, 2 * n + 1, Facing::Right));
        jump_types.insert((n, y3, Facing::Up), "3-1".to_owned()); // + !
    }

    // 1-6 edge ?
    for (x1, x6) in (1..(n + 1)).zip(((2 * n + 1)..(3 * n + 1)).rev()) {
        jumps.insert((x1, 3 * n + 1, Facing::Right), (x6, 4 * n, Facing::Left));
        jump_types.insert((x1, 3 * n + 1, Facing::Right), "1-6".to_owned());

        jumps.insert((x6, 4 * n + 1, Facing::Right), (x1, 3 * n, Facing::Left));
        jump_types.insert((x6, 4 * n + 1, Facing::Right), "6-1".to_owned());
    }

    // 2-5 edge ++
    for (y2, y5) in (1..(n + 1)).zip(((2 * n + 1)..(3 * n + 1)).rev()) {
        jumps.insert((2 * n + 1, y2, Facing::Down), (3 * n, y5, Facing::Up));
        jump_types.insert((2 * n + 1, y2, Facing::Down), "2-5".to_owned()); // !

        jumps.insert((3 * n + 1, y5, Facing::Down), (2 * n, y2, Facing::Up));
        jump_types.insert((3 * n + 1, y5, Facing::Down), "5-2".to_owned()); // +
    }

    // 2-6 edge ?
    for (x2, y6) in ((n + 1)..(2 * n + 1)).zip(((3 * n + 1)..(4 * n + 1)).rev()) {
        jumps.insert((x2, 0, Facing::Left), (3 * n, y6, Facing::Up));
        jump_types.insert((x2, 0, Facing::Left), "2-6".to_owned());

        jumps.insert((3 * n + 1, y6, Facing::Down), (x2, 1, Facing::Right));
        jump_types.insert((3 * n + 1, y6, Facing::Down), "6-2".to_owned());
    }

    // 3-5 edge (fixed bug didn't affect anything)
    for (y3, x5) in ((n + 1)..(2 * n + 1)).zip(((2 * n + 1)..(3 * n + 1)).rev()) {
        jumps.insert((2 * n + 1, y3, Facing::Down), (x5, 2 * n + 1, Facing::Right));
        jump_types.insert((2 * n + 1, y3, Facing::Down), "3-5".to_owned()); // !

        jumps.insert((x5, 2 * n, Facing::Left), (2 * n, y3, Facing::Up));
        jump_types.insert((x5, 2 * n, Facing::Left), "5-3".to_owned());
    }

    // 4-6 edge ++
    for (x4, y6) in ((n + 1)..(2 * n + 1)).zip(((3 * n + 1)..(4 * n + 1)).rev()) {
        jumps.insert((x4, 3 * n + 1, Facing::Right), (2 * n + 1, y6, Facing::Down));
        jump_types.insert((x4, 3 * n + 1, Facing::Right), "4-6".to_owned()); // +

        jumps.insert((2 * n, y6, Facing::Up), (x4, 3 * n, Facing::Left));
        jump_types.insert((2 * n, y6, Facing::Up), "6-4".to_owned());
    }

    (jumps, jump_types)
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
                'L' =>  moves.push(Move::Left),
                other => panic!("Unsupported character in path: {}", other),
            }
        }
    }

    moves.push(Move::Forward(value));
    moves
}