use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    let forest: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let count = forest
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(j, _)| check_visibility(&forest, i, *j, forest[i][*j], 'S'))
                .count()
        })
        .sum::<usize>();

    let score = forest
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, _)| {
                    check_score(
                        &forest,
                        i,
                        j,
                        forest[i][j],
                        'S',
                        &mut [('U', 0), ('L', 0), ('D', 0), ('R', 0)]
                            .iter()
                            .cloned()
                            .collect(),
                    )
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("Part 1: {}", count);
    println!("Part 2: {:?}", score);
}

fn check_visibility(forest: &Vec<Vec<u32>>, i: usize, j: usize, height: u32, dir: char) -> bool {
    let mut visible = false;
    if i == 0 || j == 0 || i == forest.len() - 1 || j == forest[i].len() - 1 {
        if dir != 'S' && forest[i][j] >= height {
            return false;
        } else {
            visible = true;
        }
    } else {
        if (dir == 'U' || dir == 'S') && forest[i - 1][j] < height {
            visible = check_visibility(forest, i - 1, j, height, 'U');
        }
        if (dir == 'L' || dir == 'S') && forest[i][j - 1] < height && !visible {
            visible = check_visibility(forest, i, j - 1, height, 'L');
        }
        if (dir == 'D' || dir == 'S') && forest[i + 1][j] < height && !visible {
            visible = check_visibility(forest, i + 1, j, height, 'D');
        }
        if (dir == 'R' || dir == 'S') && forest[i][j + 1] < height && !visible {
            visible = check_visibility(forest, i, j + 1, height, 'R');
        }
    }
    return visible;
}

fn check_score(
    forest: &Vec<Vec<u32>>,
    i: usize,
    j: usize,
    height: u32,
    dir: char,
    count: &mut HashMap<char, usize>,
) -> usize {
    if i != 0 && (dir == 'U' || dir == 'S') {
        *count.get_mut(&'U').unwrap() += 1;
        if forest[i - 1][j] < height && i != 0 {
            check_score(forest, i - 1, j, height, 'U', count);
        }
    }
    if j != 0 && (dir == 'L' || dir == 'S') {
        *count.get_mut(&'L').unwrap() += 1;
        if forest[i][j - 1] < height && j != 0 {
            check_score(forest, i, j - 1, height, 'L', count);
        }
    }
    if i != forest.len() - 1 && (dir == 'D' || dir == 'S') {
        *count.get_mut(&'D').unwrap() += 1;
        if forest[i + 1][j] < height && i != forest.len() - 1 {
            check_score(forest, i + 1, j, height, 'D', count);
        }
    }
    if j != forest[i].len() - 1 && (dir == 'R' || dir == 'S') {
        *count.get_mut(&'R').unwrap() += 1;
        if forest[i][j + 1] < height && j != forest[i].len() - 1 {
            check_score(forest, i, j + 1, height, 'R', count);
        }
    }
    let score = count.iter().map(|(_, v)| v).product::<usize>();
    return score;
}
