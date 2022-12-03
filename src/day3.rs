use std::{collections::HashSet, iter::FromIterator, path::Path};

use utils;

fn part1(filename: impl AsRef<Path>) {
    let packs = utils::lines_from_file(filename).expect("Could not load lines");

    let mut sum = 0_u32;
    for pack_content in packs {
        let pack_prios: Vec<_> = pack_content
            .chars()
            .map(|x| {
                if x.is_uppercase() {
                    x as u32 - 'A' as u32 + 27
                } else {
                    x as u32 - 'a' as u32 + 1
                }
            })
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
pub fn part1_example_input() {
    part1("/workspaces/advent-of-code-2022/src/day3_example_input.txt");
}

pub fn part1_real_input() {
    part1("/workspaces/advent-of-code-2022/src/day3_real_input.txt");
}
