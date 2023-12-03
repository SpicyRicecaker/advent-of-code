use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // loop through, add all numbers to registry
    // have a next_num function which goes to next number in input

    let map: Vec<Vec<char>> = std::fs::read_to_string("3.txt")?
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    // dbg!(&map);

    let mut sum = 0;

    let mut i = 0;
    let mut j = 0;
    while i < map.len() {
        while j < map[0].len() {
            // print!("{},", map[i][j]);
            // goto next i,j
            if !map[i][j].is_numeric() {
                j += 1;
                continue;
            }
            // let start = i;
            // let mut end = i;

            // grab num
            let mut num = String::new();
            let mut connected = false;
            loop {
                // print!("{}", map[i][j]);
                num.push(map[i][j]);
                // look around
                if !connected {
                    'a: for dy in -1..=1 {
                        for dx in -1..=1 {
                            // let dbg_top = dy == 1 && dx == 0 && j == 3;
                            // if dbg_top {
                            //     println!("hello mom {}", map[i][j]);
                            // }

                            if dy == 0 && dx == 0 {
                                continue;
                            }

                            let newi: i32 = i as i32 + dy;
                            let newj: i32 = j as i32 + dx;
                            // if dbg_bot_left {
                            //     println!("{newi},{newj}");
                            // }
                            // ensure dy dx are in range
                            if !(0..(map.len() as i32)).contains(&newi)
                                || !(0..(map[0].len() as i32)).contains(&newj)
                            {
                                continue;
                            }
                            let newi = newi as usize;
                            let newj = newj as usize;

                            if map[newi][newj] != '.' && !map[newi][newj].is_numeric() {
                                connected = true;
                                break 'a;
                            }
                        }
                    }
                }
                j += 1;
                if j >= map[0].len() || !map[i][j].is_numeric() {
                    j -= 1;
                    break;
                }
            }
            // println!("ending off on {j}");
            // println!("num is {num}");
            // convert num into actual
            if connected {
                sum += num.parse::<u32>().unwrap();
            }
            j += 1;
        }
        // println!();
        j = 0;
        i += 1;
    }

    println!("{sum}");

    Ok(())
}
