use std::ops::Range;

use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = std::fs::read_to_string("5.txt").unwrap();
    let mut it = s.lines();

    // first line is seeds
    let mut seeds = {
        let mut seeds: Vec<_> = it
            .next()
            .unwrap()
            .replace("seeds: ", "")
            .split_whitespace()
            // .inspect(|s| {
            //     dbg!(s);
            // })
            .map(|str| str.parse::<i64>().unwrap())
            .collect();
        let mut t: Vec<Range<i64>> = vec![];
        let mut i = 0_usize;
        while i < seeds.len() - 1 {
            t.push(seeds[i]..(seeds[i] + seeds[i + 1]));
            i += 2;
        }
        t
    };
    let mut seeds_next = vec![];

    let re = Regex::new(r##".* map:\n((?:\d+(?: |\n))+)"##).unwrap();

    re.captures_iter(&s).for_each(|matches| {
        let (ranges_from, ranges_to): (Vec<_>, Vec<_>) = matches[1]
            .lines()
            .map(|l| {
                let a = l
                    .split_whitespace()
                    .map(|str| str.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();

                // aoc format: to, from, len
                // my format : from, to
                (a[1]..(a[1] + a[2]), a[0]..(a[0] + a[2]))
            })
            .unzip();

        for seed in seeds.iter() {
            // seed range intersects range mappings
            for i in 0..ranges_from.len() {
                if let Some(o) = overlap(seed.clone(), ranges_from[i].clone()) {
                    seeds_next.push(shift(o, -ranges_from[i].start + ranges_to[i].start));
                }
            }

            // note places where the seed range does not come into
            // contact with other seed ranges
            seeds_next.extend_from_slice(&complement(seed.clone(), ranges_from.clone()));
        }

        std::mem::swap(&mut seeds, &mut seeds_next);
        seeds_next.clear();
    });

    seeds.sort_by(|a, b| a.start.cmp(&b.start));
    println!("{}", seeds[0].start);

    Ok(())
}

#[test]
fn test_overlap() {
    assert!(overlap(0..5, 1..4).is_some());
    assert!(overlap(0..5, -1..6).is_some());
    assert!(overlap(0..5, 3..6).is_some());
}

fn overlap(a: Range<i64>, b: Range<i64>) -> Option<Range<i64>> {
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

fn shift(a: Range<i64>, diff: i64) -> Range<i64> {
    (a.start + diff)..(a.end + diff)
}

#[test]
fn test_complement() {
    // lol
    assert_eq!(complement(0..2, vec![0..0]), vec![0..2]);
    assert_eq!(complement(0..10, vec![3..5, 6..9]), vec![0..3, 9..10, 5..6]);
    assert_eq!(complement(0..10, vec![0..10]), vec![]);
    assert_eq!(complement(0..5, vec![7..10]), vec![0..5]);
    assert_eq!(complement(5..10, vec![1..3]), vec![5..10]);
}

fn complement(a: Range<i64>, mut v: Vec<Range<i64>>) -> Vec<Range<i64>> {
    // find all as not in v
    // vs are guaranteed to be non-overlapping
    // vs are not guaranteed to be in order
    // vs are guaranteed to be greater than 1 len

    // output does not have to be in order
    let mut out = vec![];
    v.sort_by(|a, b| a.start.cmp(&b.start));
    if a.start < v[0].start {
        out.push(a.start..v[0].start.min(a.end));
    }
    if a.end > v.last().unwrap().end {
        out.push(v.last().unwrap().end.max(a.start)..a.end);
    }
    for i in 0..(v.len() - 1) {
        if v[i].end < v[i + 1].start {
            out.push(v[i].end..v[i + 1].start);
        }
    }

    out
}
