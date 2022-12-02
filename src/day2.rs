use std::path::Path;
use utils;

pub fn part1_example_input() {
    let total_points = get_total_points("/workspaces/advent-of-code-2022/src/day2_example_input.txt");
    println!("{}", total_points.to_string());    
}

pub fn part1_real_input() {
    let total_points = get_total_points("/workspaces/advent-of-code-2022/src/day2_input.txt");
    println!("{}", total_points.to_string());    
}

fn get_total_points(filename: impl AsRef<Path>) -> u32 {
    let lines = utils::lines_from_file(filename).expect("Could not load lines");
    let mut total_points : u32 = 0;
    for line in lines {
        let opponent_choice = line.chars().nth(0).unwrap();
        let my_choice = line.chars().nth(2).unwrap();
        total_points += get_points(opponent_choice, my_choice);        
    }
    total_points
}


fn get_points(opponent_choice: char, my_choice: char) -> u32 {
    //1 for Rock, 2 for Paper, and 3 for Scissors
    //X, A for Rock, Y, B for Paper, and Z, C Scisscors
    let c1 = opponent_choice as u32 - 'A' as u32 + 1;
    let c2 = my_choice as u32 - 'X' as u32 + 1;
    //1 rock, //2 paper, 3 scissors
    let win_points = match c1 {
        1 if c2 == 1 => 3,
        1 if c2 == 2 => 6,
        1 if c2 == 3 => 0,
        2 if c2 == 1 => 0,
        2 if c2 == 2 => 3,
        2 if c2 == 3 => 6,
        3 if c2 == 1 => 6,
        3 if c2 == 2 => 0,
        3 if c2 == 3 => 3,
        _ => 0
    };
    win_points + c2
}