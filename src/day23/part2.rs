use std::{fs, collections::{HashSet, HashMap}};
use lazy_static::lazy_static;


struct Direction {
    get_adjecent: fn(&(i32, i32)) -> [(i32, i32); 3],
    get_move: fn(&(i32, i32)) -> (i32, i32),
}

#[allow(dead_code)]
pub fn solve() {
    let mut elves = read_input(String::from("src/day23/inputs/input.txt"));
    println!("Input count: {}", elves.len());

    println!("\nInitial state:");
    print_elves(&elves);

    let mut round = 0;
    loop {
        if do_round(&mut elves, &round) == 0 {
            println!("\nFinal state:");
            print_elves(&elves);

            println!("\nResult: {}", round + 1);
            break;
        }

        round = round + 1;
    }
}

fn print_elves(elves: &HashSet<(i32, i32)>) -> () {
    let (min_x, min_y, max_x, max_y) = get_rectangle(elves);

    for x in *min_x..(*max_x + 1) {
        for y in *min_y..(*max_y + 1) {
            if elves.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn get_rectangle(elves: &HashSet<(i32, i32)>) -> (&i32, &i32, &i32, &i32) {
    let min_x = elves.iter().map(|(x, _)| x).min().unwrap();
    let min_y = elves.iter().map(|(_, y)| y).min().unwrap();

    let max_x = elves.iter().map(|(x, _)| x).max().unwrap();
    let max_y = elves.iter().map(|(_, y)| y).max().unwrap();

    (min_x, min_y, max_x, max_y)
}

fn do_round(elves: &mut HashSet<(i32, i32)>, round: &usize) -> usize {
    // First half: propose moves
    let proposed_moves: Vec<((i32, i32), (i32, i32))> = elves.iter()
        .filter_map(|elf| propose_move(elves, round, elf))
        .collect();

    // Second half: apply uncontested moves
    let uncontested_moves = get_uncontested_moves(proposed_moves);
    let move_count = uncontested_moves.len();

    for (elf_move, elf) in uncontested_moves.into_iter() {
        elves.remove(&elf);
        elves.insert(elf_move);
    }

    move_count
}

fn propose_move(elves: &HashSet<(i32, i32)>, round: &usize, elf: &(i32, i32)) -> Option<((i32, i32), (i32, i32))> {
    lazy_static! {
        static ref NEXT: [Direction; 4] = init_directions();
    }

    if get_all_neighbors(elf).iter().all(|space| !elves.contains(space)) {
        return None;
    }

    for i in 0..4 {
        let direction = &NEXT[(*round + i) % 4];

        let has_adjecent = (direction.get_adjecent)(elf).iter()
            .any(|space| elves.contains(space));
        if !has_adjecent {
            return Some((elf.clone(), (direction.get_move)(elf)));
        }
    }

    None
}

fn get_all_neighbors(elf: &(i32, i32)) -> [(i32, i32); 8] {
    let (x, y) = elf;
    [
        (*x - 1, *y - 1),
        (*x - 1, *y),
        (*x - 1, *y + 1),
        (*x, *y + 1),
        (*x + 1, *y + 1),
        (*x + 1, *y),
        (*x + 1, *y - 1),
        (*x, *y - 1),
    ]
}

fn get_uncontested_moves(proposed_moves: Vec<((i32, i32), (i32, i32))>) -> HashMap<(i32, i32), (i32, i32)> {
    let mut uncontested_moves: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut declined_moves: HashSet<(i32, i32)> = HashSet::new();

    for (elf, elf_move) in proposed_moves.into_iter() {
        if declined_moves.contains(&elf_move) {
            continue;
        }

        if uncontested_moves.contains_key(&elf_move) {
            uncontested_moves.remove(&elf_move);
            declined_moves.insert(elf_move);
        } else {
            uncontested_moves.insert(elf_move, elf);
        }
    }

    uncontested_moves
}

fn init_directions() -> [Direction; 4] {
    [
        // North
        Direction {
            get_adjecent: |(x, y)| [
                (*x - 1, *y - 1), (*x - 1, *y), (*x - 1, *y + 1)
            ],
            get_move: |(x, y)| (*x - 1, *y),
        },
    
        // South
        Direction {
            get_adjecent: |(x, y)| [
                (*x + 1, *y - 1), (*x + 1, *y), (*x + 1, *y + 1)
            ],
            get_move: |(x, y)| (*x + 1, *y),
        },

        // West
        Direction {
            get_adjecent: |(x, y)| [
                (*x - 1, *y - 1), (*x, *y - 1), (*x + 1, *y - 1)
            ],
            get_move: |(x, y)| (*x, *y - 1),
        },

        // East
        Direction {
            get_adjecent: |(x, y)| [
                (*x - 1, *y + 1), (*x, *y + 1), (*x + 1, *y + 1)
            ],
            get_move: |(x, y)| (*x, *y + 1),
        },    
    ]
}

fn read_input(filename: String) -> HashSet<(i32, i32)> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.lines().enumerate()
        .flat_map(|(x, line)| line.chars()
            .enumerate()
            .filter(|(_, ch)| *ch == '#')
            .map(move |(y, _)| (i32::try_from(x).unwrap(), i32::try_from(y.clone()).unwrap()))
        ).collect()
}