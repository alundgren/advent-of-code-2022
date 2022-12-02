use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() {    
    day2();
}

fn day2() {
    let lines = lines_from_file("./src/day2_input.txt").expect("Could not load lines");
    for line in lines {
        let opponent_choice = line.chars().nth(0).unwrap();
        let my_choice = line.chars().nth(2).unwrap();
        let points = get_points(opponent_choice, my_choice);
        println!("{}", points.to_string());
        //let points = get_points(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap());
    }    
}
fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn get_points(opponent_choice: char, my_choice: char) -> u32 {
    //X, A for Rock, Y, B for Paper, and Z, C Scisscors
    let c1 = opponent_choice as u32 - 'X' as u32;
    let c2 = my_choice as u32 - 'A' as u32;
    //'0' rock, //'1' paper, '2' scissors
    match c1 {
        0 if c2 == 0 => 3,
        0 if c2 == 1 => 6,
        0 if c2 == 2 => 0,
        1 if c2 == 0 => 0,
        1 if c2 == 1 => 3,
        1 if c2 == 2 => 6,
        2 if c2 == 0 => 6,
        2 if c2 == 1 => 0,
        2 if c2 == 2 => 3,
        _ => 0
    }
}