use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let (path, nodes) = input.trim().split_once("\n\n").unwrap();
    let nodes = get_map(nodes);
    let current = "AAA";
    if let Some(value) = solve_path(path, &nodes, current, |c| c == "ZZZ") {
        return Some(value);
    }
    None
}

fn solve_path<'a>(
    path: &str,
    nodes: &HashMap<&str, (&'a str, &'a str)>,
    mut current: &'a str,
    end_condition: impl Fn(&str) -> bool,
) -> Option<u32> {
    for (i, path) in path.chars().cycle().enumerate() {
        let (l, r) = nodes.get(current).unwrap();
        current = match path {
            'R' => r,
            'L' => l,
            _ => panic!("unknown path option"),
        };
        if end_condition(current) {
            return Some(i as u32 + 1);
        }
    }
    None
}

fn get_map(nodes: &str) -> HashMap<&str, (&str, &str)> {
    let nodes: HashMap<&str, (&str, &str)> = nodes
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once(" = ").unwrap();
            let (left, right) = rest[1..rest.len() - 1].split_once(", ").unwrap();
            (name, (left, right))
        })
        .collect();
    nodes
}

pub fn part_two(input: &str) -> Option<u64> {
    let (path, nodes) = input.trim().split_once("\n\n").unwrap();
    let nodes = get_map(nodes);

    let start_nodes = nodes.keys().filter(|k| k.ends_with('A')).collect_vec();
    let mut lcd: HashSet<u64> = HashSet::new();
    for node in start_nodes {
        let r = solve_path(path, &nodes, node, |c| c.ends_with('Z'));
        let factors = primes::factors(r.unwrap() as u64);
        lcd.extend(factors);
    }
    let result: u64 = lcd.iter().product();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
