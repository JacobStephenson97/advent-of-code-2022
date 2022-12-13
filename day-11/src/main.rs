#[derive(Debug)]
struct Monkey<'a> {
    items: Vec<usize>,
    op: char,
    modifier: &'a str,
    monkey_if_true: usize,
    monkey_if_false: usize,
    divisor: usize,
    inspects: usize,
}

impl Monkey<'_> {
    fn operation(&mut self, item: usize) -> usize {
        let modifier = if self.modifier == "old" {
            item.to_string()
        } else {
            self.modifier.to_string()
        };

        // println!("{} {}", item, modifier);
        let modifier = modifier.parse::<usize>().unwrap();
        match &self.op {
            '+' => item + modifier,
            '-' => item - modifier,
            '*' => item * modifier,
            '/' => item / modifier,
            _ => panic!("Unknown operation"),
        }
    }
    fn test(&self, item: &usize) -> usize {
        if item % self.divisor == 0 {
            self.monkey_if_true as usize
        } else {
            self.monkey_if_false as usize
        }
    }

    fn play(&mut self, modulo: usize) {
        let mut i = 0;
        while i < self.items.len() {
            let new = self.operation(self.items[i]);
            self.items[i] = new % modulo as usize;
            i += 1;
            self.inspects += 1;
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let monkeys_input = input.split("\r\n\r\n");
    let mut monkeys: Vec<Monkey> = Vec::new();
    for (_, monk) in monkeys_input.enumerate() {
        let items = monk
            .lines()
            .nth(1)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .trim()
            .split(", ")
            .map(|x| x.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()
            .unwrap();
        let m = Monkey {
            items: items,

            op: monk
                .lines()
                .nth(2)
                .unwrap()
                .split_whitespace()
                .nth(4)
                .unwrap()
                .chars()
                .nth(0)
                .unwrap(),
            modifier: monk
                .lines()
                .nth(2)
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap(),
            divisor: monk
                .lines()
                .nth(3)
                .unwrap()
                .split_whitespace()
                .nth(3)
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            monkey_if_true: monk
                .lines()
                .nth(4)
                .unwrap()
                .split_whitespace()
                .nth(5)
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            monkey_if_false: monk
                .lines()
                .nth(5)
                .unwrap()
                .split_whitespace()
                .nth(5)
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            inspects: 0,
        };

        monkeys.push(m);
    }
    for _ in 0..10000 {
        let lcd = monkeys
            .iter()
            .map(|x| x.divisor as usize)
            .product::<usize>();
        for i in 0..monkeys.len() {
            monkeys[i].play(lcd);
            for j in 0..monkeys[i].items.len() {
                let new = monkeys[i].test(&monkeys[i].items[0]);
                let item = monkeys[i].items[0];
                monkeys[new].items.push(item);
                monkeys[i].items.remove(0);
            }
        }
    }
    let mut sorted = monkeys
        .iter()
        .map(|x| x.inspects as usize)
        .collect::<Vec<usize>>();
    sorted.sort_by(|a, b| b.cmp(a));
    println!("{:?}", sorted.iter().take(2).product::<usize>());
}
