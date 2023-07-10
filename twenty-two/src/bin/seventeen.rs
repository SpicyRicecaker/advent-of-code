use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let binding = std::fs::read_to_string("seventeen.txt").unwrap();
    let mut it_gust = binding
        .chars()
        .map(|c| match c {
            '>' => 1,
            '<' => -1,
            _ => unreachable!(),
        })
        .cycle();

    // how to represent shapes?
    // let's use a default cartesian graph: let some coordinate be centered at
    // (0,0) and other coordinates can be based off of that
    let mut it_shapes = [Shape::Line, Shape::Cross, Shape::J, Shape::I, Shape::Square]
        .into_iter()
        .cycle();

    let mut map: BTreeMap<i32, BTreeSet<i32>> = BTreeMap::new();
    (0..7).for_each(|x| {
        map.entry(x).or_insert_with(BTreeSet::new).insert(0);
    });

    // vector of coordinates seems reasonable for the final representation

    (0..2022).for_each(|_| {
        // first spawn the shape

        // get the highest y
        let y_highest = *map
            .values()
            .map(|set| set.iter().rev().next().unwrap())
            .max()
            .unwrap();
        // left edge is 2 units
        let x_spawn = 2;

        let mut s: Vec<_> = {
            let c = it_shapes.next().unwrap().get_coords();

            let y_diff = c[0][1] - c[1][1];
            // dbg!(y_diff);
            // get the difference between the leftmost and bottommost thing
            let y_spawn = y_highest + 4 + y_diff;

            c.into_iter()
                // 3 above the highest x
                .map(|[x, y]| [x + x_spawn, y + y_spawn])
                .collect()
        };
        // dbg!(&s);

        // gust, downward, until downward reaches a shape
        loop {
            // println!("begin:");
            // print_map_and_current(&s, &map);
            let gust = it_gust.next().unwrap();
            // dbg!(gust);
            let collision = s
                .iter()
                .map(|[x, y]| [x + gust, *y])
                .any(|[x, y]| !(0..7).contains(&x) || map.get(&x).unwrap().contains(&y));

            if !collision {
                s.iter_mut().for_each(|[x, _]| {
                    *x += gust;
                });
            }
            // println!("after gust:");
            // print_map_and_current(&s, &map);

            // try downward
            let collision = s
                .iter()
                .map(|[x, y]| [*x, y - 1])
                .any(|[x, y]| map.get(&x).unwrap().contains(&y));

            if !collision {
                s.iter_mut().for_each(|[_, y]| {
                    *y -= 1;
                });
                // println!("after drop:");
                // print_map_and_current(&s, &map);
            } else {
                break;
            }
        }
        // cement s into map
        s.into_iter().for_each(|[x, y]| {
            map.get_mut(&x).unwrap().insert(y);
        });
    });

    // find the highest y
    let y_highest = *map
        .values()
        .map(|set| set.iter().rev().next().unwrap())
        .max()
        .unwrap();
    dbg!(y_highest);
}

fn print_map_and_current(current: &[[i32; 2]], m: &BTreeMap<i32, BTreeSet<i32>>) {
    // get the highest and lowest y
    let mut highest_y = m.values().map(|x| x.iter().last().unwrap()).max().unwrap();
    current.iter().for_each(|[_, y]| {
        if y > highest_y {
            highest_y = y;
        }
    });

    for y in (0..=*highest_y).rev() {
        for x in 0..7 {
            if current.contains(&[x, y]) || m.get(&x).unwrap().contains(&y) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("----------------------");
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Line,
    Cross,
    J,
    I,
    Square,
}

impl Shape {
    fn get_coords(self) -> Vec<[i32; 2]> {
        // let shapes = r##"####

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
        // ##"##;

        // in the order of leftmost, then downmost, except for I
        match self {
            Shape::Line => vec![[0, 0], [1, 0], [2, 0], [3, 0]],
            Shape::Cross => vec![[0, 0], [1, -1], [1, 0], [1, 1], [2, 0]],
            Shape::J => vec![[0, 0], [1, 0], [2, 0], [2, 1], [2, 2]],
            Shape::I => vec![[0, 0], [0, -3], [0, -2], [0, -1]],
            Shape::Square => vec![[0, 0], [1, 0], [0, 1], [1, 1]],
        }
    }
}
