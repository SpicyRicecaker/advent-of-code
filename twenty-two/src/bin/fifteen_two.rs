use std::collections::HashSet;
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

// we also need a list of beacons, because border
fn get_invalid_from_row(
    row: isize,
    sensors: &[Sensor],
    beacon_coords: &HashSet<(isize, isize)>,
) -> HashSet<isize> {
    let mut invalid: HashSet<isize> = HashSet::new();

    for sensor in sensors {
        // add the distance from row to sensor
        let abs_y = sensor.y.abs_diff(row);
        let dist_for_x = sensor.manhattan_dist as isize - abs_y as isize;
        if dist_for_x >= 0 {
            // this is the breathing room left for x. We can invalidate all within the range
            for x in (sensor.x - dist_for_x)..=(sensor.x + dist_for_x) {
                if !beacon_coords.contains(&(x, row)) {
                    invalid.insert(x);
                }
            }
        }
    }
    invalid
}

fn main() {
    let r = Regex::new(r#"x=(-?\d+), y=(-?\d+)"#).unwrap();

    let (sensors, beacons): (Vec<_>, HashSet<_>) = std::fs::read_to_string("fifteen.txt")
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

            (
                Sensor {
                    x: sx,
                    y: sy,
                    manhattan_dist: sx.abs_diff(bx) + sy.abs_diff(by),
                },
                (bx, by),
            )
        })
        .unzip();

    // dbg!(v);

    let instant = Instant::now();

    // each row has 4_000_000 possible locations
    // each column has 4_000_000 possible locations

    for row in 0..=4_000_000 {
        let set = get_invalid_from_row(row, &sensors, &beacons);
        dbg!(set.len());
        if set.len() == 3_999_999 {
            let x = (0..=4_000_000).find(|i| !set.contains(i)).unwrap();
            println!("found beacon slot at x: {x}, y: {row}");
            println!("res: {}", x * row);
            break;
        }
        println!("processed: {:.2}%", (row as f64 / 4_000_000f64) * 100f64)
    }

    dbg!(instant.elapsed());
    // let res = get_invalid_from_row(10, &sensors, &beacons);
}
