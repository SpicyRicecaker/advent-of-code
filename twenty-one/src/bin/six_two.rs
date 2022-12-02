const DAYS: u32 = 256;
// const DAYS: u32 = 18;

fn main() {
    let state = advent_of_code_2021::config(); // each fish can be
                                               // 0-8
    let mut pool = vec![0_u64; 9];

    state
        .input("input6.txt")
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .for_each(|f| pool[f] += 1);

    (0..DAYS).for_each(|_| {
        // we store the number of births we have
        let births = pool[0];
        // we shift all things back
        (1..pool.len()).for_each(|i| {
            pool[i - 1] = pool[i];
        });
        // set our babies to the number of births
        pool[8] = births;
        // add number of adults to (6)
        pool[6] += births;
    });

    let out: u64 = pool.iter().sum();
    println!("{}", out);
}
