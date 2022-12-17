enum NestedVector {
    Value(u8),
    Vec(Vec<NestedVector>),
}

fn main() {
    std::fs::read_to_string("thirteen.txt")
        .unwrap()
        .split("\n\n")
        .map(|pair| pair.lines())
        .for_each(|mut lines| {
            // dbg!(lines.next());
            // dbg!(lines.next());

            let first = lines.next().unwrap();
            // what do we know?
            // we know that we have to use a stack somehow in the construction of the datastructure
            // we also know that we need to have some way of representing a vec of arbitrary vecs

            // the problem with using this nested vector here is that it is not fun
            // keeping ownership and a pointer at the same time

            let chars = first.chars().collect();

            let f = recurse_build();

            // let's say that the vector is now constructed
            // in each iteration we'd have to compare the types of a and b
            // if a holds a list of lists, and b holds a list of lists, then we'd have to drop down a layer
            // if we used matching it'd be pretty elegant because we could match both at the same time
        });
}

fn recurse_build() -> NestedVector::Vec {
            let mut f = NestedVector::Vec(vec![]);

            for c in first.chars() {
                match c {
                    // drop down a layer
                    '[' => {}
                    // go up a layer
                    ']' => {}
                    ',' => continue,
                    _ => {
                        let num = c.to_digit(10).unwrap() as u8;
                        // add num to current layer
                    }
                }
            }

}

fn recurse_check() -> bool {
    todo!()
}
