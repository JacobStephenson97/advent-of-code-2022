fn main() {
    let input = include_str!("../input.txt");
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);

    let mut tails = Vec::from([
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ]);

    let mut tail_seen_part_one: Vec<(i32, i32)> = Vec::from([(0, 0)]);
    let mut tail_seen_part_two: Vec<(i32, i32)> = Vec::from([(0, 0)]);
    for line in input.lines() {
        let (dir, dist) = line.split_at(1);
        let dist = dist.trim().parse::<i32>().unwrap();

        for _ in 0..dist {
            match dir {
                "R" => head_pos.0 += 1,
                "L" => head_pos.0 -= 1,
                "U" => head_pos.1 += 1,
                "D" => head_pos.1 -= 1,
                _ => panic!("Unknown direction"),
            }
            //part 1
            move_tail(head_pos, &mut tail_pos);

            //part 2
            move_tail(head_pos, &mut tails[0]);
            for i in 1..tails.len() {
                move_tail(tails[i - 1], &mut tails[i]);
            }

            if !tail_seen_part_one.contains(&tail_pos) {
                tail_seen_part_one.push(tail_pos);
            }
            if !tail_seen_part_two.contains(&tails[8]) {
                tail_seen_part_two.push(tails[8]);
            }
        }
    }
    println!("Part 1: {}", tail_seen_part_one.len());
    println!("Part 2: {}", tail_seen_part_two.len());
}

fn check_tail_pos(head_pos: (i32, i32), tail_pos: (i32, i32)) -> bool {
    if (head_pos.0 - tail_pos.0).abs() == 1 && (head_pos.1 - tail_pos.1).abs() == 1 {
        return true;
    } else if (head_pos.0 - tail_pos.0).abs() + (head_pos.1 - tail_pos.1).abs() <= 1 {
        return true;
    }
    false
}

fn move_tail(head_pos: (i32, i32), tail_pos: &mut (i32, i32)) {
    if *tail_pos == head_pos || check_tail_pos(head_pos, *tail_pos) {
        return;
    }
    tail_pos.1 = if head_pos.1 > tail_pos.1 {
        tail_pos.1 + 1
    } else if head_pos.1 < tail_pos.1 {
        tail_pos.1 - 1
    } else {
        tail_pos.1
    };

    tail_pos.0 = if head_pos.0 > tail_pos.0 {
        tail_pos.0 + 1
    } else if head_pos.0 < tail_pos.0 {
        tail_pos.0 - 1
    } else {
        tail_pos.0
    };
}
