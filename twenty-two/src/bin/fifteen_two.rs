use rayon::prelude::*;
use regex::{CaptureMatches, Regex};


use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct Sensor {
    x: i32,
    y: i32,
    // it cannot be this distance, and because there's no ambiguity it also
    // invalidates the border
    m_d: u32,
}

const MAX: u32 = 4_000_000;

// we also need a list of beacons, because border
fn get_invalid_from_row(row: i32, sensors: &[Sensor]) -> Option<u32> {
    let mut intervals: Vec<(i32, i32)> = sensors
        .iter()
        .filter_map(|sensor| {
            // dbg!(sensor);
            let distance_to_row = row.abs_diff(sensor.y);
            // dbg!(distance_to_row);
            if distance_to_row <= sensor.m_d {
                let distance_left = sensor.m_d - distance_to_row;
                // dbg!(distance_left);
                Some((
                    (sensor.x - distance_left as i32).max(0),
                    (sensor.x + distance_left as i32).min(MAX as i32),
                ))
            } else {
                None
            }
        })
        .collect();

    intervals.sort();
    // dbg!(&intervals);

    let mut x = vec![];

    let last_interval = intervals
        .into_iter()
        .reduce(|(x_1s, x_1e), (x_2s, x_2e)| {
            if x_2s <= x_1e + 1 {
               (x_1s, x_2e.max(x_1e))
            } else {
                x.push((x_1s, x_1e));
                (x_2s, x_2e)
            }
        })
        .unwrap();

    x.push(last_interval);

    // dbg!(&x);

    if x.len() == 2 {
        Some(((x[1].0 - x[0].1) / 2 + x[0].1) as u32)
    } else {
        // check if the first or last edge is not included
        if !(x[0].0..=x[0].1).contains(&0) {
            Some(0)
        } else if !(x[0].0..=x[0].1).contains(&(MAX as i32)) {
            Some(MAX)
        } else {
            None
        }
    }
}

fn main() {
    let r = Regex::new(r#"(-?\d+)"#).unwrap();

    let c = move |m: &mut CaptureMatches| m.next().unwrap()[0].parse::<i32>().unwrap();

    let sensors: Vec<_> = std::fs::read_to_string("fifteen.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let mut r = r.captures_iter(l);
            let r = &mut r;

            let (sx, sy) = (c(r), c(r));

            let (bx, by) = (c(r), c(r));

            Sensor {
                x: sx,
                y: sy,
                m_d: sx.abs_diff(bx) + sy.abs_diff(by),
            }
        })
        .collect();
    // dbg!(&sensors);

    // let (sender, receiver) = sync_channel(0);

    // adding the counter code below slows the code by like 700x lol

    // let mut counter = 0;
    // thread::spawn(move || loop {
    //     if receiver.try_recv().is_ok() {
    //         counter += 1;
    //         println!("processed: {:.2}%", (counter as f64 / MAX as f64) * 100f64);
    //     }
    // });

    let instant = Instant::now();

    let (row, x) = (0..=MAX as i32)
        .into_par_iter()
        .find_map_any(move |row| {
        // .find_map(move |row| {
            // let t = sender.clone();
            // t.send(1).unwrap();
            get_invalid_from_row(row, &sensors).map(|x| (row, x))
        })
        .unwrap();

    dbg!((row, x));
    dbg!(x as usize * 4_000_000usize + row as usize);

    dbg!(instant.elapsed());

    // let res = get_invalid_from_row(10, &sensors, &beacons);
}
