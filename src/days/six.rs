const DAYS: u32 = 80;
// const DAYS: u32 = 18;

pub fn run(state: crate::State) {
    let mut pool: Vec<u32> = state
        .input("input6.txt")
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    
    (0..DAYS).for_each(|_| {
        (0..pool.len()).for_each(|fish| {
            match pool[fish] {
                0 => {
                    // push new fish to back
                    pool.push(8);
                    pool[fish] = 6;
                }
                _ => {
                    pool[fish] -= 1;
                }
            }
        });
    });

    // let out: String = pool
    //     .iter()
    //     .map(|c| c.to_string())
    //     .collect::<Vec<String>>()
    //     .join(",");
    // println!("{}", out);
    println!("{}", pool.len());
}
