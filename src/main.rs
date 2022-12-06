mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod utils;

extern crate regex;

fn main() {
    println!("day1");
    day1::part1_example_input();
    day1::part1_real_input();
    day1::part2_example_input();
    day1::part2_real_input();
    
    println!();
    println!("day2");
    day2::part1_example_input();
    day2::part1_real_input();
    day2::part2_example_input();
    day2::part2_real_input();

    println!();
    println!("day3");
    day3::part1_example_input();
    day3::part1_real_input();
    day3::part2_example_input();
    day3::part2_real_input();

    println!();
    println!("day4");
    day4::part1_example_input();
    day4::part1_real_input();
    day4::part2_example_input();
    day4::part2_real_input();

    println!();
    println!("day5");
    day5::part1_example_input();
    day5::part1_real_input();
    day5::part2_example_input();
    day5::part2_real_input();
}

