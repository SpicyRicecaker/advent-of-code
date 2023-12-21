fn main() {
    // B is the total time of this race
    // C is the previous distance record
    // x is the amount of time we press the button

    // the amount of time we press the button is the velocity
    // the amount of time left is the total time minus the time spent pressing the button

    // C = (B - x) * x
    // C = -x^2 + Bx
    // 0 = -x^2 + Bx - C
    let (times, distance) = {
        let input = std::fs::read_to_string("6.txt").unwrap();
        let mut it = input.lines().map(|l| {
            l.split(':')
                .skip(1)
                .next()
                .unwrap()
                .split_whitespace()
                .map(|w| w.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        });
        (it.next().unwrap(), it.next().unwrap())
    };

    let ways = (0..times.len())
        .map(|i| {
            // dbg!(distance[i], times[i]);
            let c = -distance[i] as f64;
            let b = times[i] as f64;
            let a = -1. as f64;

            let det = (b.powf(2.) - 4. * a * c) as f64;

            let roots = [(-b - det.sqrt()) / (2. * a), (-b + det.sqrt()) / (2. * a)];
            dbg!(roots[0], roots[1]);
            // next greater integer
            // previous lesser integer
            // since a is neg, need to sub 1 from 0
            next_smallest_integer(roots[0]) as u32 - next_greatest_integer(roots[1]) as u32 + 1
        })
        .inspect(|v| {
            dbg!(v);
        })
        .reduce(|acc, x| acc * x)
        .unwrap();

    dbg!(ways);
}

fn next_greatest_integer(n: f64) -> f64 {
    (n + 1.).floor()
}

fn next_smallest_integer(n: f64) -> f64 {
    (n - 1.).ceil()
}
