fn main() {
    let v: Vec<Vec<u32>> = std::fs::read_to_string("eight.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    // edges are y = 0, x = 0, y = len() - 1, x = [0]len() - 1
    // for each tree in the middle, raycast outwards, if it aint blocked it's visible and we go next

    let dxdy = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let range = 0..(v.len() as i32);
    let domain = 0..(v[0].len() as i32);

    let mut count = 0;
    for y in 1..(v.len() - 1) {
        for x in 1..(v[0].len() - 1) {
            let height = v[y][x];

            for (dx, dy) in dxdy.into_iter() {
                let (mut cx, mut cy): (i32, i32) = (x as i32, y as i32);
                cx += dx;
                cy += dy;

                let mut visible = true;
                while domain.contains(&cx) && range.contains(&cy) {
                    if v[cy as usize][cx as usize] >= height {
                        visible = false;
                        break;
                    }
                    cx += dx;
                    cy += dy;
                }
                if visible {
                    count += 1;
                    break;
                }
            }
        }
    }
    count += 2 * v.len() + 2 * (v[0].len() - 2);
    dbg!(count);
}
