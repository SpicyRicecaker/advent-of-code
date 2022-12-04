use std::ops::RangeInclusive;

fn main() {
    let out = std::fs::read_to_string("four.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let mut range = l
                .split(',')
                .map(|split| {
                    let mut iter = split.split('-').map(|num| num.parse::<u32>().unwrap());
                    iter.next().unwrap()..=iter.next().unwrap()
                })
                .collect::<Vec<RangeInclusive<u32>>>();

            range.sort_by(|a, b| a.start().cmp(b.start()));
            let mut iter = range.into_iter();

            (iter.next().unwrap(), iter.next().unwrap())
        })
        .fold(0, |acc, (left, right)| {
            if left.contains(right.start()) {
                acc + 1
            } else {
                acc
            }
        });

    println!("{out}");
}
