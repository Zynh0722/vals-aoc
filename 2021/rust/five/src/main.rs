use std::{
    cmp,
    collections::HashMap,
    hash::Hash,
    io::{self, BufRead},
};

enum Direction {
    X,
    Y,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point(i32, i32);

impl Point {
    pub fn x(&self) -> i32 {
        self.0
    }
    pub fn y(&self) -> i32 {
        self.1
    }
}

#[derive(Debug)]
struct Line(Point, Point);

impl Line {
    pub fn from_str(string: &str) -> Line {
        let mut points = string
            .split(" -> ")
            .map(|pair| {
                pair.split(",")
                    .map(|a| a.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .map(|pair| Point(pair[0], pair[1]))
            .collect::<Vec<Point>>();

        Line(points.pop().unwrap(), points.pop().unwrap())
    }

    pub fn get_points(&self) -> Option<Vec<Point>> {
        self.get_range().and_then(|(start, finish, dir)| {
            Some(
                (start..finish + 1)
                    .map(|a| match dir {
                        Direction::X => Point(a, self.0.y()),
                        Direction::Y => Point(self.0.x(), a),
                    })
                    .collect(),
            )
        })
    }

    fn get_range(&self) -> Option<(i32, i32, Direction)> {
        if self.x_is_straight() {
            Some(Self::min_max(self.get_ys()))
        } else if self.y_is_straight() {
            Some(Self::min_max(self.get_xs()))
        } else {
            None
        }
    }

    fn min_max((a, b, dir): (i32, i32, Direction)) -> (i32, i32, Direction) {
        (cmp::min(a, b), cmp::max(a, b), dir)
    }

    fn get_ys(&self) -> (i32, i32, Direction) {
        (self.0.y(), self.1.y(), Direction::Y)
    }

    fn get_xs(&self) -> (i32, i32, Direction) {
        (self.0.x(), self.1.x(), Direction::X)
    }

    pub fn x_is_straight(&self) -> bool {
        self.0.x() == self.1.x()
    }

    pub fn y_is_straight(&self) -> bool {
        self.0.y() == self.1.y()
    }
}

fn main() {
    let stdin = io::stdin();
    let points: Vec<Point> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| Line::from_str(&l))
        .filter_map(|l| l.get_points())
        .flatten()
        .collect();

    let mut hash: HashMap<Point, usize> = HashMap::new();

    for point in &points {
        *hash.entry(*point).or_default() += 1;
    }

    let danger_points = hash
        .into_iter()
        .filter(|(_, count)| *count >= 2)
        .map(|(a, _)| a);

    print!("{:?}", danger_points.count())
}
