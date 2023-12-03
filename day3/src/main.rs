use std::collections::{HashMap, HashSet};

const DIRECTIONS: &[(isize, isize); 8] = &[
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
];

fn main() {
    part1();
    part2();
}

fn part2() {
    let input = include_str!("input.txt");
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut accumulator = Vec::new();
    let mut numbers: Vec<Vec<(usize, usize)>> = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col.is_numeric() {
                accumulator.push((i, j));
            } else {
                if !accumulator.is_empty() {
                    numbers.push(accumulator.clone());
                    accumulator.clear();
                }
            }
        }

        if !accumulator.is_empty() {
            numbers.push(accumulator.clone());
            accumulator.clear();
        }
    }

    let mut global_stars = HashMap::new();

    for num in numbers {
        let parsed = num
            .iter()
            .map(|n| grid[n.0][n.1])
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        let mut stars = HashSet::new();
        for coord in num {
            DIRECTIONS.iter().for_each(|d| {
                let i = coord.0 as isize + d.0;
                let j = coord.1 as isize + d.1;

                if i >= 0 && i < grid.len() as isize && j >= 0 && j < grid[0].len() as isize {
                    if grid[i as usize][j as usize] == '*' {
                        stars.insert((i, j));
                    }
                }
            })
        }
        for star in stars {
            let entry = global_stars.entry(star).or_insert(Vec::new());
            entry.push(parsed);
        }
    }

    let sum: u32 = global_stars
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum();
    println!("Part 2: {}", sum);
}

fn part1() {
    let input = include_str!("input.txt");

    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut accumulator = Vec::new();
    let mut numbers: Vec<Vec<(usize, usize)>> = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col.is_numeric() {
                accumulator.push((i, j));
            } else {
                if !accumulator.is_empty() {
                    numbers.push(accumulator.clone());
                    accumulator.clear();
                }
            }
        }

        numbers.push(accumulator.clone());
        accumulator.clear();
    }

    let sum: u32 = numbers
        .iter()
        .filter(|coords| {
            // There must be at least one symbol around us
            coords.iter().any(|coord| {
                DIRECTIONS.iter().any(|d| {
                    let i = coord.0 as isize + d.0;
                    let j = coord.1 as isize + d.1;

                    if i < 0 || i >= grid.len() as isize || j < 0 || j >= grid[0].len() as isize {
                        false
                    } else {
                        let char = grid[i as usize][j as usize];
                        !(char.is_numeric() || char == '.')
                    }
                })
            })
        })
        .map(|coords| {
            coords
                .iter()
                .map(|coord| grid[coord.0][coord.1])
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        })
        .sum();

    println!("Part 1: {}", sum);
}
