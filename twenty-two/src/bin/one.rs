#[derive(Debug)]
struct C {
    idx: usize,
    val: u32,
}

fn main() {
    let input = std::fs::read_to_string("one.txt").unwrap();
    let iter = input.split("\n\n");

    let mut output = iter
        .map(|l| {
            l.split_whitespace()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    output.sort();
    dbg!(&output);
    dbg!(output[output.len() - 3..].to_vec());
    dbg!(output[output.len() - 3..].iter().sum::<u32>());
    // .map(|l| {
    //     dbg!(&l);
    //     l
    // })
    // .enumerate()

    // .fold(C { idx: 0, val: 0 }, |acc, (idx, val)| {
    //     if val > acc.val {
    //         C { idx, val }
    //     } else {
    //         acc
    //     }
    // });

    // println!("{}", output.idx);
}
