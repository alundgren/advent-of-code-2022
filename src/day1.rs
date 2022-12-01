use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn iterate_numbers(filename: impl AsRef<Path>, f: &mut dyn FnMut(Option<i64>)) {
    let lines = lines_from_file(filename).expect("Could not load lines");
    for line in lines {
        if line.is_empty() {
            f(None);
        } else {
            f(Some(line.parse::<i64>().unwrap()));
        }
    }
    f(None); //Inject a fake last empty line so we dont have to treat the last group differently
}

fn get_max_calories(filename: impl AsRef<Path>) -> i64 {
    let mut max_elf_calories = 0;
    let mut current_elf_calories = 0;
    iterate_numbers(filename, &mut |calorie_count| match calorie_count {
        Some(x) => current_elf_calories += x,
        None => {
            if current_elf_calories > max_elf_calories {
                max_elf_calories = current_elf_calories;
            }
            current_elf_calories = 0;
        }
    });
    max_elf_calories
}

fn get_top_three_max_calories_sum(filename: impl AsRef<Path>) -> i64 {
    let mut elf_calories = Vec::<i64>::new();
    let mut current_elf_calories = 0;
    iterate_numbers(filename, &mut |calorie_count| match calorie_count {
        Some(x) => current_elf_calories += x,
        None => {
            elf_calories.push(current_elf_calories);
            current_elf_calories = 0;
        }
    });
    elf_calories.sort_by(|a, b| b.cmp(a));
    elf_calories[0..3].iter().sum()
}

pub fn part1_real_input() {
    let max_calories = get_max_calories("/workspaces/vscode-remote-try-rust/src/day1_input.txt");
    println!("{max_calories}");
    assert!(
        max_calories == 68787,
        "Expected 68787 but got {}",
        max_calories
    );
}

pub fn part1_example_input() {
    let max_calories =
        get_max_calories("/workspaces/vscode-remote-try-rust/src/day1_example_input.txt");
    println!("{max_calories}");
    assert!(
        max_calories == 24000,
        "Expected 24000 but got {}",
        max_calories
    );
}

pub fn part2_example_input() {
    let result = get_top_three_max_calories_sum(
        "/workspaces/vscode-remote-try-rust/src/day1_example_input.txt",
    );
    println!("{result}");
    assert!(result == 45000, "Expected 45000 but got {}", result);
}

pub fn part2_real_input() {
    let result =
        get_top_three_max_calories_sum("/workspaces/vscode-remote-try-rust/src/day1_input.txt");
    println!("{result}");
    //assert!(result == 45000, "Expected 45000 but got {}", result);
}
