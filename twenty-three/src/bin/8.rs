use std::{
    collections::{HashMap, HashSet},
    ops::ControlFlow,
};

use regex::Regex;
fn main() {
    let input = std::fs::read_to_string("8.txt").unwrap();
    let mut it = input.lines();
    let seq = it.next().unwrap();

    it.next();

    let re = Regex::new(r#"(...) = \((...), (...)\)"#).unwrap();

    // use adjacency list
    let mut m: HashMap<String, (String, String)> = HashMap::new();

    it.for_each(|l| {
        let caps = re.captures(l).unwrap();

        m.insert(caps[1].into(), (caps[2].into(), caps[3].into()));
    });

    let mut state: String = "AAA".into();
    let mut iterations = 0;
    seq.chars().cycle().try_for_each(|x| {
        match x {
            'L' => {
                state = m.get(&state).unwrap().0.clone();
            }
            'R' => {
                state = m.get(&state).unwrap().1.clone();
            }
            _ => unreachable!(),
        }
        iterations += 1;
        if state.as_str() == "ZZZ" {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    });
    dbg!(iterations);
}
