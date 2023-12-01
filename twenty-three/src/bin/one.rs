fn main() {
    let res: u32 = std::fs::read_to_string("one.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let s = l.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();
            let mut new = String::new();
            new.push(*s.first().unwrap());
            new.push(*s.last().unwrap());
            new.parse::<u32>().unwrap()
        })
        .sum();

    println!("{res}");
}
