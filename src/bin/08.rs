use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let (path, nodes) = input.trim().split_once("\n\n").unwrap();
    let nodes = get_map(nodes);
    let current = "AAA";
    Some(solve_path(path, &nodes, current, |c| c == "ZZZ"))
}

fn solve_path<'a>(
    path: &str,
    nodes: &HashMap<&str, (&'a str, &'a str)>,
    mut current: &'a str,
    end_condition: impl Fn(&str) -> bool,
) -> u64 {
    for (i, path) in path.chars().cycle().enumerate() {
        let (l, r) = nodes.get(current).unwrap();
        current = match path {
            'R' => r,
            'L' => l,
            _ => panic!("unknown path option"),
        };
        if end_condition(current) {
            return i as u64 + 1;
        }
    }
    panic!("unsolvable")
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

    let start_nodes = nodes.keys().filter(|k| k.ends_with('A'));
    let mut lcd: HashSet<u64> = HashSet::new();
    for node in start_nodes {
        let r = solve_path(path, &nodes, node, |c| c.ends_with('Z'));
        lcd.extend(primes::factors(r));
    }
    Some(lcd.iter().product())
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
