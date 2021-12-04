use std::cmp::Ordering;

struct BitCount {
    one_count: u32,
    zero_count: u32,
}

// Copy-pasted code
// Could probably use a func as a parameter to return different values depending on the flag

fn recurse(
    master: Vec<String>,
    current: usize,
    max: u32,
    criteria: &dyn Fn(BitCount, Vec<String>, Vec<String>) -> Vec<String>,
) -> String {
    // If there are 1 lines, return
    if master.len() == 1 {
        return master.into_iter().next().unwrap();
    };

    let mut zeros = Vec::new();
    let mut ones = Vec::new();

    let mut bitcount: BitCount = BitCount {
        one_count: 0,
        zero_count: 0,
    };

    // Find the most significant bit at position current
    master.into_iter().for_each(|s| {
        //      Push to most significant pile
        match s.chars().nth(current).unwrap() {
            '0' => {
                bitcount.zero_count += 1;
                zeros.push(s);
            }
            '1' => {
                bitcount.one_count += 1;
                ones.push(s);
            }
            _ => panic!("unexpected char"),
        }
    });
    let res = criteria(bitcount, zeros, ones);
    recurse(res, current + 1, max, criteria)
}

pub fn run(state: crate::State) {
    let string = state.input("input3test.txt");
    let length = string.lines().take(1).next().unwrap().chars().count();

    // Problem: debuggin the returning of closures is gnarly sometimes
    let oxygen_most_common_criteria = move |bitcount: BitCount, zeros, ones| {
        match bitcount.one_count.cmp(&bitcount.zero_count) {
            // if one count is the most common or takes prio
            Ordering::Equal | Ordering::Greater => ones,
            // zeros
            Ordering::Less => zeros,
        }
    };
    let co2_least_common_criteria = move |bitcount: BitCount, zeros, ones| {
        match bitcount.zero_count.cmp(&bitcount.one_count) {
            // if zero count least common or takes prio
            Ordering::Equal | Ordering::Less => zeros,
            // ones
            Ordering::Greater => ones,
        }
    };

    let oxygen = recurse(
        string
            .lines()
            .map(|c| c.to_string())
            .collect::<Vec<String>>(),
        0,
        (length - 1) as u32,
        &oxygen_most_common_criteria,
    );

    let co2 = recurse(
        string
            .lines()
            .map(|c| c.to_string())
            .collect::<Vec<String>>(),
        0,
        (length - 1) as u32,
        &co2_least_common_criteria,
    );

    dbg!(&oxygen);

    dbg!(&co2);

    println!(
        "life support rating of the submarine is `{}`",
        to_num_from_binary(&oxygen) * to_num_from_binary(&co2)
    );
}

pub fn to_num_from_binary(str: &str) -> u32 {
    str.chars().rev().enumerate().fold(0, |acc, (i, c)| {
        acc + 2_u32.pow(i as u32) * c.to_digit(10).unwrap() as u32
    })
}
