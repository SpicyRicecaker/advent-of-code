use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
enum NestVec {
    Value(u8),
    Vec(Vec<NestVec>),
}

impl PartialOrd for NestVec {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(recurse_check_valid(self, other))
    }
}

impl Ord for NestVec {
    fn cmp(&self, other: &Self) -> Ordering {
        recurse_check_valid(self, other)
    }
}

fn main() {
    let mut v: Vec<_> = std::fs::read_to_string("thirteen.txt")
        .unwrap()
        .split("\n\n")
        .map(|pair| pair.lines())
        // .filter(|(idx, _)| *idx == 0)
        .flat_map(|mut lines| {
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

            vec![v_first, v_second].into_iter()
        })
        .collect();

    // When adding the input, MANUALLY ADD [[2]] and [[6]] in that order to the
    // end of the input lol
    let divider_2 = v[v.len() - 1 - 1].clone();
    let divider_6 = v[v.len() - 1].clone();

    v.sort();

    // dbg!(v);

    let position = (v.iter().position(|nv| *nv == divider_2).unwrap()+1)
        * (v.iter().position(|nv| *nv == divider_6).unwrap()+1);

    dbg!(position);
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
            ch => {
                // consume until ] or ,
                let mut buf = String::new();
                buf.push(ch);

                loop {
                    match c[*current_char] {
                        ',' | ']' => {
                            break;
                        }
                        c => {
                            buf.push(c);
                        }
                    }
                    *current_char += 1;
                }
                // dbg!(&buf);

                let num = buf.parse::<u8>().unwrap();
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
