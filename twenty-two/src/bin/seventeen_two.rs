use std::{
    collections::{BTreeMap, BTreeSet},
    iter::{Cycle, Enumerate, Map, Peekable},
    str::Chars,
    thread,
    time::{Duration, Instant},
};

#[derive(Debug)]
enum TickState {
    NeedNewShape,
    LettingNewShapeGust,
    LettingNewShapeFall,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Gust {
    Right,
    Left,
}

// left edge is 2 units
const X_SPAWN_OFFSET: u64 = 2;
const Y_SPAWN_OFFSET: u64 = 3;

struct App {
    tick_state: TickState,
    current_shape: Option<Vec<[u64; 2]>>,
    game_board: BTreeMap<u64, BTreeSet<u64>>,
    shapes_iter: Box<dyn Iterator<Item = Shape>>,
    gust_sequence_iter: Box<dyn Iterator<Item = (usize, Gust)>>,
}

impl App {
    fn new() -> Self {
        // read input from file
        let input_chars = {
            let input = std::fs::read_to_string("seventeen.txt").unwrap();
            input.chars().collect::<Vec<char>>()
        };

        // convert the sequence of `>` and `<` into 1s and -1s, and make it a cycle

        let gust_sequence_iter = input_chars
            .into_iter()
            .map(|c| match c {
                '>' => Gust::Right,
                '<' => Gust::Left,
                _ => unreachable!(),
            })
            .enumerate()
            .peekable()
            .cycle();

        let shapes_iter = [Shape::Line, Shape::Cross, Shape::J, Shape::I, Shape::Square]
            .into_iter()
            .cycle();

        // initialize the btree to 0
        let mut game_board: BTreeMap<u64, BTreeSet<u64>> = BTreeMap::new();
        (0..7).for_each(|x| {
            game_board.entry(x).or_insert_with(BTreeSet::new).insert(0);
        });

        Self {
            tick_state: TickState::NeedNewShape,
            current_shape: None,
            game_board,
            shapes_iter: Box::new(shapes_iter),
            gust_sequence_iter: Box::new(gust_sequence_iter),
        }
    }

    // spawn the shape on the map
    fn spawn_shape(&mut self) -> Vec<[u64; 2]> {
        let shape_coords = self.shapes_iter.next().unwrap().get_coords();

        // coordinates are relative to the leftmost coordinate

        shape_coords
            .into_iter()
            // 3 above the highest x
            .map(|[x, y]| {
                [
                    x + X_SPAWN_OFFSET,
                    y + Y_SPAWN_OFFSET + self.find_highest_y(),
                ]
            })
            .collect()
    }

    // do the gust
    fn gust(&mut self) {
        let (_, gust) = self.gust_sequence_iter.next().unwrap();

        let collision = self.current_shape.as_ref().unwrap().iter().any(|&[x, y]| {
            if x == 0 && gust == Gust::Left || x == 6 && gust == Gust::Right {
                return true;
            } else {
                let new_x = match gust {
                    Gust::Right => x + 1,
                    Gust::Left => x - 1,
                };
                if let Some(x_column) = self.game_board.get(&new_x) {
                    x_column.contains(&y)
                } else {
                    false
                }
            }
        });

        if !collision {
            self.current_shape
                .as_mut()
                .unwrap()
                .iter_mut()
                .for_each(|[x, _]| match gust {
                    Gust::Right => *x += 1,
                    Gust::Left => *x -= 1,
                });
        }
    }

    // let the shape fall.
    // returns true if shape fell into empty space, otherwise false if shape cannot move
    fn fall(&mut self) -> bool {
        let collision = self
            .current_shape
            .as_ref()
            .unwrap()
            .iter()
            .map(|&[x, y]| [x, y - 1])
            .any(|[x, y]| self.game_board.get(&x).unwrap().contains(&y));

        if !collision {
            self.current_shape
                .as_mut()
                .unwrap()
                .iter_mut()
                .for_each(|[_, y]| {
                    *y -= 1;
                });
        }
        !collision
    }

    fn tick(&mut self) {
        match self.tick_state {
            TickState::NeedNewShape => {
                self.current_shape = Some(self.spawn_shape());
                self.tick_state = TickState::LettingNewShapeGust
            }
            TickState::LettingNewShapeGust => {
                self.gust();
                self.tick_state = TickState::LettingNewShapeFall
            }
            TickState::LettingNewShapeFall => {
                if !self.fall() {
                    // add current_shape onto the map
                    self.current_shape
                        .as_mut()
                        .unwrap()
                        .into_iter()
                        .for_each(|[x, y]| {
                            self.game_board.get_mut(x).unwrap().insert(*y);
                        });
                    self.current_shape = None;
                    self.tick_state = TickState::NeedNewShape;
                } else {
                    self.tick_state = TickState::LettingNewShapeGust
                }
            }
        }
    }

    fn find_highest_y(&self) -> u64 {
        // find the highest y
        *self
            .game_board
            .values()
            .map(|set| set.iter().rev().next().unwrap())
            .max()
            .unwrap()
    }

    fn render(&self) {
        // get the highest and lowest y
        let mut highest_y = self.find_highest_y();

        if self.current_shape.is_some() {
            self.current_shape
                .as_ref()
                .unwrap()
                .iter()
                .for_each(|&[_, y]| {
                    if y > highest_y {
                        highest_y = y;
                    }
                });
            for y in (0..=highest_y).rev() {
                for x in 0..7 {
                    if self.current_shape.as_ref().unwrap().contains(&[x, y])
                        || self.game_board.get(&x).unwrap().contains(&y)
                    {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
        } else {
            for y in (0..=highest_y).rev() {
                for x in 0..7 {
                    if self.game_board.get(&x).unwrap().contains(&y) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
        }
        println!("----------------------");
    }

    fn run(&mut self) -> u64 {
        for _ in 0..1_000_000_000_000usize {
            self.tick();
            self.render();
            thread::sleep(Duration::from_secs(1));
        }

        self.find_highest_y()
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum Shape {
    Line,
    Cross,
    J,
    I,
    Square,
}

impl Shape {
    fn get_coords(self) -> Vec<[u64; 2]> {
        // ####

        // .#.
        // ###
        // .#.

        // ..#
        // ..#
        // ###

        // #
        // #
        // #
        // #

        // ##
        // ##

        // the coords are in the order of leftmost, then downmost
        match self {
            Shape::Line => vec![[0, 0], [1, 0], [2, 0], [3, 0]],
            Shape::Cross => vec![[0, 1], [1, 0], [1, 1], [1, 2], [2, 1]],
            Shape::J => vec![[0, 0], [1, 0], [2, 0], [2, 1], [2, 2]],
            Shape::I => vec![[0, 1], [0, 0], [0, 2], [0, 3]],
            Shape::Square => vec![[0, 0], [1, 0], [0, 1], [1, 1]],
        }
    }
}

mod m {
    use super::Shape;
    #[derive(Hash)]
    pub struct Input {
        pub s: Shape,
        pub i_gust: usize,
        pub v_countour: Vec<[i32; 2]>,
    }

    pub struct Output {
        pub turn: usize,
        pub height: usize,
    }
}

fn main() {
    let mut app = App::new();
    app.run();
}
