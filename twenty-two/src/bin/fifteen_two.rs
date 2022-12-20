use std::time::Instant;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Sensor {
    x: i32,
    y: i32,
    // it cannot be this distance, and because there's no ambiguity it also
    // invalidates the border
    manhattan_dist: u32,
}

const MAX: u32 = 4_000_000;
const ONE_MORE: u32 = MAX + 1;

// we also need a list of beacons, because border
fn get_invalid_from_row<'a>(
    row: i32,
    sensors: &[Sensor],
    invalid: &'a mut Vec<bool>,
) -> (&'a mut Vec<bool>, u32) {
    let mut invalids = 0;

    for sensor in sensors {
        // add the distance from row to sensor
        let distance_to_row = row.abs_diff(sensor.y);

        // if the absolute value
        if distance_to_row > sensor.manhattan_dist {
            continue;
        }

        let dist_for_x = (sensor.manhattan_dist - distance_to_row) as i32;
        // this is the breathing room left for x. We can invalidate all within the ra
        for x in (sensor.x - dist_for_x).max(0)..=(sensor.x + dist_for_x).min(MAX as i32) {
            if !invalid[x as usize] {
                invalid[x as usize] = true;
                invalids += 1;
                if invalids > MAX {
                    return (invalid, invalids);
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
        .map(|l| {
            let mut r = r.captures_iter(l);

            let g = r.next().unwrap();
            let (sx, sy) = (g[1].parse::<i32>().unwrap(), g[2].parse::<i32>().unwrap());

            let g = r.next().unwrap();
            let (bx, by) = (g[1].parse::<i32>().unwrap(), g[2].parse::<i32>().unwrap());

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

    let invalid: Vec<bool> = vec![false; ONE_MORE as usize];

    for row in 0..=MAX as i32 {
        let instant = Instant::now();
        let mut invalid = invalid.clone();
        let (set, l) = get_invalid_from_row(row, &sensors, &mut invalid);
        dbg!(l);
        if l == MAX {
            let x = set.iter().position(|p| !*p).unwrap();
            println!("found beacon slot at x: {x}, y: {row}");
            println!("res: {}", x as u32 * 4000000 + row as u32);
            break;
        }
        println!("processed: {:.2}%", (row as f64 / MAX as f64) * 100f64);
        dbg!(instant.elapsed());
        dbg!("estimated time left:", instant.elapsed() * MAX);
    }

    // let res = get_invalid_from_row(10, &sensors, &beacons);
}
