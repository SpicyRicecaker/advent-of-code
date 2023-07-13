#![allow(unused)]

use std::{collections::BTreeMap, ops::RangeInclusive};

#[derive(Debug)]
enum Object {
    Sand,
    Wall,
}

struct Board {
    // we use a btreemap because we're not sure about the insertion order,
    // otherwise a vec would be fine
    map: BTreeMap<isize, BTreeMap<usize, Object>>,
    grains_in_rest: usize,
    floor: usize,
    // x, y
    tentative_sand: (isize, usize),
}

enum Direction {
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_array(&self) -> [isize; 2] {
        match self {
            Direction::Down => [0, 1],
            Direction::Left => [-1, 1],
            Direction::Right => [1, 1],
        }
    }
}

impl Board {
    fn render(&self, domain: RangeInclusive<isize>, range: RangeInclusive<usize>) {
        range.for_each(|y| {
            domain
                .clone()
                .map(|x| {
                    let Some(set) = self.map.get(&x) else {
                        return ".";
                    };

                    match set.get(&y) {
                        Some(s) => match s {
                            Object::Sand => "o",
                            Object::Wall => "#",
                        },
                        None => ".",
                    }
                })
                .for_each(|c| {
                    print!("{}", c);
                });
            println!();
        })
    }

    // function that takes in a grain of sand and tries to move it until it
    // either becomes stationary or repeats infinitely.
    // returns an ok if the operation succeeds without timeout, basically
    // also updates the cost of placing things

    fn drop_sand(&mut self, direction: Direction) -> Result<(), ()> {
        // set current tentative grain of sand to 500, 0
        // check for the existence of a block below the current in two steps

        let d = direction.as_array();
        let new_pos = (
            self.tentative_sand.0 + d[0],
            (self.tentative_sand.1 as isize + d[1]) as usize,
        );

        // ensure the floor exists at this new position lol
        self.map
            .entry(new_pos.0)
            .or_insert_with(BTreeMap::new)
            .insert(self.floor, Object::Wall);

        let Some(set) = self.map.get(&new_pos.0) else {
            return Err(());
        };

        let mut moved = true;

        match direction {
            Direction::Down => {
                // 2. vertical position strictly underneath current
                let Some((y, _)) = set.iter().find(|(y, _)| **y > self.tentative_sand.1) else {
                    return Err(());
                };
                if y - self.tentative_sand.1 <= 1 {
                    moved = false;
                } else {
                    self.tentative_sand.1 = y - 1;
                }
            }
            Direction::Left => {
                // only try moving one to the left down
                if set.get(&new_pos.1).is_none() {
                    self.tentative_sand = new_pos;
                } else {
                    moved = false;
                }
            }

            Direction::Right => {
                if set.get(&new_pos.1).is_none() {
                    self.tentative_sand = new_pos;
                } else {
                    moved = false;
                }
            }
        }

        match direction {
            Direction::Down => self.drop_sand(Direction::Left),
            Direction::Left => {
                if moved {
                    self.drop_sand(Direction::Down)
                } else {
                    self.drop_sand(Direction::Right)
                }
            }
            Direction::Right => {
                if moved {
                    self.drop_sand(Direction::Down)
                } else {
                    // add to hashmap
                    self.map
                        .get_mut(&self.tentative_sand.0)
                        .expect("HOW DOES THING COME TO REST WITHOUT PLATFORM UNDERNEATH")
                        .insert(self.tentative_sand.1, Object::Sand);
                    // cleanup seed
                    self.tentative_sand = (500, 0);
                    Ok(())
                }
            }
        }
    }
}

fn main() {
    // map of x, to depth
    let mut map: BTreeMap<isize, BTreeMap<usize, Object>> = BTreeMap::new();

    let mut floor = usize::MIN;

    std::fs::read_to_string("fourteen.txt")
        .unwrap()
        .lines()
        .for_each(|l| {
            let coords = l.split(" -> ").map(|coord| {
                let mut iter = coord.split(',').map(|n| n.parse::<usize>().unwrap());
                [iter.next().unwrap(), iter.next().unwrap()]
            });

            coords
                .clone()
                .zip(coords.skip(1))
                .for_each(|([start_x, start_y], [end_x, end_y])| {
                    if start_y > floor {
                        floor = start_y;
                    }
                    if end_y > floor {
                        floor = end_y;
                    }
                    // increment xs
                    if start_x != end_x {
                        for i in start_x.min(end_x)..=start_x.max(end_x) {
                            map.entry(i as isize)
                                .or_insert_with(BTreeMap::new)
                                .insert(start_y, Object::Wall);
                        }
                    }
                    // increment ys
                    else {
                        for i in start_y.min(end_y)..=start_y.max(end_y) {
                            map.entry(start_x as isize)
                                .or_insert_with(BTreeMap::new)
                                .insert(i, Object::Wall);
                        }
                    }
                });
        });

    floor += 2;

    // add floor to all widths
    map.values_mut().for_each(|v| {
        v.insert(floor, Object::Wall);
    });

    let mut board = Board {
        map,
        grains_in_rest: 0,
        floor,
        // tentative sand
        tentative_sand: (500, 0),
    };

    // loop invariant is basically that there exists no object at the same
    // horizontal location and lower vertical position as the current grain of sand

    // board.render(0..=600, 0..=200);

    let mut count = 0;

    loop {
        assert!(board.drop_sand(Direction::Down).is_ok());
        count += 1;
        if let Some(v) = board.map.get(&500) {
            if let Some(value) = v.get(&0) {
                dbg!(value);
                break;
            }
        }
    }
    board.render(0..=600, 0..=200);
    dbg!(count);
}
