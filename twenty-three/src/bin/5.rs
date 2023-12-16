use std::ops::Range;

use regex::Regex;

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
    let s = std::fs::read_to_string("5.txt").unwrap();
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
        .map(|str| str.parse::<i64>().unwrap())
        .collect();
    let mut seeds_2: Vec<Range<i64>> = vec![];
    let mut i = 0_usize;
    while i < seeds.len() {
        seeds_2.push((seeds[i]..(seeds[i] + seeds[i + 1])));
        i += 2;
    }
    let mut seeds = seeds_2;

    let re = Regex::new(r##".* map:\n((?:\d+(?: |\n))+)"##).unwrap();

    let mut v_ranges: Vec<Vec<(Range<i64>, Range<i64>)>> = vec![];
    re.captures_iter(&s).for_each(|matches| {
        // dbg!("new round DBG");
        let ranges: Vec<(Range<i64>, Range<i64>)> = matches[1]
            .lines()
            .map(|l| {
                let a = l
                    .split_whitespace()
                    .map(|str| str.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();

                // could binary search here but input is small
                // to, from, len
                // -> from, to, len
                (a[1]..(a[1] + a[2]), a[0]..(a[0] + a[2]))
            })
            .collect();
        v_ranges.push(ranges);
    });

    let mut smallest_loc = std::i64::MAX;

    let total_seed_ranges = seeds.len();
    dbg!(total_seed_ranges);
    for seed_range in seeds.iter() {
        let seeds_in_this_range = seed_range.end - seed_range.start;
        dbg!(seeds_in_this_range);

        for seed in seed_range.clone() {
            let mut seed = seed;

            for ranges in v_ranges.iter() {
                for range in ranges {
                    if range.0.contains(&seed) {
                        seed = (seed - range.0.start) + range.1.start;
                        break;
                    }
                }
            }

            if seed < smallest_loc {
                smallest_loc = seed;
            }
        }
    }

    dbg!(smallest_loc);

    Ok(())
}
