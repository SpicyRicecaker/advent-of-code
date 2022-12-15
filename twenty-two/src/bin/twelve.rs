const DXDY: [[i32; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Path {
    count: u32,
    traveled: Vec<Vec<bool>>,
}

fn main() {
    // up, down, left or right all can work

    // receive the inputs. let the starting position be S and ending position be E
    let mut starting: [usize; 2] = [0, 0];
    let mut ending: [usize; 2] = [0, 0];
    let b: Vec<Vec<_>> = std::fs::read_to_string("twelve.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(|(column, c)| match c {
                    'S' => {
                        starting[0] = column;
                        starting[1] = row;
                        0
                    }
                    'E' => {
                        ending[0] = column;
                        ending[1] = row;
                        25
                    }
                    c => (c as u8 - b'a') as u32,
                })
                .collect()
        })
        .collect();

    // the elevation we are going to can be at most 1 higher

    // println!("BOARD");
    // for y in 0..b.len() {
    //     for x in 0..b[0].len() {
    //         print!("{}", (b[y][x] as u8 + b'a') as char);
    //     }
    //     println!();
    // }
    // println!();
    // println!();
    // println!();

    let traveled = vec![vec![false; b[0].len()]; b.len()];
    let mut paths = vec![];

    // dbg!(starting);
    // dbg!(ending);

    run(0, starting, ending, traveled, &b, &mut paths);

    paths.sort();

    // we need to find the set of all possible paths toward somewhere, without
    // repeats
    let r = &paths[0];

    // println!("{}", r.count);
    // for y in 0..r.traveled.len() {
    //     for x in 0..r.traveled[0].len() {
    //         print!("{}", if r.traveled[y][x] { 1 } else { 0 });
    //     }
    //     println!();
    // }
}

fn run(
    count: u32,
    current: [usize; 2],
    destination: [usize; 2],
    mut traveled: Vec<Vec<bool>>,
    board: &Vec<Vec<u32>>,
    paths: &mut Vec<Path>,
) {
    // dbg!(&traveled);
    traveled[current[1]][current[0]] = true;

    if current == destination {
        paths.push(Path {
            count,
            traveled: traveled.clone(),
        });
        return;
    }

    DXDY.into_iter().for_each(|[dx, dy]| {
        let new: [i32; 2] = [current[0] as i32 + dx, current[1] as i32 + dy];

        // dbg!(new);

        if (0..(board.len() as i32)).contains(&new[1])
            && (0..(board[0].len() as i32)).contains(&new[0])
            && !traveled[new[1] as usize][new[0] as usize]
        {
            let new = [new[0] as usize, new[1] as usize];
            if board[new[1]][new[0]] <= board[current[1]][current[0]] + 1 {
                run(count + 1, new, destination, traveled.clone(), board, paths);
            }
        }
    });
}
