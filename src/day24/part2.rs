use std::{fs, collections::HashSet};

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[allow(dead_code)]
pub fn solve() {
    let (height, width, mut blizzards) = read_input(String::from("src/day24/inputs/input.txt"));
    println!("Input count: {} by {} with {} blizzards", height, width, blizzards.len());

    let start = (0, 1);
    let almost_start = (1, 1);
    let end = (height - 1, width - 2);
    let almost_end = (height - 2, width - 2);

    let there;
    (there, blizzards) = do_traverse(&height, &width, blizzards, &start, &almost_end);

    let back;
    (back, blizzards) = do_traverse(&height, &width, blizzards, &end, &almost_start);

    let there_again;
    (there_again, _) = do_traverse(&height, &width, blizzards, &start, &almost_end);

    let result = there + back + there_again;
    println!("Result: {} = {} there, {} back, {} there again", result, there, back, there_again);
}

fn do_traverse(height: &u8, width: &u8, mut blizzards: Vec<(u8, u8, Direction)>, start: &(u8, u8), almost_end: &(u8, u8)) -> (u16, Vec<(u8, u8, Direction)>) {
    let mut minutes = 0u16;
    let mut positions : HashSet<(u8, u8)> = HashSet::new();
    positions.insert(*start);

    loop {
        if positions.contains(almost_end) {
            break;
        }

        minutes = minutes + 1;

        // Move blizzards
        blizzards = move_blizzards(&height, &width, blizzards);

        let taken_spaces = calc_taken_spaces(&blizzards);

        // Process possible positions
        positions = positions.into_iter()
            .flat_map(|p| get_moves(&height, &width, p))
            .collect::<HashSet<(u8, u8)>>()
            .into_iter()
            .filter(|p| !taken_spaces.contains(p))
            .collect();
    }

    // Process last move
    (minutes + 1, move_blizzards(height, width, blizzards))
}

fn move_blizzards(height: &u8, width: &u8, blizzards: Vec<(u8, u8, Direction)>) -> Vec<(u8, u8, Direction)> {
    blizzards.into_iter()
        .map(|(x, y, direction)| match direction {
            Direction::Right => {
                if y == *width - 2 {
                    (x, 1, direction)
                } else {
                    (x, y + 1, direction)
                }
            },
            Direction::Down => {
                if x == *height - 2 {
                    (1, y, direction)
                } else {
                    (x + 1, y, direction)
                }
            },
            Direction::Left => {
                if y == 1 {
                    (x, *width - 2, direction)
                } else {
                    (x, y - 1, direction)
                }
            },
            Direction::Up => {
                if x == 1 {
                    (*height - 2, y, direction)
                } else {
                    (x - 1, y, direction)
                }
            },
        })
        .collect()
}

fn calc_taken_spaces(blizzards: &Vec<(u8, u8, Direction)>) -> HashSet<(u8, u8)> {
    blizzards.iter()
        .map(|(x, y, _)| (*x, *y))
        .collect()
}

fn get_moves(height: &u8, width: &u8, (x, y): (u8, u8)) -> Vec<(u8, u8)> {
    let mut moves = Vec::new();

    moves.push((x, y));

    if x == 0 {
        moves.push((x + 1, y));
    } else if x == *height - 1 {
        moves.push((x - 1, y));
    } else {
        if x > 1 {
            moves.push((x - 1, y));
        }

        if x < *height - 2 {
            moves.push((x + 1, y));
        }

        if y > 1 {
            moves.push((x, y - 1));
        }

        if y < *width - 2 {
            moves.push((x, y + 1));
        }
    }

    moves
}

fn read_input(filename: String) -> (u8, u8, Vec<(u8, u8, Direction)>) {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    let mut lines = text.lines().enumerate();

    // Discard first line
    let (_, first_line) = lines.next().unwrap();
    let width = u8::try_from(first_line.len()).unwrap();

    let mut blizzards = Vec::new();

    let mut max_x = 0;
    for (x, line) in lines {
        max_x = x;
        for (y, ch) in line.chars().enumerate() {
            if let Some(direction) = read_direction(&ch) {
                blizzards.push(
                    (u8::try_from(x).unwrap(), u8::try_from(y).unwrap(), direction)
                );
            }
        }
    }

    let height = u8::try_from(max_x + 1).unwrap();

    (height, width, blizzards)
}

fn read_direction(ch: &char) -> Option<Direction> {
    match ch {
        '>' => Some(Direction::Right),
        'v' => Some(Direction::Down),
        '<' => Some(Direction::Left),
        '^' => Some(Direction::Up),
        _ => None,
    }
}