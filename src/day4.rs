use std::{path::Path};

use utils;

fn part1(filename: impl AsRef<Path>, is_part1: bool) -> u32 {
    utils::lines_from_file(filename)
    .iter()
    .map(|line| {
        let [elf1,elf2] = utils::split_to_pair(line, ',');
        let [elf1_lower, elf1_upper] = utils::split_to_pair(&elf1, '-').map(|x| x.parse::<i32>().unwrap());
        let [elf2_lower, elf2_upper] = utils::split_to_pair(&elf2, '-').map(|x| x.parse::<i32>().unwrap());

        let is_contained =
        if is_part1 {
            (elf1_lower >= elf2_lower && elf1_upper <= elf2_upper) || (elf2_lower >= elf1_lower && elf2_upper <= elf1_upper)
        } else {
            //why: https://stackoverflow.com/questions/3269434/whats-the-most-efficient-way-to-test-if-two-ranges-overlap
            elf1_lower <= elf2_upper && elf2_lower <= elf1_upper
        };
        
        u32::from(is_contained)
    })
    .sum()
}

pub fn part1_example_input() {
    let result = part1("/workspaces/advent-of-code-2022/src/day4_example_input.txt", true);
    println!("{:?}", result);
}

pub fn part1_real_input() {
    let result = part1("/workspaces/advent-of-code-2022/src/day4_real_input.txt", true);
    println!("{:?}", result);
}

pub fn part2_example_input() {
    let result = part1("/workspaces/advent-of-code-2022/src/day4_example_input.txt", false);
    println!("{:?}", result);
}

pub fn part2_real_input() {
    let result = part1("/workspaces/advent-of-code-2022/src/day4_real_input.txt", false);
    println!("{:?}", result);
}