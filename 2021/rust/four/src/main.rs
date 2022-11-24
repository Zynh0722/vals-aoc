use colored::*;
use std::fmt;
use std::io::{self, BufRead};

#[derive(Copy, Clone)]
struct Cell {
    pub value: u8,
    pub marked: bool,
}

impl Cell {
    pub fn new(value: u8) -> Self {
        Self {
            value,
            marked: false,
        }
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }

    pub fn mark(&mut self, value: u8) -> Self {
        if value == self.value {
            self.marked = true;
        }

        *self
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:>2}",
            if self.marked {
                self.value.to_string().green()
            } else {
                self.value.to_string().red()
            }
        )
    }
}

struct Board {
    cells: [[Cell; 5]; 5],
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [[Cell::new(0); 5]; 5],
        }
    }

    pub fn new_from_str(str: &str) -> Self {
        let mut new = Self::new();

        str.split("\n")
            .into_iter()
            .map(|row| row.split_whitespace())
            .enumerate()
            .for_each(|(i, row)| {
                row.enumerate()
                    .for_each(|(j, cell)| new.cells[i][j].set_value(cell.parse().unwrap()));
            });

        new
    }

    fn get_points(&self) -> u32 { 
        self.cells.iter().fold(0, |acc, row| acc + row.iter().fold(0, |acc, cell| if !cell.marked { acc + cell.value as u32 } else {acc} ))
     }

    pub fn bingo(&self) -> Option<u32> {
        for row in self.cells {
            let mut row_marks: u8 = 0;

            for cell in row {
                row_marks += cell.marked as u8;
            }

            if row_marks == 5 {
                return Some(self.get_points())
            }
        }

        for i in 0..self.cells.len() {
            let mut col_marks: u8 = 0;

            for j in 0..self.cells[i].len() {
                col_marks += self.cells[j][i].marked as u8;
            }

            if col_marks == 5 {
                return Some(self.get_points())
            }
        }

        None
    }

    pub fn mark(&mut self, lot: u8) {
        self.cells = self.cells.map(|row| row.map(|mut cell| cell.mark(lot)));
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.cells
            .iter()
            .map(|row| writeln!(f, "{:?}", row))
            .collect()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.cells
            .iter()
            .map(|row| writeln!(f, "{:?}", row))
            .collect()
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .join("\n")
        .split("\n\n")
        .map(|str| str.to_owned())
        .collect::<Vec<String>>();

    let lots = input
        .first()
        .unwrap()
        .split(",")
        .map(|str| str.parse::<u8>().unwrap());
    let mut boards: Vec<Board> = input[1..]
        .to_owned()
        .iter()
        .map(|board| Board::new_from_str(board))
        .collect();

    println!("{}", lots.clone().count());

    let mut points = 0;

    for lot in lots {
        for board in &mut boards {
            board.mark(lot);
        }

        // if boards.iter().any(|board| board.bingo().is_some()) {
        //     break;
        // }
        match boards.iter().map(|board| board.bingo()).filter(|opt| opt.is_some()).next().or(None) {
            Some(x) => { points += x.unwrap() * lot as u32; break; },
            _ => ()
        }
    }

    for board in boards {
        println!("{}", board);
    }

    println!("{}", points);
}
