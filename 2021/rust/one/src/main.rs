use std::io::{self, BufRead};
use std::num;

#[derive(Debug)]
enum UwUError {
    Io(io::Error),
    Parse(std::num::ParseIntError),
    Iter
}

impl From<io::Error> for UwUError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<num::ParseIntError> for UwUError {
    fn from(err: num::ParseIntError) -> Self {
        Self::Parse(err)
    }
}

fn main() -> Result<(), UwUError> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let mut previous: i32 = lines.next().ok_or(UwUError::Iter)?.parse()?;
    let mut total: i32 = 0;

    for line in lines {
        let current = line.parse()?;

        if previous < current {
            total += 1;
        }

        previous = current;
    }

    print!("{}", total);

    Ok(())
}