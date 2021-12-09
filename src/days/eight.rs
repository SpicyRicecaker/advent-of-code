fn get_digit_from_len(len: usize) -> Option<u8> {
    match len {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}

pub fn run(state: crate::State) {
    let mut count = 0;
    state
        .input("input8.txt")
        .lines()
        .for_each(|l| { 
            l.split(" | ").nth(1).unwrap().split_whitespace().for_each(|m| {
                if get_digit_from_len(m.len()).is_some() {
                    count += 1;
                }
            });
        });
    println!("unique numbers: {}", count);
}
