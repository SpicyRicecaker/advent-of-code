use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let re: Regex = Regex::new(r#"Card\s+\d+: (.*)"#)?;

    let mut S: HashMap<usize, usize> = HashMap::new();

    let f = std::fs::read_to_string("4.txt")?;
    let lines: Vec<_> = f.lines().collect();
    let n = lines.len();
    lines
        .into_iter()
        .map(|l| {
            re.captures(l)
                .unwrap()
                .iter()
                .nth(1)
                .unwrap()
                .unwrap()
                .as_str()
        })
        .enumerate()
        .rev()
        .for_each(|(i, l)| {
            // dbg!(i);
            let mut it = l.split('|').map(|split| {
                split
                    .split_whitespace()
                    .map(|str_num| str_num.parse::<u32>().unwrap())
            });
            let hashset: HashSet<u32> = it.next().unwrap().collect();

            if i == n - 1 {
                S.insert(i, 0);
            } else {
                let w_i = it.next().unwrap().filter(|x| hashset.contains(x)).count();
                let mut s = w_i;
                for j in 1..=w_i {
                    // guaranteed not to panic
                    s += S[&(i + j)];
                }
                S.insert(i, s);
            }
        });

    // dbg!(&S);

    println!("{}", /* the generated cards */ S.values().sum::<usize>() + /* the cards we already have*/ S.len());

    Ok(())
}
