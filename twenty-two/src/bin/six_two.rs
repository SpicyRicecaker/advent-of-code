
use std::ops::ControlFlow;
fn main() {
    let chars = std::fs::read_to_string("six.txt")
        .unwrap()
        .chars()
        .map(|c| c as u32 - b'a' as u32)
        .collect::<Vec<u32>>();

    let res = (0..chars.len()).position(|i| {
        let mut v = vec![false; 26];

        let mut no_rep = true;

        let _ = (i..i+14).try_for_each(|j| {
            if v[chars[j] as usize]  {
                no_rep = false;
                return ControlFlow::Break(0);
            } else {
                v[chars[j] as usize] = true;
            }
            ControlFlow::Continue(())
        });

        no_rep
    }).unwrap() + 14;

    dbg!(res);
}
