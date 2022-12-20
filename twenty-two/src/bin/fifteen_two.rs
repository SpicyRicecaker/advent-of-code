use std::time::Instant;

use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Beacon {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
struct Sensor {
    x: isize,
    y: isize,
    // it cannot be this distance, and because there's no ambiguity it also
    // invalidates the border
    manhattan_dist: usize,
}

const MAX: usize = 4000000;
const ONE_MORE: isize = (MAX + 1) as isize;

// we also need a list of beacons, because border
fn get_invalid_from_row(row: isize, sensors: &[Sensor]) -> (Vec<bool>, isize) {
    let mut invalid: Vec<bool> = vec![false; ONE_MORE as usize];
    let mut invalids = 0;

    for sensor in sensors {
        // add the distance from row to sensor
        let abs_y = sensor.y.abs_diff(row);
        let dist_for_x = sensor.manhattan_dist as isize - abs_y as isize;
        if dist_for_x >= 0 {
            // this is the breathing room left for x. We can invalidate all within the ra
            for x in (sensor.x - dist_for_x).max(0)..=(sensor.x + dist_for_x).min(MAX as isize) {
                if !invalid[x as usize] {
                    invalid[x as usize] = true;
                    invalids += 1;
                }
            }
        }
    }
    (invalid, invalids)
}

fn main() {
    let r = Regex::new(r#"x=(-?\d+), y=(-?\d+)"#).unwrap();

    let sensors: Vec<_> = std::fs::read_to_string("fifteen.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(idx, l)| {
            let mut r = r.captures_iter(l);

            // dbg!(idx, l);

            let g = r.next().unwrap();
            let (sx, sy) = (
                g[1].parse::<isize>().unwrap(),
                g[2].parse::<isize>().unwrap(),
            );

            let g = r.next().unwrap();
            let (bx, by) = (
                g[1].parse::<isize>().unwrap(),
                g[2].parse::<isize>().unwrap(),
            );

            Sensor {
                x: sx,
                y: sy,
                manhattan_dist: sx.abs_diff(bx) + sy.abs_diff(by),
            }
        })
        .collect();

    // dbg!(v);

    // each row has 4_000_000 possible locations
    // each column has 4_000_000 possible locations

    for row in 0..=(MAX as isize) {
        let instant = Instant::now();
        let (set, l) = get_invalid_from_row(row, &sensors);
        // dbg!(l);
        if l == MAX as isize {
            let x = set.into_iter().position(|p| !p).unwrap();
            println!("found beacon slot at x: {x}, y: {row}");
            println!("res: {}", x as isize * 4000000 + row);
            break;
        }
        println!("processed: {:.2}%", (row as f64 / MAX as f64) * 100f64);
        dbg!(instant.elapsed());
    }

    // let res = get_invalid_from_row(10, &sensors, &beacons);
}
