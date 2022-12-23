use regex::Regex;
use std::{
    collections::{BTreeMap, HashMap},
    time::Instant,
};

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

    let ((lookup, list), a_index, legend): ((Vec<_>, Vec<_>), usize, HashMap<usize, String>) = {
        let mut legend: HashMap<String, usize> = HashMap::new();

        let mut counter = 0..;

        let mut connections: BTreeMap<usize, (u32, Vec<usize>)> = BTreeMap::new();

        let regex =
            Regex::new(r#"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)"#)
                .unwrap();

        // hashmap < vectors < bitmaps

        // the problem is if we use bitmaps there's no way to lookup a flag without
        // making our own hashing function basically, though they work wonderfully with traveled + opened vectors

        // we use bitmaps instead of integers lol
        // it will be a 2x32 bit bitmap

        // sol 3: we use bitmaps and just convert our current into an adjacency matrix

        std::fs::read_to_string("sixteen.txt")
            .unwrap()
            .lines()
            .for_each(|l| {
                let c = regex.captures(l).unwrap();

                let s = &c[1];
                let r = c[2].parse::<u32>().unwrap();

                let parent = *legend
                    .entry(s.to_string())
                    .or_insert_with(|| counter.next().unwrap());

                c[3].split(", ").for_each(|s| {
                    let edge = *legend
                        .entry(s.to_string())
                        .or_insert_with(|| counter.next().unwrap());
                    // dbg!(edge);
                    connections
                        .entry(parent)
                        .or_insert((r, Vec::new()))
                        .1
                        .push(edge);
                });
            });
        // dbg!(legend.len());

        // convert to adjacency matrix + lookup matrix
        // for traveled & opened, use u64, since we know there are only 57 elements max lol
        // let adjacency_list: Vec<Vec<u8>> = vec![vec![]; legend.len()];
        (
            connections.into_iter().map(|(_, (r, c))| (r, c)).unzip(),
            *legend.get("AA").unwrap(),
            legend.into_iter().map(|(k, v)| (v, k)).collect(),
        )
    };

    // // dbg!(&m);
    let mut p_g = 0;
    // let mut path_g = vec![];

    let instant = Instant::now();

    // dbg!(&lookup, &list, a_index);

    recurse(
        26,
        [a_index, a_index],
        0,
        &lookup,
        &list,
        0,
        [0b1 << a_index, 0b1 << a_index],
        Turn::Me,
        &mut p_g,
        [true, true], // &mut path_g,
                      // vec![],
    );

    dbg!(instant.elapsed());

    dbg!(p_g);
    // let path_g: Vec<_> = path_g
    //     .into_iter()
    //     .map(|a| match a {
    //         Action::Open(a, b, c) => Action2::Open(a, b, legend.get(&c).unwrap().clone()),
    //         Action::Goto(a, c) => Action2::Goto(a, legend.get(&c).unwrap().clone()),
    //     })
    //     .collect();
    // dbg!(path_g);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Turn {
    Me = 0,
    Elephant = 1,
}

impl Turn {
    #[inline(always)]
    fn switch(self) -> Self {
        match self {
            Turn::Me => Turn::Elephant,
            Turn::Elephant => Turn::Me,
        }
    }
}

// #[derive(Debug, Clone)]
// enum Action {
//     Open(Turn, u32, usize),
//     Goto(Turn, usize),
// }

// #[derive(Debug, Clone)]
// enum Action2 {
//     Open(Turn, u32, String),
//     Goto(Turn, String),
// }

#[inline(always)]
fn recurse(
    t_l: u32,
    c: [usize; 2],
    p: u32,
    lookup: &[u32],
    list: &Vec<Vec<usize>>,
    o: u64,
    t: [u64; 2],
    turn: Turn,
    p_g: &mut u32,
    alive: [bool; 2],
    // path_g: &mut Vec<Action>,
    // path: Vec<Action>,
) {
    // brute force algo, assuming infinite compute:
    // for each of 2 ppl
    //   2 actions
    //     1. move to a new node
    //     2. open current valve
    // the problem is the total of four actions needs to be done for 1 recursion statement.
    // the simple way to clone all our values

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

    if t_l == 0 {
        if p > *p_g {
            // dbg!(p, " 123123123123123");
            *p_g = p;
            // *path_g = path;
        }
        return;
    }

    // dbg!(turn as usize);
    let c1 = c[turn as usize];

    let r = lookup[c1];
    let c1_bin = 0b1 << c1;
    let c_o = c1_bin & o == c1_bin;
    // dbg!(c, c_o, r);
    
    let new = turn.switch();

    let mut did_something = false;
    if !c_o && r != 0 {
        // dbg!("ff");
        let o = o | c1_bin;
        let mut t = t;
        t[turn as usize] = c1_bin;

        // let mut path = path.clone();
        // path.push(Action::Open(turn, p + (t_l - 1) * r, c1));

        did_something = true;

        // if t.switch is not alive
        //   don't switch
        //   tick down time

        recurse(
            if !alive[new as usize] || turn == Turn::Elephant {
                t_l - 1
            } else {
                t_l
            },
            c,
            p + (t_l - 1) * r,
            lookup,
            list,
            o,
            t,
            if !alive[new as usize] { turn } else { new },
            p_g,
            alive, // path_g,
                   // path,
        );
    }
    for n in list[c1].iter() {
        let bin = 0b1 << n;
        if bin & t[turn as usize] == bin {
            continue;
        }
        // dbg!("hello world");

        let mut t = t;
        t[turn as usize] |= bin;

        let mut c = c;
        c[turn as usize] = *n;
        // println!("t: {t:b} bin:{bin:b}");

        // let mut path = path.clone();
        // path.push(Action::Goto(turn, *n));

        did_something = true;

        recurse(
            if !alive[new as usize] || turn == Turn::Elephant {
                t_l - 1
            } else {
                t_l
            },
            c,
            p,
            lookup,
            list,
            o,
            t,
            if !alive[new as usize] { turn } else { new },
            p_g,
            alive, // path_g,
                   // path,
        );
    }

    // do nothing?
    if !did_something {
        let mut alive = alive;
        alive[turn as usize] = false;

        recurse(
            if turn == Turn::Elephant { t_l - 1 } else { t_l },
            c,
            p,
            lookup,
            list,
            o,
            t,
            turn.switch(),
            p_g,
            alive, // path_g,
                   // path,
        );
    }
}

// // time complexity would be the (average number of edges per node + 1) to the
// // power of 30, which is around 2^30

// // are there any reduntant cases we can remove?
// // immediately going back the way we came without opening anything would be redundant
// // we build up statik shiv charges and discharge it when we choose to open a valve
