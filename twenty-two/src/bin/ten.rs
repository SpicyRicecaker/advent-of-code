fn main() {
    let during: [u32; 6] = [20, 60, 100, 140, 180, 220];
    let mut current: usize = 0;

    let mut cycles_completed = 0;
    let mut value: i32 = 1;

    let mut total = 0;

    std::fs::read_to_string("ten.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .for_each(|l| {
            match l.len() {
                1 => {
                    // noop
                    cycles_completed += 1;
                }
                2 => {
                    cycles_completed += 1;
                    if current < during.len() && cycles_completed == during[current]- 1 {
                        total += value * during[current] as i32;
                        current += 1;
                    }
                    cycles_completed += 1;
                    value += l[1].parse::<i32>().unwrap();
                }
                _ => {
                    panic!()
                }
            }
            if current < during.len() && cycles_completed == during[current] - 1 {
                total += value * during[current] as i32;
                current += 1;
            }
            dbg!(value);
            dbg!(total);
        });
    println!("{total}");
}
