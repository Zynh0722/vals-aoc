use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut previous: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    let mut total: i32 = 0;

    for line in lines {
        let current = line.unwrap().parse().unwrap();

        if previous < current {
            total += 1;
        }

        previous = current;
    }

    print!("{}", total);
}