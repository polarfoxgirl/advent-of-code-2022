use std::collections::HashMap;


struct Monkey {
    items: Vec<u64>,
    inspect: fn(u64) -> u64,
    throw: fn(u64) -> usize,
    inspect_counter: usize,
}

#[allow(dead_code)]
pub fn solve() {

    let mut monkeys = HashMap::new();
    // init_test_monkeys(&mut monkeys);
    init_monkeys(&mut monkeys);
    println!("Added {} monkeys", monkeys.len());

    // let mcd: u64 = 23 * 19 * 13 * 17;
    let mcd: u64 = 7 * 13 * 5 * 19 * 2 * 11 * 17 * 3;

    let rounds = 10000;
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for (j, item) in get_throws(&mcd, monkeys.get_mut(&i).unwrap()) {
                let target = monkeys.get_mut(&j).unwrap();
                target.items.push(item);
            }
        }
    }

    for i in 0..monkeys.len() {
        println!("Monkey {}: {}", i,  monkeys.get(&i).unwrap().inspect_counter);
    }
}

fn get_throws(mcd: &u64, monkey: &mut Monkey) -> Vec<(usize, u64)> {
    let throws: Vec<(usize, u64)> = monkey.items
        .drain(..)
        .map(|item| {
            let new_level = (monkey.inspect)(item) % *mcd;
            ((monkey.throw)(new_level), new_level)
        }).collect();
    monkey.inspect_counter = monkey.inspect_counter + throws.len();

    return throws;
}

#[allow(dead_code)]
fn init_monkeys(monkeys: &mut HashMap<usize, Monkey>) -> () {
    monkeys.insert(0, Monkey {
        items: vec![66, 79],
        inspect: |x| x * 11,
        throw: |x| if x % 7 == 0 { return 6 } else { return 7 },
        inspect_counter: 0,
    });

    monkeys.insert(1, Monkey {
        items: vec![84, 94, 94, 81, 98, 75],
        inspect: |x| x * 17,
        throw: |x| if x % 13 == 0 { return 5 } else { return 2 },
        inspect_counter: 0,
    });

    monkeys.insert(2, Monkey {
        items: vec![85, 79, 59, 64, 79, 95, 67],
        inspect: |x| x + 8,
        throw: |x| if x % 5 == 0 { return 4 } else { return 5 },
        inspect_counter: 0,
    });

    monkeys.insert(3, Monkey {
        items: vec![70],
        inspect: |x| x + 3,
        throw: |x| if x % 19 == 0 { return 6 } else { return 0 },
        inspect_counter: 0,
    });

    monkeys.insert(4, Monkey {
        items: vec![57, 69, 78, 78],
        inspect: |x| x + 4,
        throw: |x| if x % 2 == 0 { return 0 } else { return 3 },
        inspect_counter: 0,
    });

    monkeys.insert(5, Monkey {
        items: vec![65, 92, 60, 74, 72],
        inspect: |x| x + 7,
        throw: |x| if x % 11 == 0 { return 3 } else { return 4 },
        inspect_counter: 0,
    });

    monkeys.insert(6, Monkey {
        items: vec![77, 91, 91],
        inspect: |x| x * x,
        throw: |x| if x % 17 == 0 { return 1 } else { return 7 },
        inspect_counter: 0,
    });

    monkeys.insert(7, Monkey {
        items: vec![76, 58, 57, 55, 67, 77, 54, 99],
        inspect: |x| x + 6,
        throw: |x| if x % 3 == 0 { return 2 } else { return 1 },
        inspect_counter: 0,
    });
}

#[allow(dead_code)]
fn init_test_monkeys(monkeys: &mut HashMap<usize, Monkey>) -> () {
    monkeys.insert(0, Monkey {
        items: vec![79, 98],
        inspect: |x| x * 19,
        throw: |x| if x % 23 == 0 { return 2 } else { return 3 },
        inspect_counter: 0,
    });

    monkeys.insert(1, Monkey {
        items: vec![54, 65, 75, 74],
        inspect: |x| x + 6,
        throw: |x| if x % 19 == 0 { return 2 } else { return 0 },
        inspect_counter: 0,
    });

    monkeys.insert(2, Monkey {
        items: vec![79, 60, 97],
        inspect: |x| x * x,
        throw: |x| if x % 13 == 0 { return 1 } else { return 3 },
        inspect_counter: 0,
    });

    monkeys.insert(3, Monkey {
        items: vec![74],
        inspect: |x| x + 3,
        throw: |x| if x % 17 == 0 { return 0 } else { return 1 },
        inspect_counter: 0,
    });
}