use crate::State;

struct Submarine {
    horizontal: u32,
    depth: u32,
}

pub fn run(state: State) {
    let mut submarine = Submarine {
        horizontal: 0,
        depth: 0,
    };
    // Get input
    state
        .input("input2.txt")
        .split("\r\n")
        .map(|c| {
            let mut iter = c.split_whitespace();
            (
                iter.next().unwrap(),
                iter.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .for_each(|(command, value)| match command {
            "forward" => submarine.horizontal += value,
            "down" => {
                submarine.depth += value;
            }
            "up" => {
                submarine.depth -= value;
            }
            _ => panic!("unexpected movement command"),
        });

    let res = submarine.horizontal * submarine.depth;

    println!("{}", res);
}
