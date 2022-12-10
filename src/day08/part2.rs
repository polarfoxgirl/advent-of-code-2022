use std::{fs, collections::HashMap};

#[allow(dead_code)]
pub fn solve() {
    let (height, width, tree_heights) = read_input(String::from("src/day08/inputs/input.txt"));
    println!("Input count: {}", tree_heights.len());

    // Calc left views
    let mut left_views = HashMap::new();
    fn step_left((i, j): &(usize, usize), step: usize) -> (usize, usize) {
        (*i, *j - step)
    }

    for i in 0..height {
        for j in 0..width {
            let mut view = 0;

            if j > 0 {
                let tree_height = tree_heights.get(&(i, j)).unwrap();
                view = look_back(
                    &tree_heights, 
                    &left_views, 
                    step_left, 
                    &(i, j), 
                    tree_height, 
                    1
                );
            }

            left_views.insert((i, j), view);
        }
    }

    // Calc right views
    let mut right_views = HashMap::new();
    fn step_right((i, j): &(usize, usize), step: usize) -> (usize, usize) {
        (*i, *j + step)
    }

    for i in 0..height {
        for j in (0..width).rev() {
            let mut view = 0;

            if j < width - 1 {
                let tree_height = tree_heights.get(&(i, j)).unwrap();
                view = look_back(
                    &tree_heights, 
                    &right_views, 
                    step_right, 
                    &(i, j), 
                    tree_height, 
                    1
                );
            }

            right_views.insert((i, j), view);
        }
    }

    // Calc up views
    let mut up_views = HashMap::new();
    fn step_up((i, j): &(usize, usize), step: usize) -> (usize, usize) {
        (*i - step, *j)
    }

    for j in 0..width {
        for i in 0..height {
            let mut view = 0;

            if i > 0 {
                let tree_height = tree_heights.get(&(i, j)).unwrap();
                view = look_back(
                    &tree_heights, 
                    &up_views, 
                    step_up, 
                    &(i, j), 
                    tree_height, 
                    1
                );
            }

            up_views.insert((i, j), view);
        }
    }

    // Calc down views
    let mut down_views = HashMap::new();
    fn step_down((i, j): &(usize, usize), step: usize) -> (usize, usize) {
        (*i + step, *j)
    }

    for j in 0..width {
        for i in (0..height).rev() {
            let mut view = 0;

            if i < height - 1 {
                let tree_height = tree_heights.get(&(i, j)).unwrap();
                view = look_back(
                    &tree_heights, 
                    &down_views, 
                    step_down, 
                    &(i, j), 
                    tree_height, 
                    1
                );
            }

            down_views.insert((i, j), view);
        }
    }

    let mut tree_scores = HashMap::new();
    for i in 0..height {
        for j in 0..width {
            let tree = (i, j);
            let left_view = left_views.get(&tree).unwrap();
            let right_view = right_views.get(&tree).unwrap();
            let up_view = up_views.get(&tree).unwrap();
            let down_view = down_views.get(&tree).unwrap();

            let score = *left_view * *right_view * *up_view * *down_view;
            tree_scores.insert(tree, score);
        }
    }

    let result = tree_scores.values().map(|x| *x).max().unwrap_or_default();
    println!("Result: {}", result)
}

fn look_back(
        tree_heights: &HashMap<(usize, usize), u32>, 
        back_views: &HashMap<(usize, usize), usize>, 
        apply_step: fn(&(usize, usize), usize) -> (usize, usize), 
        tree: &(usize, usize), 
        tree_height: &u32, 
        step: usize
    ) -> usize {
    let other_tree = apply_step(tree, step);
    let other_tree_height = tree_heights.get(&other_tree).unwrap();

    if other_tree_height >= tree_height {
        return step;
    }

    let next_step = back_views.get(&other_tree).unwrap();
    if *next_step == 0 {
        return step;
    }

    return look_back(tree_heights, back_views, apply_step, tree, tree_height, *next_step + step);
}

fn read_input(filename: String) -> (usize, usize, HashMap<(usize, usize), u32>) {
    println!("Reading file {}", filename);
    let text = fs::read_to_string(filename).expect("Failed to read input");

    let mut tree_heights = HashMap::new();
    let lines: Vec<&str> = text.lines().collect();

    let height = lines.len();
    let mut width = 0;

    for (i, line) in lines.iter().enumerate() {
        let heights: Vec<u32> = line.chars().filter_map(|ch| ch.to_digit(10)).collect();
        if 0 == width {
            width = heights.len();
        }

        for (j, height) in heights.iter().enumerate() {
            tree_heights.insert((i, j), *height);
        }
    }

    (height, width, tree_heights)
}