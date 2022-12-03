use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let elf_calories = input
        .split("\n\n")
        .map(|group| group.split("\n"))
        .map(|group| group.fold(0, |acc, value| acc + value.parse::<i32>().unwrap()));

    let max_calories = elf_calories
        .max_by_key(|v| *v)
        .unwrap();
    
    print!("{}", max_calories)
}
