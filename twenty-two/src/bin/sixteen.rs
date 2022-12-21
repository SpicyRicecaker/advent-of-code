use std::collections::{HashMap, HashSet, VecDeque};

use regex::Regex;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// struct Node {
//     label:
// }

#[derive(Debug)]
struct Info {
    r: u32,
    o: bool,
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
                o: false,
                e: HashSet::new(),
            };

            c[3].split(", ").for_each(|s| {
                info.e.insert(s.to_string());
            });

            m.insert(s.to_string(), info);
        });

    let mut t_l = 30;

    let mut c = String::from("AA");

    // dbg!(&m);

    let mut p_t = 0;
    while t_l != 0 {
        // generate map of closest routes to next node using bellman-ford algo
        let i = bellman_ford(&c, &m);
        // dbg!(&i);

        // use greedy algo to check if it is worth it to travel there

        let mut p_gen = 0;
        let mut res = None;
        for (k, v) in i.iter().filter(|(k, _)| !m.get(**k).unwrap().o) {
            // 1 extra second to turn it on
            let t_l_local = t_l as i32 - v.d as i32 - 1;
            let p_gen_local = t_l_local * m.get(*k).unwrap().r as i32;

            if p_gen_local > p_gen {
                p_gen = p_gen_local;
                res = Some((k, v));
            }
        }

        if let Some((k, v)) = res {
            t_l -= v.d + 1;

            {
                let mut p = VecDeque::new();

                let mut n: &str = k;

                let mut d = 0;

                // {
                //     let r = i.get(n).unwrap();
                    // p.push_front(format!("{}{}", n, r.d);
                // }
                p.push_front(n);

                while n != c {
                    let r = i.get(n).unwrap();
                    n = r.p.unwrap();
                    d += r.d;
                    p.push_front(n);
                }

                println!(
                    r##"took path {p:?},
                 which took {d} minute(s),
                 and for the next {t_l} minute(s) will generate 
                 {p_gen} pressure 🥲"##
                );
            }

            c = k.to_string();
            p_t += p_gen;
        } else {
            // return
            break;
        }
        // open gate
        println!("moved to {c} 😀");
        m.get_mut(&c).unwrap().o = true;
    }
    dbg!(p_t);
}

#[derive(Debug)]
struct Metrics<'a> {
    d: u32,
    a: bool,
    p: Option<&'a str>,
}

fn bellman_ford<'a>(s: &str, m: &'a HashMap<String, Info>) -> HashMap<&'a str, Metrics<'a>> {
    let mut i: HashMap<&str, Metrics> = m
        .keys()
        .map(|k| {
            (
                k.as_str(),
                Metrics {
                    d: u32::MAX,
                    a: false,
                    p: None,
                },
            )
        })
        .collect();

    {
        let r = i.get_mut(s).unwrap();
        r.d = 0;
        r.a = true;
    }

    loop {
        let mut edited = false;
        for n in m.keys() {
            if !i.get(n.as_str()).unwrap().a {
                continue;
            }

            for e in m.get(n.as_str()).unwrap().e.iter() {
                let d_new = i.get(n.as_str()).unwrap().d + 1;
                if d_new < i.get(e.as_str()).unwrap().d {
                    edited = true;
                    let r = i.get_mut(e.as_str()).unwrap();
                    r.d = d_new;
                    r.a = true;
                    r.p = Some(n);
                }
            }
        }
        if !edited {
            break;
        }
    }
    i
}
