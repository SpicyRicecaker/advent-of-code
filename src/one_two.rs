use crate::State;

pub fn run(state: State) {
    let input = state
        .input("input1.txt")
        .split_whitespace()
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut summed_input = Vec::new();
    for i in 2..input.len() {
        summed_input.push(input[i] + input[i - 1] + input[i - 2]);
    }

    let mut increments = 0;
    summed_input.iter().enumerate().skip(1).for_each(|(i, &c)| {
        // dbg!(i);
        if c > summed_input[i - 1] {
            increments += 1;
        }
    });

    println!("{}", increments);
}
