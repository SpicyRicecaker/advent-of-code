use std::cmp::Ordering;

#[derive(Debug, Clone)]
enum NestVec {
    Value(u8),
    Vec(Vec<NestVec>),
}

fn main() {
    let res: usize = std::fs::read_to_string("thirteen.txt")
        .unwrap()
        .split("\n\n")
        .map(|pair| pair.lines())
        .enumerate()
        // .filter(|(idx, _)| *idx == 0)
        .filter_map(|(idx, mut lines)| {
            // dbg!(lines.next());
            // dbg!(lines.next());

            // what do we know?
            // we know that we have to use a stack somehow in the construction of the datastructure
            // we also know that we need to have some way of representing a vec of arbitrary vecs

            // the problem with using this nested vector here is that it is not fun
            // keeping ownership and a pointer at the same time

            let v_first = {
                let chars: Vec<_> = lines.next().unwrap().chars().collect();

                let mut index = 0;

                let mut v = NestVec::Vec(vec![]);
                recurse_build(&mut v, &mut index, &chars);
                v
            };

            let v_second = {
                let chars: Vec<_> = lines.next().unwrap().chars().collect();

                let mut index = 0;

                let mut v = NestVec::Vec(vec![]);
                recurse_build(&mut v, &mut index, &chars);
                v
            };

            // let's say that the vector is now constructed
            // in each iteration we'd have to compare the types of a and b
            // if a holds a list of lists, and b holds a list of lists, then we'd have to drop down a layer
            // if we used matching it'd be pretty elegant because we could match both at the same time
            let res = match recurse_check_valid(&v_first, &v_second) {
                Ordering::Less | Ordering::Equal => Some(idx + 1),
                Ordering::Greater => None,
            };
            dbg!(res);
            res
        })
        .sum();
    dbg!(res);
}

fn recurse_check_valid(f: &NestVec, s: &NestVec) -> Ordering {
    // println!("{:?}", f);
    // println!("{:?}", s);
    // println!("===============================================================");
    match (f, s) {
        (NestVec::Value(a), NestVec::Value(b)) => a.cmp(b),
        (NestVec::Value(_), NestVec::Vec(_)) => {
            // "convert value to list containing its own value as the first value"
            let t = NestVec::Vec(vec![f.clone()]);
            recurse_check_valid(&t, s)
        }
        (NestVec::Vec(_), NestVec::Value(_)) => {
            let t = NestVec::Vec(vec![s.clone()]);
            recurse_check_valid(f, &t)
        }
        (NestVec::Vec(a), NestVec::Vec(b)) => {
            for index in 0..a.len().max(b.len()) {
                // if left runs out first declare less then
                // if both run out at the same time OK
                // if right runs out first declare greater
                match (a.get(index), b.get(index)) {
                    (None, None) => break,
                    (None, Some(_)) => return Ordering::Less,
                    (Some(_), None) => return Ordering::Greater,
                    (Some(a), Some(b)) => match recurse_check_valid(a, b) {
                        o @ Ordering::Less => return o,
                        Ordering::Equal => {}
                        o @ Ordering::Greater => return o,
                    },
                }
            }
            Ordering::Equal
        }
    }
}

fn recurse_build(parent_v: &mut NestVec, current_char: &mut usize, c: &[char]) {
    // this index code is scuffed lol
    loop {
        *current_char += 1;
        match c[*current_char - 1] {
            // drop down a layer
            '[' => match parent_v {
                NestVec::Vec(inside_v) => {
                    let mut new_v = NestVec::Vec(vec![]);
                    recurse_build(&mut new_v, current_char, c);
                    inside_v.push(new_v);
                }
                _ => {
                    panic!()
                }
            },
            // go up a layer
            ']' => {
                return;
            }
            ',' => {}
            c => {
                let num = c.to_digit(10).unwrap() as u8;
                // add num to current layer
                match parent_v {
                    NestVec::Vec(inside_v) => {
                        inside_v.push(NestVec::Value(num));
                    }
                    _ => {
                        panic!()
                    }
                }
            }
        }
        if *current_char == c.len() {
            break;
        }
    }
}
