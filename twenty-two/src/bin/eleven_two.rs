use num_bigint::BigUint;

use std::{collections::VecDeque, fmt::Debug};

#[derive(Debug)]
struct MonkeyTreehouse {
    monkeys: Vec<Monkey>,
}

impl MonkeyTreehouse {
    fn run(&mut self, idx: usize) {
        while let Some(item) = self.monkeys[idx].items.pop_front() {
            let m = &mut self.monkeys[idx];
            m.inspections += 1;
            let new = (m.operation)(item) / BigUint::new(vec![3]);
            if (m.test)(&new) {
                let res = m.blocks[1];
                self.monkeys[res]
                    .items
                    .push_back(new);
            } else {
                let res = m.blocks[0];
                self.monkeys[res]
                    .items
                    .push_back(new);
            }
        }
    }
}

struct Monkey {
    items: VecDeque<BigUint>,
    // closure that takes in a value B and returns a usize
    operation: Box<dyn Fn(BigUint) -> BigUint>,
    // 0 for false, 1 for true
    test: Box<dyn Fn(&BigUint) -> bool>,
    blocks: [usize; 2],
    inspections: u32,
}

impl PartialEq for Monkey {
    fn eq(&self, other: &Self) -> bool {
        self.inspections == other.inspections
    }
}

impl PartialOrd for Monkey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inspections.partial_cmp(&other.inspections)
    }
}

impl Eq for Monkey {}

impl Ord for Monkey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inspections.cmp(&other.inspections)
    }
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("blocks", &self.blocks)
            .finish()
    }
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let mut lines = s.lines();

        lines.next();

        let items: VecDeque<BigUint> = lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|s| s.parse::<BigUint>().unwrap())
            .collect();

        let t = lines.next().unwrap().to_string();

        let operation = Box::new(move |num: BigUint| -> BigUint {
            let mut iter = t.split("= ").nth(1).unwrap().split_whitespace();

            let num1 = match iter.next().unwrap() {
                "old" => num.clone(),
                s => s.parse::<BigUint>().unwrap(),
            };
            let op = iter.next().unwrap();
            let num2 = match iter.next().unwrap() {
                "old" => num,
                s => s.parse::<BigUint>().unwrap(),
            };
            match op {
                "*" => num1 * num2,
                "+" => num1 + num2,
                _ => {
                    unreachable!()
                }
            }
        });

        let t = lines.next().unwrap().to_string();

        let test = Box::new(move |num: &BigUint| -> bool {
            num % t
                .split("divisible by ")
                .nth(1)
                .unwrap()
                .parse::<BigUint>()
                .unwrap()
                == BigUint::new(vec![0])
        });

        let mut blocks = [0, 0];

        blocks[1] = lines
            .next()
            .unwrap()
            .split("to monkey ")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        blocks[0] = lines
            .next()
            .unwrap()
            .split("to monkey ")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Self {
            items,
            operation,
            test,
            blocks,
            inspections: 0,
        }
    }
}

fn main() {
    let monkeys: Vec<Monkey> = std::fs::read_to_string("eleven.txt")
        .unwrap()
        .split("\n\n")
        .map(Monkey::from)
        .collect();

    let mut monkey_treehouse = MonkeyTreehouse { monkeys };

    let rounds = 20;

    (0..rounds).for_each(|_| {
        (0..monkey_treehouse.monkeys.len()).for_each(|m| {
            monkey_treehouse.run(m);
            // println!("{:?}", &monkey_treehouse);
        });
    });

    let mut monkeys = monkey_treehouse.monkeys;
    monkeys.sort();
    let res: u32 = monkeys
        .into_iter()
        .rev()
        .take(2)
        .map(|m| m.inspections)
        .product();
    dbg!(res);
}
