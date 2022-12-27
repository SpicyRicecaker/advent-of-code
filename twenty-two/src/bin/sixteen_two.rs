use regex::Regex;
use std::{
    collections::{hash_map::Entry, BTreeMap, HashMap, VecDeque},
    ops::AddAssign,
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

    // a queue is the correct solution to this problem, however, the main problem remains how to take care of repeats

    let mut r = RecursionInfo::new();
    r.run();
    // expect DD20*28(560) + BB13*25(325) + JJ21*21(441) + HH22*13(286)
    // dbg!(r);
    println!("{}", r.state_best.p.value);
}

#[derive(Debug)]
struct RecursionInfo {
    players: usize,
    state_best: State,
    t: HashMap<usize, u32>,
    m: Vec<Vec<u32>>,
    flows: Vec<u32>,
    i_a: usize,
    worklist: VecDeque<State>,
}

impl RecursionInfo {
    fn new() -> Self {
        let (m, flows, i_a) = {
            let mut legend: Vec<String> = vec![];
            let mut m: BTreeMap<usize, (u32, Vec<usize>)> = BTreeMap::new();

            let regex = Regex::new(
                r#"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)"#,
            )
            .unwrap();

            std::fs::read_to_string("sixteen.txt")
                .unwrap()
                .lines()
                .for_each(|l| {
                    let c = regex.captures(l).unwrap();

                    let s = &c[1];
                    let r = c[2].parse::<u32>().unwrap();

                    let parent = legend.iter().position(|key| key == s).unwrap_or_else(|| {
                        legend.push(s.to_string());
                        legend.len() - 1
                    });

                    c[3].split(", ").for_each(|s| {
                        let edge = legend.iter().position(|key| key == s).unwrap_or_else(|| {
                            legend.push(s.to_string());
                            legend.len() - 1
                        });

                        m.entry(parent).or_insert((r, Vec::new())).1.push(edge);
                    })
                });
            let (flows, m): (Vec<_>, Vec<_>) = m.values().cloned().unzip();

            let i_a = legend.iter().position(|s| s == "AA").unwrap();
            // we have map + flows, need to convert into an adjacency matrix
            let mut x = vec![vec![u32::MAX; flows.len()]; flows.len()];

            for (k, v) in m.into_iter().enumerate() {
                x[k][k] = 0;
                for &n in v.iter() {
                    x[k][n] = 1;
                }
            }

            let m = floyd_warshall(x);

            // dbg!(&m);

            // generate new flow list and matric by collapsing old
            // .||    .|
            // --| -> -|
            // ---

            let iter = (0..m.len()).filter(|&n| flows[n] != 0 || n == i_a);

            let mut i_a_new = 0;
            let m: Vec<Vec<_>> = iter
                // KEEP THIS ENUMERATE
                .clone()
                .enumerate()
                .map(|(i, y)| {
                    if y == i_a {
                        i_a_new = i;
                    }
                    iter.clone().map(|x| m[y][x]).collect()
                })
                .collect();

            let flows: Vec<_> = iter.map(|y| flows[y]).collect();

            // convert to adjacency matrix + lookup matrix
            // for traveled & opened, use u64, since we know there are only 57 elements max lol
            (m, flows, i_a_new)
        };
        let players = 1;
        Self {
            players,
            state_best: State {
                v_pl: vec![
                    Player {
                        t_l: 30,
                        alive: true,
                        c: i_a
                    };
                    players
                ],
                p: Logu32::new(0),
                t: 0b1 << i_a,
                t_path: vec![i_a],
            },
            t: HashMap::new(),
            m,
            flows,
            i_a,
            worklist: VecDeque::new(),
        }
    }

