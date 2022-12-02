use std::collections::{HashMap, HashSet};

// digits are not in order

// we use the digits with unique lengths
fn get_digit_from_len(len: usize) -> Option<u8> {
    match len {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}

fn main() {
    let state = advent_of_code_2021::config(); // let all_letters = "abcdefg";
    state.input("input8testtest.txt").lines().for_each(|l| {
        let mut sections = l.split(" | ");

        let mut recorded = HashMap::new();

        sections.next().unwrap().split_whitespace().for_each(|m| {
            let entry = recorded.entry(m.len()).or_insert_with(Vec::new);
            entry.push(m.to_string());
        });

        // find diff between 7 and 1
        let top = {
            let mut one_index = HashSet::new();
            let one_string = &recorded.get(&2).unwrap()[0];
            for char in one_string.chars() {
                one_index.insert(char);
            }
            let seven = &recorded.get(&3).unwrap()[0];
            seven.chars().find(|c| one_index.get(c).is_none()).unwrap()
        };
        let (right_top, right_bot) = {
            let mut eight_index = HashSet::new();
            let eight_string = &recorded.get(&7).unwrap()[0];
            eight_string.chars().for_each(|c| {
                eight_index.insert(c);
            });
            let mut one_index = {
                let mut one_index = HashSet::new();
                let two_string = &recorded.get(&2).unwrap()[0];
                two_string.chars().for_each(|c| {
                    one_index.insert(c);
                });
                one_index
            };
            let mut right_top = '0';
            let mut right_bot = '0';
            let column_6_strings = &recorded.get(&6).unwrap();
            for string in column_6_strings.iter() {
                let mut eight_index_copy = eight_index.clone();
                // eight_index_copy.iter().for_each(|c| print!("{}", c));
                // dbg!(eight_index_copy.len());

                for char in string.chars() {
                    eight_index_copy.remove(&char);
                }
                // check if it exists in two
                let char = eight_index_copy.iter().next().unwrap();
                if one_index.get(char).is_some() {
                    right_top = *char;
                    one_index.remove(char);
                    right_bot = *one_index.iter().next().unwrap();
                    break;
                }
            }
            (right_top, right_bot)
        };
        println!(
            "top {}, top-right {}, bot-right {}",
            top, right_top, right_bot
        );
        let (mid, bot) = {
            let mut mid = '0';
            let mut bot = '0';
            // let mut seven_index = HashSet::new();
            let seven_string = &recorded.get(&3).unwrap()[0];
            // for char in seven_string.chars() {
            //     seven_index.insert(char);
            // }
            let column_5_strings = &recorded.get(&5).unwrap();
            for string in column_5_strings.iter() {
                let mut index = HashSet::new();
                for char in string.chars() {
                    index.insert(char);
                }
                for char in seven_string.chars() {
                    index.remove(&char);
                }
                // eight_index_copy.iter().for_each(|c| print!("{}", c));
                // dbg!(eight_index_copy.len());

                if index.len() == 2 {
                    println!("ffffffffffffff");
                    let mut four_index = HashSet::new();
                    let four_string = &recorded.get(&4).unwrap()[0];
                    for char in four_string.chars() {
                        four_index.insert(char);
                    }
                    // check if it exists in two
                    let mut iter = index.iter();
                    let char1 = iter.next().unwrap();
                    let char2 = iter.next().unwrap();
                    if four_index.get(char1).is_some() {
                        mid = *char1;
                        bot = *char2;
                    } else {
                        mid = *char2;
                        bot = *char1;
                    }
                    break;
                }
            }

            (mid, bot)
        };
        println!("mid {}, bot {}", mid, bot);
        let (left_top, left_bot) = {
            let mut left_top = '0';
            let mut left_bot = '0';
            let column_6_strings = &recorded.get(&6).unwrap();
            for string in column_6_strings.iter() {
                let mut index = HashSet::new();
                for char in string.chars() {
                    index.insert(char);
                }
                index.remove(&top);
                index.remove(&right_top);
                index.remove(&right_bot);
                index.remove(&mid);
                index.remove(&bot);
                if index.len() == 1 {
                    left_top = *index.iter().next().unwrap();
                    let mut eight_index = HashSet::new();
                    let eight_string = &recorded.get(&7).unwrap()[0];
                    eight_string.chars().for_each(|c| {
                        eight_index.insert(c);
                    });
                    eight_index.remove(&top);
                    eight_index.remove(&right_top);
                    eight_index.remove(&right_bot);
                    eight_index.remove(&mid);
                    eight_index.remove(&bot);
                    eight_index.remove(&left_top);
                    left_bot = *eight_index.iter().next().unwrap();
                    break;
                }
            }
            (left_top, left_bot)
        };
        println!("left top {} left bot {}", left_top, left_bot);

        // sections.next().unwrap().split_whitespace().map(|m| {
        //     decode(m)
        // }).collect::<Vec<u32>>();
    });

    // println!("unique numbers: {}", count);
}

fn decode(
    left_top: char,
    left_bot: char,
    top: char,
    mid: char,
    bot: char,
    right_top: char,
    right_bot: char,
    s: &str,
) -> u8 {
    let mut key = [false; 7];
    // let mut lt = false;
    // let mut lb = false;
    // let mut t = false;
    // let mut m = false;
    // let mut b = false;
    // let mut rt = false;
    // let mut rb = false;
    // let mut lt = false;
    // let mut lb = false;
    // let mut t = false;
    // let mut m = false;
    // let mut b = false;
    // let mut rt = false;
    // let mut rb = false;

    // s.chars().for_each(|c| {
    //     match c {
    //         left_top => {
    //             lt = true;
    //         }
    //         left_bot => {
    //             lb = true;
    //         }
    //         top => {
    //             t = true;
    //         }
    //         mid => {
    //             m = true;
    //         }
    //         bot => {
    //             b = true;
    //         }
    //         right_top => {
    //             rt = true;
    //         }
    //         right_bot => {
    //             rb = true;
    //         }
    //         _ => {}
    //     }
    // });
    // match (lt, lb, t, m, b, rt, rb) {
    //     (false.., false, false) => {
    //         2
    //     }
    // }

    s.chars().for_each(|c| match c {
        left_top => {
            key[1] = true;
        }
        left_bot => {
            key[4] = true;
        }
        top => {
            key[0] = true;
        }
        mid => {
            key[3] = true;
        }
        bot => {
            key[6] = true;
        }
        right_top => {
            key[2] = true;
        }
        right_bot => {
            key[5] = true;
        }
        _ => {}
    });
    // match key {

    // }
    todo!()
}
