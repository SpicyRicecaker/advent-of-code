use std::{collections::HashSet, error::Error};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let re: Regex = Regex::new(r#"Card\s+\d+: (.*)"#)?;

    let res: u32 = std::fs::read_to_string("4.txt")?
        .lines()
        .map(|l| {
            re.captures(l)
                .unwrap()
                .iter()
                .nth(1)
                .unwrap()
                .unwrap()
                .as_str()
        })
        .map(|l| {
            let mut it = l.split('|').map(|split| {
                split
                    .split_whitespace()
                    .map(|str_num| str_num.parse::<u32>().unwrap())
            });
            let hashset: HashSet<u32> = it.next().unwrap().collect();

            let prize = it.next().unwrap().fold(0, |acc, x| {
                if hashset.contains(&x) {
                    if acc != 0 {
                        acc * 2
                    } else {
                        1
                    }
                } else {
                    acc
                }
            });

            prize
        })
        .sum();

    println!("{}", res);

    Ok(())
}
