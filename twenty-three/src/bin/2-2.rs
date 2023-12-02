use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let res: u32 = std::fs::read_to_string("2.txt")?
        .lines()
        .map(|l| {
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
            map["red"] * map["green"] * map["blue"]
        }).sum();
    println!("{res}");
    Ok(())
}
