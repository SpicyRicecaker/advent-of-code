fn main() {
    // the draw function is called each second
    let _current: usize = 0;

    let mut cycles_completed = 0;
    let mut value: i32 = 1;

    let mut output = String::new();

    std::fs::read_to_string("ten.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .for_each(|l| {
            if ((value - 1) % 40..=(value + 1) % 40).contains(&((cycles_completed) % 40)) {
                output.push('#');
            } else {
                output.push('.');
            }
            if (cycles_completed + 1) % 40 == 0 {
                output.push('\n');
            }
            match l.len() {
                1 => {
                    // noop
                    cycles_completed += 1;
                }
                2 => {
                    cycles_completed += 1;
                    //
                    if ((value - 1) % 40..=(value + 1) % 40).contains(&((cycles_completed) % 40)) {
                        output.push('#');
                    } else {
                        output.push('.');
                    }
                    if (cycles_completed + 1) % 40 == 0 {
                        output.push('\n');
                    }

                    cycles_completed += 1;
                    value += l[1].parse::<i32>().unwrap();
                }
                _ => {
                    panic!()
                }
            }
        });
    println!("{output}");
}
