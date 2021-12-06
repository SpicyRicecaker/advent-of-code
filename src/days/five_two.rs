use std::cmp::Ordering;
#[derive(Debug)]
struct Coord {
    x: u32,
    y: u32,
}

pub fn run(state: crate::State) {
    // for i in 5..1 {
    //     println!("{}", i);
    // }
    // todo!();
    // preallocate 1000x1000 square
    let mut square = vec![vec![0_u32; 1000]; 1000];
    // let mut square = vec![vec![0_u32; 1000]; 1000];
    println!("got past allocation");
    // ((u32, u32), (u32, u32))
    state
        .input("input5.txt")
        .lines()
        .map(|l| {
            let mut coords_iter = l.split(" -> ").map(|coord| {
                let mut coord_iter = coord.split(',').map(|n| n.parse::<u32>().unwrap());
                Coord {
                    x: coord_iter.next().unwrap(),
                    y: coord_iter.next().unwrap(),
                }
            });
            (coords_iter.next().unwrap(), coords_iter.next().unwrap())
        })
        .for_each(|(begin, end)| {
            let dy = end.y.cmp(&begin.y);
            let dx = end.x.cmp(&begin.x);
            match (dy, dx) {
                (Ordering::Equal, dx) => match dx {
                    Ordering::Less => {
                        for x in (end.x..=begin.x).rev() {
                            square[begin.y as usize][x as usize] += 1;
                        }
                    }
                    Ordering::Greater => {
                        for x in begin.x..=end.x {
                            square[begin.y as usize][x as usize] += 1;
                        }
                    }
                    _ => {}
                },
                (dy, Ordering::Equal) => match dy {
                    Ordering::Less => {
                        for y in (end.y..=begin.y).rev() {
                            square[y as usize][begin.x as usize] += 1;
                        }
                    }
                    Ordering::Greater => {
                        for y in begin.y..=end.y {
                            square[y as usize][begin.x as usize] += 1;
                        }
                    }
                    _ => {}
                },
                // otherwise, it's a diagonal
                _ => {
                    // println!("{:?}->{:?}", begin, end);
                    let dy = match dy {
                        Ordering::Less => -1_i32,
                        Ordering::Greater => 1,
                        _ => unreachable!(),
                    };

                    let dx = match dx {
                        Ordering::Less => -1_i32,
                        Ordering::Greater => 1,
                        _ => unreachable!(),
                    };
                    let mut currentx = begin.x as i32;
                    let mut currenty = begin.y as i32;
                    loop {
                        square[currenty as usize][currentx as usize] += 1;
                        if currentx as u32 == end.x {
                            break;
                        } else {
                            currentx += dx;
                            currenty += dy;
                        }
                    }
                }
            };
        });
    // println!("-------begin--------");
    // for row in &square {
    //     for column in row {
    //         if *column == 0 {
    //             print!(".");
    //         } else {
    //             print!("{}", column);
    //         }
    //     }
    //     println!();
    // }
    // println!("-----end----------");

    print!("got past adding lines");
    let mut sum = 0;
    square.iter().flatten().for_each(|&n| {
        if n >= 2 {
            sum += 1
        }
    });
    println!("number of danger spots: {}", sum);
}
