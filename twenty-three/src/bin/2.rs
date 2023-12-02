use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let res: usize = std::fs::read_to_string("2.txt")?
        .lines()
        .enumerate()
        .map(|(i, s)| (i + 1, s))
        .filter(|(_, l)| {
            let mut map: HashMap<&str, u32> = HashMap::new();
            l.split(':').nth(1).unwrap().split(';').for_each(|game| {
                game.split(',').for_each(|cubes| {
                    let mut it = cubes.split_whitespace();
                    let num = it.next().unwrap().parse::<u32>().unwrap();
                    let color = it.next().unwrap();
                    let e = map.entry(color).or_insert(0);
                    *e = (*e).max(num);
                });
            });
            map["red"] <= 12 && map["green"] <= 13 && map["blue"] <= 14
        })
        .map(|(i, _)| i)
        .sum();
    println!("{res}");
    Ok(())
}
