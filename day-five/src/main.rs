fn main() {
    let input = include_str!("../input.txt");

    let (boxes, moves) = input.split_once("\r\n\r\n").unwrap();
    let boxes = boxes.lines().rev().collect::<Vec<&str>>();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    for line in boxes {
        for i in 0..line.len() / 4 + 1 {
            let chars = line.chars().skip(i * 4).take(4).collect::<Vec<char>>();
            if chars[0] == '[' {
                if stacks.len() <= i {
                    stacks.push(Vec::from([chars[1]]));
                } else {
                    stacks[i].push(chars[1])
                }
            }
        }
    }

    for line in moves.lines() {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let count = words[1].parse::<usize>().unwrap();
        let from: usize = words[3].parse().unwrap();
        let to = words[5].parse::<usize>().unwrap() - 1;
        // move_part_one(from, to, count, &mut stacks)
        move_part_two(from, to, count, &mut stacks)
    }
    let answer = stacks.iter().map(|x| x.last().unwrap()).collect::<String>();
    println!("{}", answer)
}
// fn move_part_one(from: usize, to: usize, count: usize, stacks: &mut Vec<Vec<char>>) {
//     for _ in 0..count {
//         let temp = stacks[from - 1].pop().unwrap();
//         stacks[to].push(temp);
//     }
// }
fn move_part_two(from: usize, to: usize, count: usize, stacks: &mut Vec<Vec<char>>) {
    let mut temp_vec: Vec<char> = Vec::new();
    for _ in 0..count {
        let temp = stacks[from - 1].pop().unwrap();
        temp_vec.push(temp);
    }
    for &c in temp_vec.iter().rev() {
        stacks[to].push(c);
    }
}
