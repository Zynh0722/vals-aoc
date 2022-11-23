use std::io::{self, BufRead};

#[derive(Debug)]
pub struct Sub {
    depth: i32,
    position: i32,
    aim: i32
}

impl Sub {
    pub fn new() -> Self {
        Self { depth: 0, position: 0, aim: 0 }
    }

    fn run_command(&mut self, (command, value): (&str, &str)) {
        let value: i32 = value.parse().or::<i32>(Ok(0)).unwrap();
        self.increment(match command {
            "forward" => (value, 0),
            "down" => (0, value),
            "up" => (0, -value),
            _ => (0, 0)
        })
    }

    fn increment(&mut self, (d_forward, d_aim): (i32, i32)) {
        self.position += d_forward;
        self.depth += d_forward * self.aim;
        self.aim += d_aim;
    }

    fn collect(&self) -> i32 {
        self.position * self.depth
    }
}

fn main() {
    let mut sub = Sub::new();

    let stdin = io::stdin();
    for pair in stdin.lock()
            .lines()
            .map(|l| l.unwrap())
            .map(|command_pair| 
                  command_pair.split_whitespace()
                              .map(|str| String::from(str)).collect::<Vec<_>>()) {
        sub.run_command((&pair[0][..], &pair[1][..]));
    }

    print!("{}", sub.collect());
}
