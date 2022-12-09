use std::{
    collections::HashSet,
    ops::{Add, ControlFlow},
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl Pos {
    // distance in squared terms
    fn dist_squared(&self, other: &Pos) -> i32 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2)
    }
}

fn main() {
    let dydx = [(0, 1), (0, -1), (-1, 0), (1, 0)];
    let diagonals: Vec<_> = [-1, 1]
        .into_iter()
        .flat_map(|dy| [-1, 1].map(|dx| (dy, dx)))
        .collect();
    let diagonals = &diagonals;

    let mut snake = vec![Pos::default(); 10];

    let mut traveled: HashSet<Pos> = HashSet::new();
    traveled.insert(snake[0]);

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
        .for_each(|(cycx, magnitude)| {
            // dbg!(dydx, magnitude);
            (0..magnitude).for_each(|_| {
                // .123#  -> ..123#
                // .123 # -> .12 3# -> .1 23#
                // when the head is out of range, we need to set the tail to the
                // buffer

                // store the head's current position
                // let mut buffer_temp = snake[0];
                // move the snake's head to the new position
                snake[0] = snake[0] + cycx;

                // starting from the next segment after the head
                (0..(snake.len() - 1)).for_each(|i| {
                    // compare the current segment to previous segment
                    // if the length is greater than 2
                    let dist = snake[i].dist_squared(&snake[i + 1]);
                    if dist > 2 {
                        if snake[i].x == snake[i + 1].x {
                            snake[i + 1].y += (snake[i].y - snake[i + 1].y) / 2;
                        } else if snake[i].y == snake[i + 1].y {
                            snake[i + 1].x += (snake[i].x - snake[i + 1].x) / 2;
                        } else {
                            snake[i + 1] = snake[i + 1]
                                + *diagonals
                                    .iter()
                                    .find(|dydx| {
                                        (snake[i + 1] + **dydx).dist_squared(&snake[i]) <= 2
                                    })
                                    .unwrap();
                        }
                    }
                    // ControlFlow::Break(-1)
                });
                if traveled.insert(snake[snake.len() - 1]) {
                    count += 1;
                }
                // dbg!("_)0000000000000000000000");
                // display_snake(&snake);
                // dbg!("_)0000000000000000000000");
                // dbg!(&snake);
                // display_snake(&snake);
                // println!("___________");
            });
            // display_snake(&snake);
            // println!("___________");
            // the tail always lags behind self
            // we need to keep track of what we've visted already
            // bestway is a hashset since we don't know if we'll be going to the
            // negative coords
            // we only increment our count if our previous position changed
        });

    dbg!(count);
}

// shift snake 10 to right
// board of 25?
// fn display_snake(v: &[Pos]) {
//     let mut board: Vec<_> = (0..50)
//         .map(|_| (0..50).map(|_| '.').collect::<Vec<_>>())
//         .collect();

//     for (idx, p) in v.iter().enumerate() {
//         board[(p.y + 25) as usize][(p.x + 25) as usize] = (idx as u8 + b'0') as char;
//         // dbg!(board[(p.y + 25) as usize][(p.x + 25) as usize]);
//     }

//     for line in board {
//         for c in line.iter() {
//             print!("{c}");
//         }
//         println!();
//     }
// }

fn display_snake(v: &[Pos]) {
    let mut board: Vec<_> = (0..5)
        .map(|_| (0..6).map(|_| '.').collect::<Vec<_>>())
        .collect();

    for (idx, p) in v.iter().enumerate() {
        board[p.y as usize][p.x as usize] = (idx as u8 + b'0') as char;
        // dbg!(board[(p.y + 25) as usize][(p.x + 25) as usize]);
    }

    for line in board.iter().rev() {
        for c in line.iter() {
            print!("{c}");
        }
        println!();
    }
}
