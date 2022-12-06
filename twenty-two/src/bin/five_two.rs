fn main() -> Result<(), Box<dyn std::error::Error>> {
    let str = std::fs::read_to_string("five.txt")?;

    let mut instructions = str.split("\n\n");

    // instructions.clone().for_each(|x| { println!("{x}");
    // });

    let board = instructions
        .next()
        .unwrap()
        .lines()
        .rev()
        .skip(1)
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; board[0].len() / 4 + 1];

    // bump index by four
    // need a vector of vector of chars
    for (idx, i) in (0..board[0].len())
        .into_iter()
        .skip(1)
        .step_by(4)
        .enumerate()
    {
        for j in 0..board.len() {
            if board[j][i] != ' ' {
                stacks[idx].push(board[j][i]);
            }
        }
    }

    dbg!(&stacks);

    instructions
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|l| l.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .for_each(|v| {
            let (num, from, to) = (v[0], v[1] - 1, v[2] - 1);

            let last = stacks[to].len();

            (0..num).for_each(|_| {
                let c = stacks[from].pop().unwrap();
                stacks[to].insert(last, c);
            });
            dbg!(&stacks);
        });

    let res = stacks
        .into_iter()
        .filter_map(|mut v| v.pop())
        .collect::<String>();

    dbg!(res);

    Ok(())
}

