#![allow(unused)]

use regex::Regex;
use std::{
    collections::{BTreeMap, HashMap},
    time::Instant,
};

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// struct Node {
//     label:
// }

#[derive(Clone, Copy)]
struct Cell {
    r: u8,
    b: u64,
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

    let ((lookup, list), a_index): ((Vec<_>, Vec<_>), usize) = {
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
        )
    };

    // // dbg!(&m);
    let mut p_g = 0;

    let instant = Instant::now();

    // dbg!(&lookup, &list, a_index);

    recurse(30, a_index, 0, &lookup, &list, 0, 0, &mut p_g);

    dbg!(instant.elapsed());

    dbg!(p_g);
}

fn recurse(
    t_l: u32,
    c: usize,
    p: u32,
    lookup: &[u32],
    list: &Vec<Vec<usize>>,
    o: u64,
    t: u64,
    p_g: &mut u32,
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

    if t_l == 0 {
        if p > *p_g {
            *p_g = p;
        }
        return;
    }

    let r = lookup[c];
    let bin = 0b1 << c;
    let c_o = bin & o == bin;
    // dbg!(c, c_o, r);

    if !c_o && r != 0 {
        // dbg!("ff");
        let o = o | bin;
        let t = bin;
        recurse(t_l - 1, c, p + (t_l - 1) * r, lookup, list, o, t, p_g);
    }

    for n in list[c].iter() {
        let bin = 0b1 << n;
        if bin & t == bin {
            continue;
        }
        // dbg!("hello world");

        let t = t | bin;
        // println!("t: {t:b} bin:{bin:b}");
        recurse(t_l - 1, *n, p, lookup, list, o, t, p_g);
    }
}

// // time complexity would be the (average number of edges per node + 1) to the
// // power of 30, which is around 2^30

// // are there any reduntant cases we can remove?
// // immediately going back the way we came without opening anything would be redundant
// // we build up statik shiv charges and discharge it when we choose to open a valve
