use regex::Regex;
use std::{
    collections::{btree_map::Entry, BTreeMap, VecDeque},
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
    dbg!(r.get_max_pressure_with_two_players());
}

// keeps track of the last 8 assignments to itself
#[derive(Clone, Debug)]
struct Logger<T>
where
    T: Clone,
{
    val: T,
    past: Vec<T>,
}

impl<T> Logger<T>
where
    T: Clone,
{
    fn new(val: T) -> Self {
        Self { val, past: vec![] }
    }
    fn assign(&mut self, val: T) {
        self.past.push(self.val.clone());
        self.val = val;
    }
}

// java's idea of using a class for every file is actually really helpful for
// splitting up and organizing code we can create functions which access a lot
// of "global" variables easier, since we don't have to pass in all the values
#[derive(Debug)]
struct RecursionInfo {
    // set of all permutations of possible paths. I was taught this idea by
    // reddit user u/RookBe in their post @
    // https://www.reddit.com/r/adventofcode/comments/zn6k1l/comment/j1fpf18/?utm_source=share&utm_medium=web2x&context=3
    // which references their blog post at
    // https://nickymeuleman.netlify.app/garden/aoc2022-day16 essentially the
    // idea is to operate on the set of maximum pressures for all given
    // **combinations (not permutations)** of valves we can open within the time
    // limit, instead of simulating two players.
    t: BTreeMap<usize, u32>,
    m: Vec<Vec<u32>>,
    flows: Vec<u32>,
    i_a: usize,
    dbg_legend: Vec<String>,
}

impl RecursionInfo {
    fn new() -> Self {
        let (m, flows, i_a, dbg_legend) = {
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

            let m = shortest_distance_between_all_vertices_via_floyd_warshall(x);

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

            let dbg_legend: Vec<_> = iter.clone().map(|i| legend[i].clone()).collect();

            let flows: Vec<_> = iter.map(|y| flows[y]).collect();

            // convert to adjacency matrix + lookup matrix
            // for traveled & opened, use u64, since we know there are only 57 elements max lol
            (m, flows, i_a_new, dbg_legend)
        };

        Self {
            t: BTreeMap::new(),
            m,
            flows,
            i_a,
            dbg_legend,
        }
    }

    fn visit(&mut self, s: State, worklist: &mut VecDeque<State>) {
        // check if we've already recorded this combination of paths
        // if not, add our pressure
        // if yes, and the pressure generated here is greater, update the
        // pressure
        match self.t.entry(s.t) {
            Entry::Vacant(e) => {
                e.insert(s.p.value);
            }
            Entry::Occupied(mut e) => {
                if s.p.value > *e.get() {
                    e.insert(s.p.value);
                }
            }
        }

        self.m[s.c]
            .iter()
            .enumerate()
            .filter(|(_, dist)| s.t_l > **dist)
            .filter(|&(idx, _)| {
                let bin = 0b1 << idx;
                bin & s.t != bin
            })
            .map(|(i, dist)| (i, s.t_l - *dist - 1))
            .for_each(|(idx, t_l)| {
                // travel to new edge and update pressure
                let mut s = State {
                    t_l,
                    c: idx,
                    ..s.clone()
                };
                s.p += t_l * self.flows[idx];
                s.t |= 0b1 << idx;

                worklist.push_back(s);
            });
    }

    fn run(&mut self) {
        let mut worklist = VecDeque::new();

        worklist.push_back(State {
            t_l: 26,
            c: self.i_a,
            p: Logu32::new(0),
            t: 0b1 << self.i_a,
        });

        // alive, in which case we try to move
        //   if we have no traversable connections, dc
        // dead, in which case we don't do anything, and wait for the other to finish

        let mut count = 0;
        while let Some(s) = worklist.pop_front() {
            self.visit(s, &mut worklist);
            count += 1;
        }
        dbg!(count);
    }

    fn get_max_pressure_with_two_players(&self) -> u32 {
        // we have a list of all combinations of opened valves and their
        // pressures from these, we create another combination of opened valves
        // which do not intercept at all, and get the max pressure

        let mut p_best = 0;

        let bin_a = 0b1 << self.i_a;

        for (t_a, p_a) in self.t.iter() {
            for (t_b, p_b) in self.t.iter() {
               // the only path in common should be the initial a node
               if t_a & t_b == bin_a {
                    let p_sum = p_a + p_b;
                    if p_sum > p_best {
                        p_best = p_sum;
                    }
               }
            }
        }
        
        p_best
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
    t_l: u32,
    c: usize,
    p: Logu32,
    t: usize,
}

// The idea to reduce the amount of permutations we need to compute by finding
// the shortest distance between all vertices (via floyd-warshall) and then
// removing vertices which have 0 flow was given to me by some reddit user on
// r/AdventOfCode
fn shortest_distance_between_all_vertices_via_floyd_warshall(
    mut a: Vec<Vec<u32>>,
) -> Vec<Vec<u32>> {
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
