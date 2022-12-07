use std::{collections::{VecDeque}, path::Path, fs};

pub fn part1_example() {    
    let result = solve("/workspaces/advent-of-code-2022/src/day6_example_input.txt", 4);
    println!("{:?}", result);
}


pub fn part2_example() {    
    let result = solve("/workspaces/advent-of-code-2022/src/day6_example_input.txt", 14);
    println!("{:?}", result);
}

pub fn part1_real() {    
    let result = solve("/workspaces/advent-of-code-2022/src/day6_real_input.txt", 4);
    println!("{:?}", result);
}

pub fn part2_real() {    
    let result = solve("/workspaces/advent-of-code-2022/src/day6_real_input.txt", 14);
    println!("{:?}", result);
}

fn solve(filename: impl AsRef<Path>, window_length: usize) -> usize {
    let input = fs::read_to_string(filename).expect("No such file");
    let mut window = VecDeque::<char>::new();
    for (i, c) in input.chars().enumerate() {
        while window.contains(&c) {
            window.pop_front();
        }
        window.push_back(c);
        if window.len() == window_length {
            return i + 1;
        }
    }
    panic!("no index found");
}