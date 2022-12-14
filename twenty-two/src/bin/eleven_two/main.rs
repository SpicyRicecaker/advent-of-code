mod monkey;

use monkey::*;
use std::{fmt::Debug, collections::VecDeque};

pub struct MonkeyFactory {
    lcm: usize,
}

impl MonkeyFactory {
    fn new() -> Self {
        MonkeyFactory {
            lcm: 1,
        }
    }

    pub fn new_monkey(&mut self, s: &str) -> Monkey {
        let mut lines = s.lines();

        lines.next();
        
        let items: VecDeque<usize> = lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let t = lines.next().unwrap().to_string();

        let operation = Box::new(move |num: usize| -> usize {
            let mut iter = t.split("= ").nth(1).unwrap().split_whitespace();

            let num1 = match iter.next().unwrap() {
                "old" => num,
                s => s.parse::<usize>().unwrap(),
            };

            let op = iter.next().unwrap();

            let num2 = match iter.next().unwrap() {
                "old" => num,
                s => s.parse::<usize>().unwrap(),
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

        let divisor = t
                .split("divisible by ")
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();

        let test = Box::new(move |num: &usize| -> bool {
            num % divisor == 0 
        });

        self.lcm *= divisor;

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

        Monkey {
            items,
            operation,
            test,
            blocks,
            inspections: 0,
        }
    }
}

fn main() {
    let mut monkey_factory = MonkeyFactory::new();
    let monkeys: Vec<Monkey> = std::fs::read_to_string("eleven.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| monkey_factory.new_monkey(s))
        .collect();

    let mut monkey_treehouse = MonkeyTreehouse { monkeys, lcm: monkey_factory.lcm };

    dbg!(monkey_treehouse.lcm);

    let rounds = 10_000;

    (0..rounds).for_each(|round| {
        (0..monkey_treehouse.monkeys.len()).for_each(|m| {
            monkey_treehouse.run(m);
            // println!(
            //     "{:?}",
            //     &monkey_treehouse
            //         .monkeys
            //         .iter()
            //         .map(|m| m.items.clone())
            //         .collect::<Vec<_>>()
            // );
        });
        // println!("finished round {round}");
    });

    let mut monkeys = monkey_treehouse.monkeys;
    monkeys.sort();

    let res: usize = monkeys
        .into_iter()
        .rev()
        .take(2)
        .map(|m| m.inspections)
        .product();
    dbg!(res);
}
