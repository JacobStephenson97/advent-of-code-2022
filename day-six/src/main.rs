use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}
fn part_one(input: &str) -> usize {
    let mut buff: VecDeque<char> = VecDeque::new();
    'outer: for (i, c) in input.chars().enumerate() {
        if buff.len() > 3 {
            buff.pop_front();
        }
        buff.push_back(c);
        let mut buff2: Vec<char> = Vec::new();
        for c in &buff {
            if buff2.contains(&c) {
                continue 'outer;
            } else {
                buff2.push(*c);
            }
            if buff2.len() == 4 {
                println!("{}: {:?}", i, buff2);
                return i + 1;
            }
        }
    }
    return 0;
}
fn part_two(input: &str) -> usize {
    let mut buff: VecDeque<char> = VecDeque::new();
    'outer: for (i, c) in input.chars().enumerate() {
        if buff.len() > 13 {
            buff.pop_front();
        }
        buff.push_back(c);
        let mut buff2: Vec<char> = Vec::new();
        for c in &buff {
            if buff2.contains(&c) {
                continue 'outer;
            } else {
                buff2.push(*c);
            }
            if buff2.len() == 14 {
                println!("{}: {:?}", i, buff2);
                return i + 1;
            }
        }
    }
    return 0;
}
