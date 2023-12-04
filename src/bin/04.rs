use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(4);

fn get_numbers(numbers: &str) -> HashSet<u32> {
    numbers
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect()
}

fn calculate_winning(input: &str) -> impl Iterator<Item = u32> + '_ {
    let wins = input.trim().lines().map(|line| {
        let (_, rest) = line.split_once(':').unwrap();
        let (my, winning) = rest.split_once('|').unwrap();
        let my = get_numbers(my);
        let winning = get_numbers(winning);
        my.intersection(&winning).count() as u32
    });
    wins
}

pub fn part_one(input: &str) -> Option<u32> {
    let wins = calculate_winning(input);
    Some(
        wins.map(|count| if count == 0 { 0 } else { 2_u32.pow(count - 1) })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let wins = calculate_winning(input).collect_vec();
    let mut card_count = vec![1_u32; wins.len()];
    for (i, wins) in wins.iter().enumerate() {
        for x in i + 1..=(i + *wins as usize) {
            card_count[x] += card_count[i];
        }
    }
    Some(card_count.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
