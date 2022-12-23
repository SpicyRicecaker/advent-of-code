use std::collections::{HashMap, HashSet};

use regex::Regex;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// struct Node {
//     label:
// }

#[derive(Debug)]
struct Info {
    r: u32,
    e: HashSet<String>,
}
fn main() {
    // we have a graph with values of each node and a distance between each node of 1
    // we can travel a maximum of 30 times - the amount of times we attempt to open a pressure vault

    // 57! is way too slow

    // we want to collect the maximum possible amount of values with those 30 travels.

    // bellman-ford algorithm can find us the minimum distance between a node A and B
    // we can only collect the berry once

    // generate bellman ford for every single node, 57 * n^2 = O(n^2)
    // greedy: find node i st (t - distance[i] - 1) * value[i] > out of all shortest paths
    //   p += value[i]
    //   value[i] = 0
    //   t -= 1
    //   l = i

    let mut m: HashMap<String, Info> = HashMap::new();

    let regex =
        Regex::new(r#"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)"#)
            .unwrap();
    std::fs::read_to_string("sixteen.txt")
        .unwrap()
        .lines()
        .for_each(|l| {
            let c = regex.captures(l).unwrap();

            let s = &c[1];
            let r = c[2].parse::<u32>().unwrap();

            let mut info = Info {
                r,
                e: HashSet::new(),
            };

            c[3].split(", ").for_each(|s| {
                info.e.insert(s.to_string());
            });

            m.insert(s.to_string(), info);
        });

    // dbg!(&m);

    let mut p_g = 0;
    let mut path_g = vec![];

    recurse(
        30,
        String::from("AA"),
        0,
        &m,
        HashSet::new(),
        HashSet::new(),
        &mut p_g,
        vec![],
        &mut path_g,
    );

    dbg!(p_g);
    dbg!(path_g);
}

#[derive(Debug, Clone)]
enum Action {
    OpenValve(String, u32),
    GotoValve(String, u32),
}

fn recurse(
    t_l: u32,
    c: String,
    p: u32,
    m: &HashMap<String, Info>,
    o: HashSet<String>,
    t: HashSet<String>,
    p_g: &mut u32,
    path: Vec<Action>,
    path_g: &mut Vec<Action>,
) {
    // brute force algo, assuming infinite compute:
    //   2 actions
    //     1. move to a new node
    //     2. open current valve
    // minutes, or if all the valves are opened

    // let v, p, t_l, m, c
    // if time left is 0
    //   set p_g = max(p_g, p)
    // if !m[c].o
    //   m clone
    //   m set m[c] open
    //   recurse (m, t_l - 1, p + (t_l - 1) * m[c].v, ..)
    // for n in m[c]
    //   set c clone to n
    //   recurse (m clone, c, ..)

    // dbg!("FUC");
    if p > *p_g {
        *p_g = p;
        *path_g = path.clone();
    }
    // dbg!("FUC");
    if t_l == 0 {
        return;
    }

    // dbg!("FUC");
    let r = m.get(&c).unwrap();
    let c_o = o.contains(&c);

    if !c_o && r.r != 0 {
        let mut o = o.clone();
        o.insert(c.clone());
        let mut path = path.clone();
        // dbg!(p + (t_l - 1));
        path.push(Action::OpenValve(c.clone(), p + (t_l - 1) * r.r));
        let mut t = HashSet::new();
        t.insert(c.clone());
        recurse(t_l - 1, c, p + (t_l - 1) * r.r, m, o, t, p_g, path, path_g);
    } 

        for n in r.e.iter() {
        if t.contains(n) {
            continue;
        }

        let mut t = t.clone();
        t.insert(n.clone());

        let mut path = path.clone();
        path.push(Action::GotoValve(n.clone(), p));

        recurse(t_l - 1, n.clone(), p, m, o.clone(), t, p_g, path, path_g);
    }
}

// time complexity would be the (average number of edges per node + 1) to the
// power of 30, which is around 2^30

// are there any reduntant cases we can remove?
// immediately going back the way we came without opening anything would be redundant
// we build up statik shiv charges and discharge it when we choose to open a valve
