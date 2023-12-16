use std::ops::Range;

use regex::Regex;

#[test]
fn test_overlap() {
    assert!(overlap(0..5, 1..4).is_some());
    assert!(overlap(0..5, -1..6).is_some());
    assert!(overlap(0..5, 3..6).is_some());
}

fn overlap(a: Range<i32>, b: Range<i32>) -> Option<Range<i32>> {
    if a.contains(&b.start) && a.contains(&b.end) {
        return Some(b);
    }

    if a.contains(&b.start) {
        return Some(b.start..a.end);
    }

    if a.contains(&b.end) {
        return Some(a.start..b.end + 1);
    }

    if b.contains(&a.start) && b.contains(&a.end) {
        return Some(a);
    }

    None
}

fn shift(a: Range<i32>, diff: i32) -> Range<i32> {
    (a.start + diff)..(a.end + diff)
}

#[test]
fn test_complement() {
    // lol
    assert_eq!(complement(0..2, vec![0..0]), vec![0..2]);
    assert_eq!(complement(0..10, vec![3..5, 6..9]), vec![0..3, 9..10, 5..6]);
    assert_eq!(complement(0..10, vec![0..10]), vec![]);
}

fn complement(a: Range<i32>, mut v: Vec<Range<i32>>) -> Vec<Range<i32>> {
    // find all as not in v
    // vs are guaranteed to be non-overlapping
    // vs are not guaranteed to be in order
    // vs are guaranteed to be greater than 1 len

    // output does not have to be in order
    let mut out = vec![];
    v.sort_by(|a, b| a.start.cmp(&b.start));
    if a.start < v[0].start {
        out.push(a.start..v[0].start);
    }
    if a.end > v.last().unwrap().end {
        out.push(v.last().unwrap().end..a.end);
    }
    for i in 0..(v.len() - 1) {
        if v[i].end < v[i + 1].start {
            out.push(v[i].end..v[i + 1].start);
        }
    }

    out
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = std::fs::read_to_string("5-1e.txt").unwrap();
    let mut it = s.lines();

    // first line is seeds
    let mut seeds: Vec<_> = it
        .next()
        .unwrap()
        .replace("seeds: ", "")
        .split_whitespace()
        // .inspect(|s| {
        //     dbg!(s);
        // })
        .map(|str| str.parse::<i32>().unwrap())
        .collect();
    let mut seeds_2: Vec<Range<i32>> = vec![];
    let mut i = 0_usize;
    while i < seeds.len() {
        seeds_2.push((seeds[i]..(seeds[i] + seeds[i + 1])));
        i += 2;
    }
    let mut seeds = seeds_2;
    println!("{:#?} seeds", seeds);
    let mut seeds_2: Vec<Range<i32>> = vec![];

    let re = Regex::new(r##".* map:\n((?:\d+(?: |\n))+)"##).unwrap();

    let mut dbg_i = 0;
    re.captures_iter(&s).for_each(|matches| {
        // dbg!("new round DBG");
        let ranges: Vec<(Range<i32>, Range<i32>)> = matches[1]
            .lines()
            .map(|l| {
                let a = l
                    .split_whitespace()
                    .map(|str| str.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();

                // could binary search here but input is small
                // to, from, len
                // -> from, to, len
                (a[1]..(a[1] + a[2]), a[0]..(a[0] + a[2]))
            })
            .collect();

        for i in 0..seeds.len() {
            for j in 0..ranges.len() {
                if let Some(o) = overlap(seeds[i].clone(), ranges[j].0.clone()) {
                    seeds_2.push(shift(o, ranges[j].1.start - ranges[j].0.start));
                }
            }
            // find which part of the seed has not been touched
            for r in complement(
                seeds[i].clone(),
                ranges.iter().map(|(s, _)| s.clone()).collect(),
            ) {
                // dbg!(&r);
                seeds_2.push(r);
            }
        }

        std::mem::swap(&mut seeds, &mut seeds_2);
        seeds_2.clear();
        // if dbg_i == 0 {
        //     dbg!(&seeds);
        // }

        // println!(
        //     "seeds after pass: {:#?}, : {:#?}",
        //     matches[0].to_string(),
        //     seeds
        // );
        // dbg_i += 1;
    });

    dbg!(&seeds);
    seeds.sort_by(|a, b| a.start.cmp(&b.start));
    dbg!(&seeds[0]);

    Ok(())
}
