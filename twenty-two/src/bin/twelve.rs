use std::collections::HashMap;

fn main() {
    const dydx: [[isize; 2]; 4] = [[1, 0], [-1, 0], [0, -1], [0, 1]];

    // we have a graph where every vertex has 4 edges
    // if we use a matrix, there's no easy way to store the distance from one node to another
    // if we use an adjacency matrix, it's hard to pinpoint the exact edges from each vertex, because it'd be just 4 compared to n
    // therefore, the easiest way to implement this would be to use an adjacency list, checking for infinite distances at compile time.

    // ALL ARE Y FOLLOWED BY X

    let mut starting = None;
    let mut ending = None;
    let map: Vec<Vec<_>> = std::fs::read_to_string("twelve.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(|(column, c)| match c {
                    // 'a'
                    'S' => {
                        starting = Some([row, column]);
                        0
                    }
                    // 'z'
                    'E' => {
                        ending = Some([row, column]);
                        25
                    }
                    _ => (c as u8 - b'a') as u32,
                })
                .collect()
        })
        .collect();

    let starting = starting.unwrap();
    let ending = ending.unwrap();

    // build the adjacency list from what we've been given
    // we'll assign each node a value based on their row * column + column
    let mut adjacency_list: HashMap<usize, HashMap<usize, usize>> = HashMap::new();

    for row in 0..map.len() {
        for column in 0..map[0].len() {
            let current_height = map[row][column];

            let adjacent_coords_with_height = dydx
                .into_iter()
                .map(|[dy, dx]| [row as isize + dy, column as isize + dx])
                .filter(|[y, x]| {
                    (0..map.len() as isize).contains(y) && (0..map[0].len() as isize).contains(x)
                })
                .map(|[y, x]| [y as usize, x as usize])
                .map(|[y, x]| {
                    let height = map[y][x];
                    if height <= current_height + 1 {
                        ([y, x], 1)
                    } else {
                        ([y, x], usize::MAX)
                    }
                })
                // if height is one heigher or lower than current, set dist to 1, otherwise infinity
                .map(|([y, x], h)| (y * map[0].len() + x, h));

            let t = adjacency_list
                .entry(row * map[0].len() + column)
                .or_insert_with(HashMap::new);

            for (adjacent_coord, height) in adjacent_coords_with_height {
                t.insert(adjacent_coord, height);
            }
        }
    }
}
