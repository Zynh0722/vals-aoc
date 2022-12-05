use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let mut elf_calories: Vec<i32> = buf
        .split("\n\n")
        .map(|group| group.split("\n"))
        .map(|group| group.fold(0, |acc, value| acc + value.parse::<i32>().unwrap()))
        .collect();
    
    elf_calories.sort();

    let elf_calories: Vec<&i32> = elf_calories.iter().rev().collect();
    
    println!("Answer 1: {}", &elf_calories[0]);
    print!("Answer 2: {}", [&elf_calories[0], &elf_calories[1], &elf_calories[2]].iter().fold(0, |acc, val| acc + **val));
}
