fn to_binary(s: &str) -> usize {
    s.chars().into_iter().fold(0usize, |acc, x| {
        acc | if x.is_lowercase() {
            0x1 << (x as usize - b'a' as usize)
        } else {
            (0x1 << (x.to_lowercase().next().unwrap() as usize - b'a' as usize)) << 26
        }
    })
}

fn main() {
    let v = std::fs::read_to_string("three.txt").unwrap();
    let val = v.lines().collect::<Vec<&str>>();

    let res = (0..val.len()).step_by(3).fold(0, |acc, i| {
        let one = to_binary(val[i]);
        let two = to_binary(val[i + 1]);
        let three = to_binary(val[i + 2]);

        let mut r = one & two & three;
        let mut counter = 0;
        while r != 0 {
            r /= 2;
            counter += 1;
        }

        acc + counter
    });
    println!("{res}");
}
