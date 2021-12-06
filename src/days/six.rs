pub fn run(state: crate::State) {
    let mut pool: Vec<u32> = state
        .input("input6testtest.txt")
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let out: String = pool
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("{}", out);
}
