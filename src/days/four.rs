use std::collections::{HashMap, HashSet};

struct Position {
    x: usize,
    y: usize,
    idx: usize,
}

enum Cell {
    Lit,
    Unlit,
}

pub fn run(state: crate::State) {
    let input = state.input("input4.txt");
    let mut board_input = input.split("\r\n\r\n");

    let called: HashSet<u32> = board_input
        .next()
        .unwrap()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect();

    let mut global_board_index: HashMap<u32, Vec<Position>> = HashMap::new();

    let mut boards: Vec<Vec<Vec<Cell>>> = board_input
        .enumerate()
        .map(|(idx, board)| {
            // get the lines
            board
                .lines()
                .enumerate()
                .map(|(y, l)| {
                    let mut empty_line = Vec::new();
                    // each line
                    l.split_whitespace()
                        // each symbol
                        .map(|l| l.parse::<u32>().unwrap())
                        .enumerate()
                        .for_each(|(x, n)| {
                            empty_line.push(Cell::Unlit);
                            let number_positions =
                                global_board_index.entry(n).or_insert_with(Vec::new);
                            number_positions.push(Position { x, y, idx });
                        });
                    empty_line
                })
                .collect::<Vec<Vec<Cell>>>()
        })
        .collect();
    
    let mut called_iter = called.iter();

    let mut numbers = 0;
    while let Some(called) = called_iter.next() {
        numbers+= 1;
        // if numbers is greater than or equal to 5
        if numbers >=5 {

        }
        // assign to board using the global number index
    };
    dbg!(boards.len());
}
