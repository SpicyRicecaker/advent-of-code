use std::collections::{HashMap, HashSet};

struct Position {
    x: usize,
    y: usize,
    idx: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CellState {
    Lit,
    Unlit,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Cell {
    num: u32,
    state: CellState,
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

    // // getting the wincons
    // board_input.next();

    let mut global_board_index: HashMap<u32, Vec<Position>> = HashMap::new();

    let mut board_state: Vec<Vec<Vec<Cell>>> = board_input
        .enumerate()
        .map(|(idx, board)| {
            // get the lines
            board
                .lines()
                .enumerate()
                // for every line
                .map(|(y, l)| {
                    // create empty line
                    let mut empty_line = Vec::new();
                    // each line
                    l.split_whitespace()
                        // each symbol, parse into u32
                        .map(|l| l.parse::<u32>().unwrap())
                        .enumerate()
                        .for_each(|(x, n)| {
                            // for each number
                            // push into empty line
                            empty_line.push(Cell {
                                state: CellState::Unlit,
                                num: n,
                            });
                            let number_positions =
                                global_board_index.entry(n).or_insert_with(Vec::new);
                            number_positions.push(Position { x, y, idx });
                        });
                    empty_line
                })
                .collect::<Vec<Vec<Cell>>>()
        })
        .collect();

    let mut numbers = 0;
    let mut board: Option<Vec<Vec<Cell>>> = None;
    let mut last_called: Option<u32> = None;
    for call in called {
        numbers += 1;
        // Assign to board_state through global_board_index
        if let Some(positions) = global_board_index.get(&call) {
            positions.iter().for_each(|position| {
                board_state[position.idx][position.y][position.x].state = CellState::Lit;
            })
        }
        // if numbers is greater than or equal to 5
        if numbers >= 5 {
            // Loop over all the boards
            if let Some(win_board) = board_state.iter().find(|b| win(b)) {
                board = Some(win_board.to_vec());
                last_called = Some(call);
                break;
            }
        }
        // assign to board using the global number index
    }
    if let Some(board) = board {
        // sum of all unmarked numbers * number that was just called
        let unmarked_sum = board.iter().flatten().fold(0, |acc, x| {
            if x.state == CellState::Lit {
                acc + x.num
            } else {
                acc
            }
        });
        println!("{}", last_called.unwrap());
        println!("{}", unmarked_sum * last_called.unwrap());
    }
}

fn win(board: &[Vec<Cell>]) -> bool {
    // rows
    let rows = board.iter().any(|n| {
        n.iter().fold(0, |acc, x| {
            if x.state == CellState::Lit {
                acc + 1
            } else {
                acc
            }
        }) == n.len()
    });

    let mut columns = false;
    for row in 0..board.len() {
        let mut alives = 0;
        for column in 0..board[0].len() {
            if board[row][column].state == CellState::Lit {
                alives += 1;
            }
        }
        if alives == 5 {
            columns = true;
            break;
        }
    }

    let mut diagonals = false;

    {
        let mut topleft = 0;
        let mut botleft = 0;

        (0..board.len()).for_each(|i| {
            if board[i][i].state == CellState::Lit {
                topleft += 1;
            }
            if board[i][board.len() - i - 1].state == CellState::Lit {
                botleft += 1;
            }
        });

        if topleft == board.len() || botleft == board.len() {
            diagonals = true;
        }
    }
    rows || columns || diagonals
}
