fn main() {
    use std::time::Instant;
    let now = Instant::now();
    {
        let lines = include_str!("../input.txt");
        let mut totals: Vec<i32> = Vec::new();
        let mut buff: i32 = 0;

        for line in lines.lines() {
            if line.is_empty() {
                totals.push(buff);
                buff = 0;
                continue;
            }
            buff += line.parse::<i32>().unwrap();
        }
        totals.push(buff);
        totals.sort();
        println!("Part 1: {}", totals[totals.len() - 1]);
        println!(
            "Part Two: {}",
            totals[totals.len() - 1] + totals[totals.len() - 2] + totals[totals.len() - 3]
        );
    }
    let elapsed = now.elapsed();
    println!("Time elapsed: {:?}", elapsed);
}
