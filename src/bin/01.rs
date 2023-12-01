advent_of_code::solution!(1);
use std::{collections::HashMap, str};

pub fn part_one(input: &str) -> Option<u32> {
    let sum: u32 = input
        .trim()
        .split('\n')
        .map(|line| {
            let digits = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<_>>();
            let v: u32 = format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
                .parse()
                .unwrap();
            v
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: HashMap<&'static str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .copied()
    .collect();

    let sum : u32 = input.trim().split('\n').map(|line| {
        let mut digits : Vec<u32> = vec![];
        let mut check = line;
        while let Some(c) = check.chars().next() {
            if c.is_ascii_digit() {
                digits.push(c.to_digit(10).unwrap());
            }
            else {
                let a = map.keys().filter(|key| check.starts_with(*key)).collect::<Vec<_>>();
                if !a.is_empty() {
                    digits.push(*map.get(*a.first().unwrap()).unwrap())
                }
            }
            check = &check[1..];
        }
        digits.first().unwrap() * 10 + digits.last().unwrap()
    }).sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
