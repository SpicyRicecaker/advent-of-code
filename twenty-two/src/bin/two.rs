fn play(p1: u32, p2: u32) -> u32 {
    if p1 == p2 {
        p1 + 1 + 3
    } else {
        let win = p1 == ((p2 + 1) % 3);
        if win {
            p1 + 1 + 6
        } else {
            p1 + 1
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("two.txt").unwrap();
    let output = input
        .lines()
        .map(|l| l.split_whitespace())
        .fold(0, |acc, mut item| {
            let first = item.next().unwrap();
            let second = item.next().unwrap();

            let them = match first {
                "A" => 0,
                "B" => 1,
                "C" => 2,
                _ => panic!(),
            };

            let me = match second {
                "X" => 0,
                "Y" => 1,
                "Z" => 2,
                _ => panic!(),
            };

            // dbg!(play(me, them));
            acc + play(me, them)
        });
    dbg!(output);
}
