use std::collections::VecDeque;

fn main() {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let res: u32 = std::fs::read_to_string("1.txt")
        .unwrap()
        .lines()
        // .take(1)
        // .skip(4)
        .map(|l| {
            let mut num = 0;
            // find first num from left
            {
                let mut buf = String::new();
                for c in l.chars() {
                    if c.is_numeric() {
                        // dbg!(c as u8 + b'0');
                        num += (c as u8 - b'0') as u32;
                        break;
                    } else {
                        buf.push(c);
                        if let Some(pos) = digits.iter().position(|&w| buf.contains(w)) {
                            num += pos as u32 + 1;
                            break;
                        }
                    }
                    // println!("processed {c}, buf is {buf}, out is {out:#?}");
                }
            }
            num *= 10;
            // find second num from rtl
            {
                let mut buf: VecDeque<char> = VecDeque::new();
                for c in l.chars().rev() {
                    if c.is_numeric() {
                        num += (c as u8 - b'0') as u32;
                        break;
                    } else {
                        buf.push_front(c);
                        let s: String = Vec::from(buf.clone()).into_iter().collect::<String>();
                        // dbg!(&s);
                        if let Some(pos) = digits.iter().position(|&w| s.contains(w)) {
                            num += pos as u32 + 1;
                            break;
                        }
                    }
                    // println!("processed {c}, buf is {buf}, out is {out:#?}");
                }
            }

            // dbg!(num);
            num
        })
        .sum();

    println!("{res}");
}
