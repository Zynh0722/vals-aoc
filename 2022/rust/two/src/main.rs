use std::io::{self, BufRead};

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn from_str(shape: &str) -> Option<Self> {
        match shape {
            "X" | "A" => Some(Self::Rock),
            "Y" | "B" => Some(Self::Paper),
            "Z" | "C" => Some(Self::Scissors),
            _ => None,
        }
    }

    pub fn points(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn beats(&self, other: &Self) -> VictoryState {
        match *self {
            Self::Rock => match *other {
                Self::Rock => VictoryState::Draw,
                Self::Paper => VictoryState::Loss,
                Self::Scissors => VictoryState::Win,
            },
            Self::Scissors => match *other {
                Self::Rock => VictoryState::Loss,
                Self::Paper => VictoryState::Win,
                Self::Scissors => VictoryState::Draw,
            },
            Self::Paper => match *other {
                Self::Rock => VictoryState::Win,
                Self::Paper => VictoryState::Draw,
                Self::Scissors => VictoryState::Loss,
            },
        }
    }
}

#[derive(Debug)]
enum VictoryState {
    Win,
    Loss,
    Draw,
}

fn get_points(opponent_shape: &Shape, your_shape: &Shape) -> i32 {
    let shape_points = your_shape.points();

    match your_shape.beats(opponent_shape) {
        VictoryState::Win => 6 + shape_points,
        VictoryState::Draw => 3 + shape_points,
        VictoryState::Loss => shape_points,
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().map(|l| l.unwrap()).map(|game| {
        let vec: Vec<String> = game.split_whitespace()
            .map(|s| s.to_string())
            .collect();

        (Shape::from_str(&vec[0]).unwrap(), Shape::from_str(&vec[1]).unwrap())
    });

    let total = input
        .map(|(opponent_shape, your_shape)| get_points(&opponent_shape, &your_shape))
        .fold(0, |acc, game_points| acc + game_points);

    print!("{}", total);
}
