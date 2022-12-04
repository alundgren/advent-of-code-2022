use std::{collections::HashSet, iter::FromIterator, path::Path};

use utils;

fn to_prio(c: char) -> u32 {
    if c.is_uppercase() {
        utils::char_int(c, 'A') + 27
    } else {
        utils::char_int(c, 'a') + 1
    }
}

fn part1(filename: impl AsRef<Path>) -> u32 {
    utils::lines_from_file(filename)
        .iter()
        .map(|pack_content| {
            let pack_prios: Vec<_> = pack_content.chars().map(to_prio).collect();
            let compartment_length = pack_prios.len() / 2;
            let compartment1_prios =
                HashSet::<u32>::from_iter(pack_prios[0..compartment_length].iter().cloned());
            let compartment2_prios =
                HashSet::<u32>::from_iter(pack_prios[compartment_length..].iter().cloned());

            let common_prios = compartment1_prios.intersection(&compartment2_prios);
            common_prios.copied().next().unwrap()            
        })
        .sum()
}

fn part2(filename: impl AsRef<Path>) -> u32 {
    let lines = utils::lines_from_file(filename);
    let mut sum = 0;
    for chunk in lines.chunks(3) {
        let s1 = HashSet::<char>::from_iter(chunk[0].chars());
        let s2 = HashSet::<char>::from_iter(chunk[1].chars());
        let s3 = HashSet::<char>::from_iter(chunk[2].chars());
        let i12: HashSet<char> = s1.intersection(&s2).cloned().collect();

        let item_prio = to_prio(*i12.intersection(&s3).next().unwrap());
        sum += item_prio;
    }
    sum
}

pub fn part1_example_input() {
    let result = part1("/workspaces/advent-of-code-2022/src/day3_example_input.txt");
    assert!(result == 157);
    println!("{:0?}", result);
}

pub fn part1_real_input() {
    let result = part1("/workspaces/advent-of-code-2022/src/day3_real_input.txt");
    assert!(result == 7553);
    println!("{:0?}", result);
}

pub fn part2_example_input() {
    let result = part2("/workspaces/advent-of-code-2022/src/day3_example_input.txt");
    assert!(result == 70);
    println!("{:0?}", result);
}

pub fn part2_real_input() {
    let result = part2("/workspaces/advent-of-code-2022/src/day3_real_input.txt");
    assert!(result == 2758);
    println!("{:0?}", result);
}
