use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Add<(i32, i32)> for Pos {
    type Output = Pos;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Pos {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

fn main() {
    let dydx = [(0, 1), (0, -1), (-1, 0), (1, 0)];

    let mut current = Pos { x: 0, y: 0 };
    let mut tail = current;

    let mut traveled: HashSet<Pos> = HashSet::new();

    traveled.insert(tail);

    let mut count = 1;
    std::fs::read_to_string("nine.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let mut iter = l.split_whitespace();
            (
                match iter.next().unwrap().chars().next().unwrap() {
                    'U' => dydx[0],
                    'D' => dydx[1],
                    'L' => dydx[2],
                    'R' => dydx[3],
                    _ => panic!(),
                },
                iter.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .for_each(|(dydx, magnitude)| {
            // dbg!(dydx, magnitude);
            (0..magnitude).for_each(|_| {
                let res = current + dydx;
                // dbg!(res);
                if (res.x - tail.x).pow(2) + (res.y - tail.y).pow(2) > 2 {
                    tail = current;
                    // dbg!(tail, current);

                    if traveled.insert(tail) {
                        count += 1;
                    }
                } 
                current = res;
            });
            // the tail always lags behind self
            // we need to keep track of what we've visted already
            // bestway is a hashset since we don't know if we'll be going to the
            // negative coords
            // we only increment our count if our previous position changed
        });

    dbg!(count);
}
