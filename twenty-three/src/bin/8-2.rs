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

    // record nodes that end with a

    let mut start_states: Vec<String> = vec![];
    it.for_each(|l| {
        let caps = re.captures(l).unwrap();

        if caps[1].ends_with('A') {
            start_states.push(caps[1].into());
        }

        m.insert(caps[1].into(), (caps[2].into(), caps[3].into()));
    });

    // find number of its it takes for a number to end up at a number with all zs
    let mut states = start_states;
    let mut v_iterations = vec![];
    for state in states.iter_mut() {
        let mut iterations: u64 = 0;
        seq.chars().cycle().try_for_each(|x| {
            // dbg!(&state);
            match x {
                'L' => {
                    *state = m.get(state).unwrap().0.clone();
                }
                'R' => {
                    *state = m.get(state).unwrap().1.clone();
                }
                _ => unreachable!(),
            }

            iterations += 1;

            // WIP what if a state goes to multiple endpoints that end with z, but we just stop at the first one?
            if state.ends_with('Z') {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });
        // find the lcm of all those numbers
        v_iterations.push(iterations);
    }

    // find lcm

    dbg!(lcm_all(&v_iterations));
}

fn lcm_all(v_iterations: &[u64]) -> u64 {
    let mut c_lcm = v_iterations[0];

    (1..v_iterations.len()).for_each(|i| {
        c_lcm = lcm(c_lcm, v_iterations[i]);
    });

    c_lcm
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    // assuming a is greater than b
    // gcd a, b = gcd a - b, b
    let mut t_a = a.max(b);
    let mut t_b = a.min(b);

    loop {
        let rem = t_a % t_b;
        if rem == 0 {
            return t_b;
        }
        t_a = t_b;
        t_b = rem;
    }
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(16, 4), 4);
    assert_eq!(gcd(1, 9), 1);
    // undefined
    // assert_eq!(gcd(0, 9), ??);
    assert_eq!(gcd(16, 5), 1);
    assert_eq!(gcd(204, 17), 17);
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(16, 4), 16);
    assert_eq!(lcm(16, 5), 80);
    assert_eq!(lcm(15, 17), 255);
}
#[test]
fn test_lcm_all() {
    assert_eq!(lcm_all(&[1, 2, 3]), 6);
}