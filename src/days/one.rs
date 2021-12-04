pub fn run(state: crate::State) {
    // Get input
    let input = state
        .input("input1.txt")
        .split_whitespace()
        .map(|c| {
            // dbg!(c);
            // let b = c.parse::<u32>().unwrap();
            // dbg!(b);
            // b
            c.parse::<u32>().unwrap()
        })
        .collect::<Vec<u32>>();

    let mut increments = 0;

    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            increments += 1;
        }
    }

    println!("{}", increments);
}