    fn run_cycle(
        &mut self,
        i: usize,
        dead_count: &mut u32,
        mut s: State,
        worklist: &mut VecDeque<State>,
    ) {
        let p = &s.v_pl[i];

        // if !p.alive {
        //     *dead_count += 1;
        //     // worklist.push_back(s);
        // } else {
        // try traversing
        let mut is_edge_exist = false;

        self.m[p.c]
            .iter()
            .enumerate()
            .filter(|(_, dist)| p.t_l > **dist)
            .filter(|&(idx, _)| {
                let bin = 0b1 << idx;
                bin & s.t != bin
            })
            .map(|(i, dist)| (i, p.t_l - *dist - 1))
            .for_each(|(idx, t_l)| {
                let mut s = s.clone();
                s.p += t_l * self.flows[idx];
                s.t |= 0b1 << idx;
                s.t_path.push(idx);

                let p = &mut s.v_pl[i];
                *p = Player { t_l, c: idx, ..*p };

                is_edge_exist = true;

                worklist.push_back(s);
            });

        if !is_edge_exist {
            s.v_pl[i].alive = false;
            // worklist.push_back(s);
            *dead_count += 1;
        }
        // }
    }

    fn run(&mut self) {
        self.worklist.push_back(self.state_best.clone());

        while let Some(s) = self.worklist.pop_front() {
            let mut dead_count = 0;

            let mut worklist_local: VecDeque<State> = VecDeque::new();
            self.run_cycle(0, &mut dead_count, s.clone(), &mut worklist_local);
            self.worklist.append(&mut worklist_local);

            if dead_count as usize == s.v_pl.len() {
                // compare
                if s.p > self.state_best.p {
                    self.state_best.p = s.p;
                }
            }
        }

        // while let Some(s) = self.worklist.pop_front() {
        //     // if our current traveled exists in t, lookup
        //     let mut worklist_local: VecDeque<State> = VecDeque::new();

        //     let mut dead_count = 0;

        //     // alive, in which case we try to move
        //     //   if we have no traversable connections, dc
        //     // dead, in which case we don't do anything, and wait for the other to finish

        //     for i in 0..self.players {
        //         if i == 0 {
        //             self.run_cycle(i, &mut dead_count, s.clone(), &mut worklist_local);
        //         } else {
        //             // pull out what is currently inside the local worklist
        //             let mut item_idx = 0;
        //             let worklist_previous_len = worklist_local.len();
        //             loop {
        //                 if item_idx == worklist_previous_len {
        //                     break;
        //                 }

        //                 let s = worklist_local.pop_front().unwrap();

        //                 self.run_cycle(i, &mut dead_count, s.clone(), &mut worklist_local);

        //                 item_idx += 1;
        //             }
        //         }
        //     }

        //     self.worklist.append(&mut worklist_local);

        //     if dead_count as usize == s.v_pl.len() {
        //         // compare
        //         if s.p > self.p_highest {
        //             self.p_highest = s.p;
        //         }
        //     }
        // }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct Logu32 {
    value: u32,
    v: Vec<u32>,
}

impl Logu32 {
    fn new(value: u32) -> Self {
        Self { value, v: vec![] }
    }
}

impl AddAssign<u32> for Logu32 {
    fn add_assign(&mut self, rhs: u32) {
        self.v.push(rhs);
        self.value += rhs;
    }
}

impl PartialEq<u32> for Logu32 {
    fn eq(&self, other: &u32) -> bool {
        self.value == *other
    }
}

impl PartialOrd<u32> for Logu32 {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(other)
    }
}

#[derive(Debug, Clone)]
struct State {
    v_pl: Vec<Player>,
    p: Logu32,
    t: usize,
    t_path: Vec<usize>,
}

#[derive(Debug, Clone)]
struct Player {
    t_l: u32,
    alive: bool,
    c: usize,
}

fn floyd_warshall(mut a: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    for k in 0..a.len() {
        for i in 0..a.len() {
            for j in 0..a.len() {
                if a[i][j] > a[i][k].saturating_add(a[k][j]) {
                    a[i][j] = a[i][k].saturating_add(a[k][j]);
                }
            }
        }
    }
    a
}

// time complexity would be the (average number of edges per node + 1) to the
// power of 30, which is around 2^30

// are there any reduntant cases we can remove?
// immediately going back the way we came without opening anything would be redundant
// we build up statik shiv charges and discharge it when we choose to open a valve
