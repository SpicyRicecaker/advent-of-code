fn main() {
    let val = std::fs::read_to_string("three.txt")
        .unwrap()
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .fold(0, |acc, (top, bot)| {
            let top = top.chars().into_iter().fold(0usize, |acc, x| {
                acc | if x.is_lowercase() {
                    0x1 << (x as usize - b'a' as usize)
                } else {
                    (0x1 << (x.to_lowercase().next().unwrap() as usize - b'a' as usize)) << 26
                }
            });
            let bot = bot.chars().into_iter().fold(0usize, |acc, x| {
                acc | if x.is_lowercase() {
                    0x1 << (x as usize - b'a' as usize)
                } else {
                    (0x1 << (x.to_lowercase().next().unwrap() as usize - b'a' as usize)) << 26
                }
            });
            // println!("{:b}", top);
            // println!("{:b}", bot);
            // println!("e: {:b}", top & bot);
            // we need to find how much in common the bit is shifted
            // one idea is to just keep bit shifting to the right until we reach
            // 0. It's basically log base 2 of the number? 
            let mut r = top & bot;
            let mut counter = 0;
            while r != 0 {
                r /= 2;
                counter += 1;
            }

            acc + counter
        });
    println!("{val}");
}
