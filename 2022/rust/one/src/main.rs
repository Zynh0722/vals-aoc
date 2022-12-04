use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let mut elf_calories: Vec<i32> = input
        .split("\n\n")
        .map(|group| group.split("\n"))
        .map(|group| group.fold(0, |acc, value| acc + value.parse::<i32>().unwrap()))
        .collect();
    
    elf_calories.sort();

    let elf_calories: Vec<&i32> = elf_calories.iter().rev().collect();
    
    println!("Answer 1: {}", &elf_calories[0]);
    print!("Answer 2: {}", [&elf_calories[0], &elf_calories[1], &elf_calories[2]].iter().fold(0, |acc, val| acc + **val));
}
