use std::cmp::Ordering;
use std::collections::HashMap;

pub fn run(state: crate::State) {
    let mut registry: HashMap<u32, u32> = HashMap::new();
    let mut max = 0;
    state
        .input("input7.txt")
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .for_each(|n| {
            let count = registry.entry(n).or_insert(0);
            *count += 1;
            if n > max {
                max = n;
            }
        });
    println!("max is {}", max);
    let mut min_fuel_cost = std::u32::MAX;
    let mut best_position = 0;
    // let mut temp = Vec::new();
    (0..=max).into_iter().for_each(|align_position| {
        let mut fuel_cost = 0;
        for (position, crabs) in &registry {
            let fuel_for_one = match align_position.cmp(position) {
                Ordering::Less => position - align_position,
                Ordering::Greater => align_position - position,
                Ordering::Equal => 0,
            };
            // println!(
            //     "difference between {} (current) and {} (median) is {}",
            //     number, median, tmp_diff
            // );
            fuel_cost += (fuel_for_one * (fuel_for_one + 1)) / 2 * crabs;
        }
        // temp.push(fuel_cost);
        if fuel_cost < min_fuel_cost {
            min_fuel_cost = fuel_cost;
            best_position = align_position;
        }
    });
    // temp.sort_unstable();
    // for num in temp.iter().take(3) {
    //     println!("{}", num);
    // }

    println!("minimum fuel cost: {} at a median of {}", min_fuel_cost, best_position);
}
