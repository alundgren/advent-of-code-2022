use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn get_max_calories(filename: impl AsRef<Path>) -> i64 {
    let lines = lines_from_file(filename).expect("Could not load lines");
	let mut max_elf_calories = 0;
	let mut current_elf_calories = 0;
    for line in lines {
        if line.is_empty() {
            current_elf_calories = 0;
        } else {
            current_elf_calories += line.parse::<i64>().unwrap();
            if current_elf_calories > max_elf_calories {
                max_elf_calories = current_elf_calories;
            }
        }
    }
    max_elf_calories
}

pub fn run_real_input() {
    let max_calories = get_max_calories("/workspaces/vscode-remote-try-rust/src/day1_input.txt");
    println!("{max_calories}");
    assert!(max_calories == 68787, "Expected 68787 but got {}", max_calories);
}

pub fn run_example_input() {
    let max_calories = get_max_calories("/workspaces/vscode-remote-try-rust/src/day1_example_input.txt");
    println!("{max_calories}");
    assert!(max_calories == 24000, "Expected 24000 but got {}", max_calories);
}