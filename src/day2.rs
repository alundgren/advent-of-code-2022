use std::path::Path;
use utils;

const MY_RPS : [char; 3] = ['X', 'Y', 'Z'];

pub fn part1_example_input() {
    let total_points = get_total_points("/workspaces/advent-of-code-2022/src/day2_example_input.txt", false);
    println!("{:?}", total_points);    
}

pub fn part1_real_input() {
    let total_points = get_total_points("/workspaces/advent-of-code-2022/src/day2_input.txt", false);
    println!("{:?}", total_points);    
}

pub fn part2_example_input() {
    let total_points = get_total_points("/workspaces/advent-of-code-2022/src/day2_example_input.txt", true);
    println!("{:?}", total_points);    
}

pub fn part2_real_input() {
    let total_points = get_total_points("/workspaces/advent-of-code-2022/src/day2_input.txt", true);
    println!("{:?}", total_points);    
}

fn get_total_points(filename: impl AsRef<Path>, is_part2 : bool) -> u32 {
    let lines = utils::lines_from_file(filename);
    let mut total_points : u32 = 0;
    for line in lines {
        let opponent_choice = line.chars().next().unwrap();
        let mut my_choice = line.chars().nth(2).unwrap();
        if is_part2 {
            my_choice = get_my_choice(opponent_choice, my_choice);
        }
        total_points += get_points(opponent_choice, my_choice);
    }
    total_points
}

fn get_my_choice(opponent_choice: char, desired_outcome : char) -> char {
    const DRAW : char = 'Y';
    const WIN : char = 'Z';
    const LOSE : char = 'X';
    const OPPONENT_CHOICE_ROCK : char = 'A';
    const OPPONENT_CHOICE_PAPER : char = 'B';
    const OPPONENT_CHOICE_SCISSORS : char = 'C';
    const MY_CHOICE_ROCK : char = 'X';
    const MY_CHOICE_PAPER: char = 'Y';
    const MY_CHOICE_SCISSORS: char = 'Z';    
    
    match opponent_choice {
        _ if desired_outcome == DRAW => *MY_RPS.get((opponent_choice as u32 - 'A' as u32) as usize).unwrap(),
        OPPONENT_CHOICE_ROCK if desired_outcome == WIN => MY_CHOICE_PAPER,
        OPPONENT_CHOICE_ROCK if desired_outcome == LOSE => MY_CHOICE_SCISSORS,
        OPPONENT_CHOICE_PAPER if desired_outcome == WIN => MY_CHOICE_SCISSORS,
        OPPONENT_CHOICE_PAPER if desired_outcome == LOSE => MY_CHOICE_ROCK,
        OPPONENT_CHOICE_SCISSORS if desired_outcome == WIN => MY_CHOICE_ROCK,
        OPPONENT_CHOICE_SCISSORS if desired_outcome == LOSE => MY_CHOICE_PAPER,
        _ => panic!("broken")
    }
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