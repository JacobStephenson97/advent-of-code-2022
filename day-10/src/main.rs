fn main() {
    let input = include_str!("../input.txt");

    let mut cycle = 1;
    let mut x = 1;

    let mut signal_strength: Vec<i32> = Vec::new();

    let mut values: Vec<i32> = Vec::new();

    let mut crt: Vec<Vec<&str>> = vec![
        vec!["."; 40],
        vec!["."; 40],
        vec!["."; 40],
        vec!["."; 40],
        vec!["."; 40],
        vec!["."; 40],
    ];

    for line in input.lines() {
        if line == "noop" {
            light_crt(cycle, x, &mut crt);
            check_strength(cycle, x, &mut signal_strength);
            cycle += 1;
            continue;
        }
        if line.starts_with("addx") {
            values.push(
                line.split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<i32>()
                    .unwrap(),
            );
            for _ in 0..2 {
                light_crt(cycle, x, &mut crt);
                check_strength(cycle, x, &mut signal_strength);
                cycle += 1;
            }
            x += line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
            continue;
        }
    }
    // println!("Total: {}", signal_strength.iter().sum::<i32>());
    for c in crt {
        println!("{:?}", c);
    }
}
fn check_strength(cycle: i32, x: i32, signal_strength: &mut Vec<i32>) {
    if cycle == 20 || (cycle - 20) % 40 == 0 {
        signal_strength.push(x * cycle);
    }
}

fn light_crt(cycle: i32, x: i32, crt: &mut Vec<Vec<&str>>) {
    let row = (cycle - 1) / 40;
    let idx = (cycle - 1) % 40;
    println!("{} {}, {}", x, row, idx);
    if idx == x - 1 || idx == x + 1 || idx == x {
        crt[row as usize][idx as usize] = "#";
    } else {
        crt[row as usize][idx as usize] = ".";
    }
}
