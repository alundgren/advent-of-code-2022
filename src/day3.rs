use std::{collections::HashSet, iter::FromIterator, path::Path};

use utils;

fn to_prio(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else {
        c as u32 - 'a' as u32 + 1
    }
}
fn part1(filename: impl AsRef<Path>) {
    let packs = utils::lines_from_file(filename).expect("Could not load lines");

    let mut sum = 0_u32;
    for pack_content in packs {
        let pack_prios: Vec<_> = pack_content
            .chars()
            .map(to_prio)
            .collect();
        let compartment_length = pack_prios.len() / 2;
        let compartment1_prios =
            HashSet::<u32>::from_iter(pack_prios[0..compartment_length].iter().cloned());
        let compartment2_prios =
            HashSet::<u32>::from_iter(pack_prios[compartment_length..].iter().cloned());

        let mut common_prios = compartment1_prios.intersection(&compartment2_prios);
        sum += *common_prios.next().unwrap();
    }

    println!("{:?}", sum);
}

fn part2(filename: impl AsRef<Path>) {
    let lines = utils::lines_from_file(filename).expect("Could not load lines");
    let mut sum = 0;
    for chunk in lines.chunks(3) {
        let s1 = HashSet::<char>::from_iter(chunk[0].chars());
        let s2 = HashSet::<char>::from_iter(chunk[1].chars());
        let s3 = HashSet::<char>::from_iter(chunk[2].chars());
        let i12 : HashSet<char> = s1.intersection(&s2).cloned().collect();

        let item_prio = to_prio(*i12.intersection(&s3).next().unwrap());
        sum += item_prio;        
    }
    println!("{:0?}", sum);
}

pub fn part1_example_input() {
    part1("/workspaces/advent-of-code-2022/src/day3_example_input.txt");
}

pub fn part1_real_input() {
    part1("/workspaces/advent-of-code-2022/src/day3_real_input.txt");
}

pub fn part2_example_input() {
    part2("/workspaces/advent-of-code-2022/src/day3_example_input.txt");
}

pub fn part2_real_input() {
    part2("/workspaces/advent-of-code-2022/src/day3_real_input.txt");
}