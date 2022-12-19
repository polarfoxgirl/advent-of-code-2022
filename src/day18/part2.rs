use std::{fs, collections::{HashSet, HashMap}};

#[allow(dead_code)]
pub fn solve() {
    let blocks = read_input(String::from("src/day18/inputs/input.txt"));
    println!("Input count: {}", blocks.len());

    let trapped_air = find_air_bubbles(&blocks);
    println!("Identified {} trapped air", trapped_air.len());

    let result = get_surface_area(&blocks, &trapped_air);
    println!("Result: {}", result);
}

fn find_air_bubbles(blocks: &HashSet<(i16,i16,i16)>) -> HashSet<(i16, i16, i16)> {
    let boundaries = get_boundaries(&blocks);
    let (min_x, max_x, min_y, max_y, min_z, max_z) = boundaries;

    let mut air_zones: HashMap<(i16, i16, i16), u16> = HashMap::new();
    let mut zone_air: HashMap<u16, HashSet<(i16, i16, i16)>> = HashMap::new();

    // Using 0 to represent "definitely not a bubble" zone
    zone_air.insert(0, HashSet::new());

    let mut next_zone = 1;

    for x in min_x..(max_x + 1) {
        for y in min_y..(max_y + 1) {
            for z in min_z..(max_z + 1) {
                let space = (x, y, z);
                if !blocks.contains(&space) {

                    let mut adjecent_zones: HashSet<u16> = get_visited_neighbors(&space).iter()
                            .filter_map(|s| air_zones.get(&s))
                            .map(|zone| *zone)
                            .collect();

                    if is_border(&space, &boundaries) {
                        adjecent_zones.insert(0);
                    } 
                    
                    
                    if adjecent_zones.len() > 0 {
                        let zone = merge_zones(&mut air_zones, &mut zone_air, &adjecent_zones);
                        air_zones.insert(space, zone);
                        zone_air.get_mut(&zone).unwrap().insert(space);
                    } else {
                        air_zones.insert(space, next_zone);
                        zone_air.insert(next_zone, HashSet::from([space]));
                        next_zone = next_zone + 1;
                    }
                }
            }
        }
    }

    println!("Found {} air zones", zone_air.len());

    zone_air.remove(&0);

    zone_air.into_values()
        .flatten()
        .collect()
}

fn is_border(space: &(i16, i16, i16), boundaries: &(i16, i16, i16, i16, i16, i16)) -> bool {
    let (x, y , z) = space;
    let (min_x, max_x, min_y, max_y, min_z, max_z) = boundaries;

    (x == min_x) || (x == max_x) || (y == min_y) || (y == max_y) || (z == min_z) || (z == max_z)

}

fn merge_zones(air_zones: &mut HashMap<(i16, i16, i16), u16>, zone_air: &mut HashMap<u16, HashSet<(i16, i16, i16)>>, zones: &HashSet<u16>) -> u16 {
    let result_zone = zones.iter().map(|z| *z).min().unwrap();

    for zone in zones {
        if *zone == result_zone {
            continue;
        }

        let air = zone_air.remove(zone).unwrap();
        let result_air = zone_air.get_mut(&result_zone).unwrap();
        for space in air {
            air_zones.insert(space, result_zone);
            result_air.insert(space);
        }
    }

    result_zone
}

fn get_surface_area(blocks: &HashSet<(i16,i16,i16)>, trapped_air: &HashSet<(i16,i16,i16)>) -> usize {
    blocks.iter()
        .map(|b| {
            get_neighbors(&b).iter()
                .filter(|n| !blocks.contains(*n) && !trapped_air.contains(*n))
                .count()
        })
        .sum()
}

fn get_visited_neighbors(space: &(i16,i16,i16)) -> [(i16,i16,i16); 3] {
    let (x, y, z) = space;
    [
        (*x - 1, *y, *z),
        (*x, *y - 1, *z),
        (*x, *y, *z - 1),
    ]
}

fn get_neighbors(block: &(i16,i16,i16)) -> [(i16,i16,i16); 6] {
    let (x, y, z) = block;
    [
        (*x - 1, *y, *z),
        (*x + 1, *y, *z),
        (*x, *y - 1, *z),
        (*x, *y + 1, *z),
        (*x, *y, *z - 1),
        (*x, *y, *z + 1),
    ]
}

fn get_boundaries(blocks: &HashSet<(i16,i16,i16)>) -> (i16, i16, i16, i16, i16, i16) {
    let min_x = blocks.iter()
        .map(|(x, _, _)| *x)
        .min()
        .unwrap();
    let max_x = blocks.iter()
        .map(|(x, _, _)| *x)
        .max()
        .unwrap();

    let min_y = blocks.iter()
        .map(|(_, y, _)| *y)
        .min()
        .unwrap();
    let max_y = blocks.iter()
        .map(|(_, y, _)| *y)
        .max()
        .unwrap();

    let min_z = blocks.iter()
        .map(|(_, _, z)| *z)
        .min()
        .unwrap();
    let max_z = blocks.iter()
        .map(|(_, _, z)| *z)
        .max()
        .unwrap();

    (min_x, max_x, min_y, max_y, min_z, max_z)
}

fn read_input(filename: String) -> HashSet<(i16,i16,i16)> {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    text.lines()
        .map(read_line)
        .collect()
}

fn read_line(line: &str) -> (i16,i16,i16) {
    if let Some((first, tail)) = line.split_once(",") {
        if let Some((second, third)) = tail.split_once(",") {
            return (first.parse().unwrap(), second.parse().unwrap(), third.parse().unwrap());
        }
    }

    panic!("Unable to parse line");
}