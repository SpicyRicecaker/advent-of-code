#[derive(Debug)]
struct Display {
    input: Vec<String>,
    output: Vec<String>,
}

fn main() {
    // num of times strings of 1, 4, 7, 8 length appear in output
    // 2, 4, 3, 7

    let out = std::fs::read_to_string("res/eight.txt")
        .unwrap()
        .lines()
        .map(|l| {
            // dbg!(l);
            let mut iter = l.split('|').map(|s| {
                // dbg!(s);
                s.split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            });
            Display {
                input: iter.next().unwrap(),
                output: iter.next().unwrap(),
            }
        })
        .fold(0, |acc, item| {
            // dbg!(&item);
            // dbg!(acc);
            acc + item.input.iter().fold(0, |acc, item| match item.len() {
                2 | 4 | 3 | 7 => {
                    // dbg!(item);
                    acc + 1
                },
                _ => acc,
            })
        });
    dbg!(out);
}
