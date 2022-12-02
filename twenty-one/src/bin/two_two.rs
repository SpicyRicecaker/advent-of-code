struct Submarine {
    aim: u32,
    horizontal: u32,
    depth: u32,
}

fn main() {
    let state = advent_of_code_2021::config();
    let mut submarine = Submarine {
        aim: 0,
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
        .for_each(|(command, value)| {
            // dbg!(command, value);
            match command {
                "forward" => {
                    submarine.horizontal += value;
                    submarine.depth += value * submarine.aim
                }
                "down" => {
                    submarine.aim += value;
                }
                "up" => {
                    submarine.aim -= value;
                }
                _ => panic!("unexpected movement command"),
            }
        });

    let res = submarine.horizontal * submarine.depth;

    println!("{}", res);
}
