use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(11);

struct Universe {
    grid: HashMap<(i64, i64), char>,
    x_empty: HashSet<i64>,
    y_empty: HashSet<i64>
}

fn calculate_distances(
universe: Universe,
    expansion: i64,
) -> i64 {
    let mut expanded: HashMap<(i64, i64), char> = HashMap::new();
    for ((x, y), c) in &universe.grid {
        let x_exp: usize = universe.x_empty.iter().filter(|v| x > v).count();
        let y_exp = universe.y_empty.iter().filter(|v| y > v).count();

        expanded.insert(
            (
                x + (x_exp as i64) * (expansion - 1),
                y + (y_exp as i64) * (expansion - 1),
            ),
            *c,
        );
    }

    let result: i64 = expanded
        .keys()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| (x1 - x2).abs() + (y1 - y2).abs())
        .sum();
    result
}

fn parse_universe(input: &str) -> Universe {
    let mut grid: HashMap<(i64, i64), char> = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    let mut x_set: HashSet<i64> = HashSet::new();
    let mut y_set: HashSet<i64> = HashSet::new();

    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                grid.insert((x as i64, y as i64), c);

                width = width.max(x);
                height = height.max(y);

                x_set.insert(x as i64);
                y_set.insert(y as i64);
            }
        }
    }

    let x_empty = not_in_range(width, x_set);
    let y_empty = not_in_range(height, y_set);

    Universe { grid, x_empty, y_empty }
}

fn not_in_range(size: usize, existing_set: HashSet<i64>) -> HashSet<i64> {
    let empty: HashSet<i64> = (0..=size)
        .filter_map(|x| {
            let x = x as i64;
            if !existing_set.contains(&x) {
                Some(x)
            } else {
                None
            }
        })
        .collect();
    empty
}

pub fn part_one(input: &str) -> Option<i64> {
    let universe = parse_universe(input);
    let result = calculate_distances(universe, 2);
    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let universe = parse_universe(input);
    let result = calculate_distances(universe, 1_000_000);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
