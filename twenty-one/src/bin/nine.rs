fn main() {
    let v = std::fs::read_to_string("res/nine.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let dydx: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    // for each column, each row
    // get all heights in that row which have adjacent tiles less than them, return the height
    // sum these heights, and 1

    // if we just iterated over the vectori

    let res = (0..v.len())
        .flat_map(|row| {
            // we need to move the row here (a usize, which is fine to move),
            // but we need to borrow v vertically, so I'm creating a temporary
            // variable to store a v ref that gets moved into the closure
            let v = &v;
            (0..v[0].len()).filter_map(move |column| {
                let curr_height = v[row][column];

                if dydx
                    .into_iter()
                    .filter_map(|(dx, dy)| {
                        let ny = row as i32 + dy;
                        let nx = column as i32 + dx;

                        if (0..v.len() as i32).contains(&ny) && (0..v[0].len() as i32).contains(&nx)
                        {
                            Some(v[ny as usize][nx as usize])
                        } else {
                            None
                        }
                    })
                    .all(|h| curr_height < h)
                {
                    Some(curr_height)
                } else {
                    None
                }
            })
        })
        .fold(0, |acc, i| i + 1 + acc);


    dbg!(res);
}
