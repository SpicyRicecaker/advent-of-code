#[derive(Debug, Clone)]
enum NestVec {
    Value(u8),
    Vec(Vec<NestVec>),
}

fn main() {
    std::fs::read_to_string("thirteen.txt")
        .unwrap()
        .split("\n\n")
        .map(|pair| pair.lines())
        .enumerate()
        .for_each(|(idx, mut lines)| {
            // dbg!(lines.next());
            // dbg!(lines.next());

            let first = lines.next().unwrap();
            // what do we know?
            // we know that we have to use a stack somehow in the construction of the datastructure
            // we also know that we need to have some way of representing a vec of arbitrary vecs

            // the problem with using this nested vector here is that it is not fun
            // keeping ownership and a pointer at the same time

            let chars: Vec<_> = first.chars().collect();
            let mut index = 0;

            let mut v = NestVec::Vec(vec![]);
            recurse_build(&mut v, &mut index, &chars);

            dbg!(v);
            let separator = std::iter::repeat("=").take(10).collect::<Vec<_>>().join("");
            println!("{separator}");


            // let's say that the vector is now constructed
            // in each iteration we'd have to compare the types of a and b
            // if a holds a list of lists, and b holds a list of lists, then we'd have to drop down a layer
            // if we used matching it'd be pretty elegant because we could match both at the same time
        });
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

fn recurse_check() -> bool {
    todo!()
}
