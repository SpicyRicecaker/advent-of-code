use std::collections::HashMap;

struct BitCount {
    one_count: u32,
    zero_count: u32,
}

pub fn run(state: crate::State) {
    // Counts number of 1s
    let mut hashmap: HashMap<usize, BitCount> = HashMap::new();

    state.input("input3.txt").split_whitespace().for_each(|l| {
        l.chars().enumerate().for_each(|(i, c)| {
            let entry = hashmap.entry(i).or_insert(BitCount {
                one_count: 0,
                zero_count: 0,
            });
            match c {
                '0' => {
                    entry.zero_count += 1;
                }
                '1' => {
                    entry.one_count += 1;
                }
                _ => panic!("unexpected char"),
            }
        });
    });
    let mut gamma = String::new();
    let mut epsilon = String::new();
    (0..hashmap.len()).for_each(|n| {
        let entry = hashmap.get(&n).unwrap();
        if entry.one_count > entry.zero_count {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    });
    println!("epsilon & gamma are `{}`", to_num_from_binary(&gamma) * to_num_from_binary(&epsilon));
}

fn to_num_from_binary(str: &str) -> u32 {
    str.chars().rev().enumerate().fold(0, |acc, (i, c)| {
        acc + 2_u32.pow(i as u32) * c.to_digit(10).unwrap() as u32
    })
}
