use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

struct Board {
    map: HashMap<usize, HashSet<usize>>,
    grains_in_rest: usize,
    // x, y
    tentative_sand: [usize; 2],
}

impl Board {
    fn render(&self, domain: RangeInclusive<usize>, range: RangeInclusive<usize>) {
        range.for_each(|y| {
            domain
                .clone()
                .map(|x| {
                    let Some(set) = self.map.get(&x) else {
                        return "."; 
                    };

                    if set.contains(&y) {
                        "#"
                    } else {
                        "."
                    }
                })
                .for_each(|c| {
                    print!("{}", c);
                });
            println!();
        })
    }
}

fn main() {
    // map of x, to depth
    let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();

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
                    // increment xs
                    if start_x != end_x {
                        for i in start_x.min(end_x)..=start_x.max(end_x) {
                            map.entry(i).or_insert_with(HashSet::new).insert(start_y);
                        }
                    }
                    // increment ys
                    else {
                        for i in start_y.min(end_y)..=start_y.max(end_y) {
                            map.entry(start_x).or_insert_with(HashSet::new).insert(i);
                        }
                    }
                });
        });

    let board = Board {
        map,
        grains_in_rest: 0,
        // tentative sand
        tentative_sand: [500, 0],
    };
    board.render(494..=503, 0..=9);
}
