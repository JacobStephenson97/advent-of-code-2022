use std::usize;

fn main() {
    let mut lines = include_str!("../input.txt")
        .split("\r\n\r\n")
        .map(|x| return x.lines().flat_map(|x| x.parse::<usize>()).sum::<usize>())
        .collect::<Vec<usize>>();

    lines.sort_by(|a, b| b.cmp(a));

    println!("{:?}", lines.iter().take(3).sum::<usize>());
}
