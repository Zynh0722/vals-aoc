use std::io::{self, BufRead};

#[derive(Debug)]
struct BitTracker {
    bits: Vec<u32>,
    total_bits: u32,
}

impl BitTracker {
    pub fn new() -> Self {
        Self {
            bits: vec![0; 12],
            total_bits: 0,
        }
    }

    pub fn increment(&mut self, binary_string: &str) {
        binary_string.chars().enumerate().for_each(|(i, bit)| {
            if bit == '0' {
                self.increment_bytes()
            }
            if bit == '1' {
                self.increment_bit(i)
            }
        });
    }

    fn increment_bytes(&mut self) {
        self.total_bits += 1;
    }

    fn increment_bit(&mut self, index: usize) {
        self.bits[index] += 1;
        self.increment_bytes();
    }

    pub fn get_gamma_epsilon(&self) -> Vec<bool> {
        self.bits
            .iter()
            .map(|bit_count| bit_count > &(self.total_bits / 12 - bit_count))
            .collect()
    }

    pub fn get_power_consumption(&self) -> u32 {
        self.get_gamma_epsilon()
            .iter()
            .map(|b| if *b { ("1", "0") } else { ("0", "1") })
            .fold(["".to_owned(), "".to_owned()], |strings, chars| {
                [strings[0].to_owned() + chars.0, strings[1].to_owned() + chars.1]
            }).iter()
            .map(|str| u32::from_str_radix(&str, 2).unwrap())
            .fold(1, |acc, val| acc * val)
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());

    let mut tracker = BitTracker::new();

    for line in lines {
        tracker.increment(&line);
    }

    print!("{:?}", tracker.get_power_consumption());
}
