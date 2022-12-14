use std::{collections::VecDeque, fmt::Debug};

pub struct Monkey {
    pub items: VecDeque<usize>,
    // closure that takes in a value B and returns a usize
    pub operation: Box<dyn Fn(usize) -> usize>,
    // 0 for false, 1 for true
    pub test: Box<dyn Fn(&usize) -> bool>,
    pub blocks: [usize; 2],
    pub inspections: usize,
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

impl Monkey {}
#[derive(Debug)]
pub struct MonkeyTreehouse {
    pub lcm: usize,
    pub monkeys: Vec<Monkey>,
}

impl MonkeyTreehouse {
    pub fn run(&mut self, idx: usize) {
        while let Some(mut item) = self.monkeys[idx].items.pop_front() {
            let m = &mut self.monkeys[idx];
            m.inspections += 1;
            // maybe divide by the gcd of all?
            item = (m.operation)(item) % self.lcm;
            if (m.test)(&item) {
                // item.value %= usize::new(vec![9, 6, 5, 7, 7]);
                let res = m.blocks[1];
                self.monkeys[res].items.push_back(item);
            } else {
                // item.value %= usize::new(vec![7, 7, 5, 6, 9]);
                // item.value %= usize::new(vec![9, 6, 5, 7, 7]);
                let res = m.blocks[0];
                self.monkeys[res].items.push_back(item);
            }
        }
    }
}
