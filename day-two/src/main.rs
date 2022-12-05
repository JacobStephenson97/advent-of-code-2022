use std::os::raw;

fn main() {
    let input = include_str!("../input.txt");

    let points_p1 = input.lines().map(|line| part_one(line)).sum::<usize>();
    let points_p2 = input.lines().map(|line| part_two(line)).sum::<usize>();

    println!("Part one: {}", points_p1);
    println!("Part one: {}", points_p2);
}

fn part_one(line: &str) -> usize {
    let them = line.split(' ').take(2).collect::<Vec<&str>>();
    return match them[0] {
        "A" => match them[1] {
            "X" => 4,
            "Y" => 8,
            "Z" => 3,
            _ => 0,
        },
        "B" => match them[1] {
            "X" => 1,
            "Y" => 5,
            "Z" => 9,
            _ => 0,
        },
        "C" => match them[1] {
            "X" => 7,
            "Y" => 2,
            "Z" => 6,
            _ => 0,
        },
        _ => 0,
    };
}
fn part_two(line: &str) -> usize {
    let them = line.split(' ').take(2).collect::<Vec<&str>>();

    let me = match them[0] {
        "A" => match them[1] {
            "X" => "Z",
            "Y" => "X",
            "Z" => "Y",
            _ => "_",
        },
        "B" => match them[1] {
            "X" => "X",
            "Y" => "Y",
            "Z" => "Z",
            _ => "_",
        },
        "C" => match them[1] {
            "X" => "Y",
            "Y" => "Z",
            "Z" => "X",
            _ => "_",
        },
        _ => "_",
    };

    return match them[0] {
        "A" => match me {
            "X" => 4,
            "Y" => 8,
            "Z" => 3,
            _ => 0,
        },
        "B" => match me {
            "X" => 1,
            "Y" => 5,
            "Z" => 9,
            _ => 0,
        },
        "C" => match me {
            "X" => 7,
            "Y" => 2,
            "Z" => 6,
            _ => 0,
        },
        _ => 0,
    };
}
