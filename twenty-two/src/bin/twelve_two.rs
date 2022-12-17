use std::collections::HashMap;

fn main() {
    const DYDX: [[isize; 2]; 4] = [[1, 0], [-1, 0], [0, -1], [0, 1]];

    // we have a graph where every vertex has 4 edges
    // if we use a matrix, there's no easy way to store the distance from one node to another
    // if we use an adjacency matrix, it's hard to pinpoint the exact edges from each vertex, because it'd be just 4 compared to n
    // therefore, the easiest way to implement this would be to use an adjacency list, checking for infinite distances at compile time.

    // ALL ARE Y FOLLOWED BY X

    let mut starting = vec![];
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
                    'S' | 'a' => {
                        starting.push([row, column]);
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

    let starting: Vec<_> = starting
        .into_iter()
        .map(|[y, x]| y * map[0].len() + x)
        .collect();

    let ending = {
        let Some([y, x]) = ending else {
            panic!("ending not found");
        };
        // dbg!(y, x);
        y * map[0].len() + x
    };
    // dbg!(starting, ending);

    // build the adjacency list from what we've been given
    // we'll assign each node a value based on their row * column + column
    let mut adjacency_list: HashMap<usize, HashMap<usize, usize>> = HashMap::new();

    for row in 0..map.len() {
        for column in 0..map[0].len() {
            let current_height = map[row][column];

            let adjacent_coords_with_height = DYDX
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
    // now we can do the bellman ford algorithm
    let mut v: Vec<_> = starting.into_iter().map(|s| bellman_ford(s, ending, &adjacency_list)).collect();
    v.sort();
    dbg!(v[0]);
}

#[derive(Debug, Clone, Copy)]
struct T {
    accessed: bool,
    distance_to: usize,
    predecessor: Option<usize>,
}

impl Default for T {
    fn default() -> Self {
        Self {
            accessed: false,
            distance_to: usize::MAX,
            predecessor: None,
        }
    }
}

// gets the distance from starting to ending
fn bellman_ford(
    starting: usize,
    ending: usize,
    graph: &HashMap<usize, HashMap<usize, usize>>,
) -> usize {
    // create an accessed array
    // create a predecessory array
    // create a distance array
    let mut legend: Vec<_> = (0..graph.len()).map(|_| T::default()).collect();
    // set our distance to starting to be 0
    {
        let s = &mut legend[starting];
        s.distance_to = 0;
        s.accessed = true;
    }

    // loop invariant is that so long as we are in this loop the value to some
    // distance for some node has been mutated
    loop {
        let mut update = false;
        // for each node
        for vertex in 0..graph.len() {
            // if we can't reach this node yet, ignore it
            if !legend[vertex].accessed {
                continue;
            }
            // for each edge of this node
            for (&neighbor, edge) in graph.get(&vertex).unwrap().iter() {
                // if adding our current distance to the edge value results
                // in the distance being less, update everything

                // do not do <=, otherwise there could be an infinite loop?
                let new_distance = legend[vertex].distance_to.saturating_add(*edge);
                if new_distance < legend[neighbor].distance_to {
                    let n = &mut legend[neighbor];
                    n.accessed = true;
                    n.distance_to = new_distance;
                    n.predecessor = Some(vertex);

                    // also specify that an update has occurred
                    update = true;
                }
            }
        }
        if !update {
            break;
        }
    }

    // for i in 0..legend.len() {
    //     println!("{i}: {}", legend[i].distance_to);
    // }

    // dbg!(ending);

    legend[ending].distance_to
}
