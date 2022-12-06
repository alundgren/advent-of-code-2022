use std::{path::Path, collections::VecDeque};
use regex::Regex;

use crate::utils;

fn push_if_non_empty(v: &mut VecDeque<char>, c: char) {
    if !c.is_whitespace() {
        v.push_back(c);
    }
}

fn solve_part1(filename: impl AsRef<Path>, is_part2 : bool) -> String {
    let lines = utils::lines_from_file(filename);
    let stack_count = lines.iter().filter(|x|!x.chars().any(|y|y.is_alphabetic())).next().unwrap().trim().split(' ').last().unwrap().parse::<usize>().unwrap();
    let mut stacks : Vec<VecDeque<char>> = Vec::new();
    for _ in 0..stack_count {
        stacks.push(VecDeque::new());
    }

    let mut is_top = true;
    let mut i = 0;
    while i < lines.len() {
        let line = &lines[i];        
        if line[0..3].chars().any(|x|x.is_ascii_digit()) {
            //stack numbers
            is_top = false;
            i += 1; //Skip the blank line
        } else if is_top {
            (0..stacks.len()).for_each(|i| {
                push_if_non_empty(&mut stacks[i], line.chars().nth(1 + 4*i).unwrap());
            });
        } else {
            let re = Regex::new(r"(?x)
            ^
            [^\d]* (\d+) [^\d]*
            [^\d]* (\d+) [^\d]*
            [^\d]* (\d+) [^\d]*
            $
            ").unwrap();
            let result = re.captures(line).unwrap();
            let move_count = result[1].parse::<usize>().unwrap();
            let from_stack_index = result[2].parse::<usize>().unwrap() - 1;
            let to_stack_index = result[3].parse::<usize>().unwrap() - 1;

            //println!("Stacks before: {:?}", stacks);
            //println!("move {:?} from {:?} to {:?}", move_count, from_stack_index + 1, to_stack_index + 1);

            let mut to_be_moved : Vec::<char> = Vec::new();
            for _ in 0..move_count {
                let tmp = stacks[from_stack_index].pop_front().unwrap();
                if is_part2 {
                    to_be_moved.push(tmp);
                } else {
                    stacks[to_stack_index].push_front(tmp);
                }                
                //println!("Stacks after moving {:?}: {:?}", tmp, stacks);
            }
            if is_part2 {
                to_be_moved.reverse();
                for tmp in to_be_moved {
                    stacks[to_stack_index].push_front(tmp);
                }
            }
        }
        i += 1;
    }

    let answer : String = stacks.iter().map(|x|{
        let f = x.front();
        
        match f {
            Some(c) => c,
            _ => &' '
        }
    }).collect();

    answer
}

pub fn part1_example_input() {
    let result = solve_part1("/workspaces/advent-of-code-2022/src/day5_example_input.txt", false);
    println!("{:?}", result);
}

pub fn part1_real_input() {
    let result = solve_part1("/workspaces/advent-of-code-2022/src/day5_real_input.txt", false);
    println!("{:?}", result);
}

pub fn part2_example_input() {
    let result = solve_part1("/workspaces/advent-of-code-2022/src/day5_example_input.txt", true);
    println!("{:?}", result);
}

pub fn part2_real_input() {
    let result = solve_part1("/workspaces/advent-of-code-2022/src/day5_real_input.txt", true);
    println!("{:?}", result);
}