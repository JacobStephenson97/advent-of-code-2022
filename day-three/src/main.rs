fn main() {
    let input = include_str!("../input.txt");

    let part_one = input
        .lines()
        .map(|line| {
            let first_half = &line[..line.len() / 2];
            let second_half = &line[line.len() / 2..];

            for c in first_half.chars() {
                if second_half.contains(c) {
                    if c.is_lowercase() {
                        return c as usize - 96;
                    } else {
                        return c as usize - 38;
                    }
                }
            }
            return 0;
        })
        .sum::<usize>();

    println!("Part One: {}", part_one);
    println!("Part Two: {}", part_two(input));
}

fn part_two(input: &str) -> usize {
    let line_vec = input.lines().collect::<Vec<&str>>();
    let mut count = 0;
    'outer: for i in 0..line_vec.len() / 3 {
        let first = line_vec[i * 3];
        let second = line_vec[i * 3 + 1];
        let third = line_vec[i * 3 + 2];

        for c in first.chars() {
            if second.contains(c) && third.contains(c) {
                if c.is_lowercase() {
                    count += c as usize - 96;
                    continue 'outer;
                } else {
                    count += c as usize - 38;
                    continue 'outer;
                }
            }
        }
    }
    return count;
}
