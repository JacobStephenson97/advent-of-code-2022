use std::{collections::HashMap, fmt::Debug};

fn main() {
    let input = include_str!("../input.txt");

    let mut map: HashMap<String, u64> = HashMap::new();
    let mut current_dir = String::new();

    map.insert("//".to_string(), 0);
    for line in input.split("$").skip(1) {
        let line = line.trim();
        if line.starts_with("cd") {
            let dir = line.split_whitespace().skip(1).next().unwrap();
            if dir == ".." {
                let mut current_dir2 = current_dir.split("/").collect::<Vec<&str>>();
                current_dir2.pop();
                current_dir2.pop();
                current_dir = current_dir2.join("/");
                current_dir.push('/');
            } else {
                current_dir.push_str(dir);
                current_dir.push('/');
            }
        } else if line.starts_with("ls") {
            for cmd in line.lines().skip(1) {
                if cmd.starts_with("dir") {
                } else {
                    let size = cmd
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .parse::<u64>()
                        .unwrap();
                    let mut cwd = current_dir.clone();
                    let new_root = map.get("//").unwrap() + size;
                    map.insert("//".to_string(), new_root);
                    while cwd != "" && cwd != "//" {
                        let parent = cwd.to_string();
                        if map.contains_key(&parent) {
                            let size = map.get(&parent).unwrap() + size;
                            map.insert(parent.clone(), size);
                        } else {
                            map.insert(parent, size);
                        }
                        let mut cwd2 = cwd.split("/").collect::<Vec<&str>>();
                        cwd2.pop();
                        cwd2.pop();
                        cwd = cwd2.join("/");
                        cwd.push('/');
                    }
                }
            }
        }
    }
    // let mut sum: u64 = 0;
    // for key in map.keys() {
    //     let size = map.get(key).unwrap();
    //     if *size < 100000 {
    //         sum += size;
    //     }
    // }
    // let mut sum2: u64 = 0;
    // for key in map.keys() {
    //     let size = map.get(key).unwrap();

    //     sum2 += size;
    // }
    let root = map.get("//").unwrap();
    // println!("{}", root);
    let REQUIRED = (root + 30000000 - 70000000);
    // println!("{}", REQUIRED);
    let mut lowest: u64 = 999999999;
    for key in map.keys() {
        // println!("{}: {}", key, map.get(key).unwrap());
        let size = map.get(key).unwrap();
        if *size > REQUIRED && *size < lowest {
            lowest = *size;
        }
    }
    println!("{}", lowest);
}
