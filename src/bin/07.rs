use itertools::Itertools;
use once_cell::sync::Lazy;
use std::{cmp::Ordering, collections::HashMap, sync::Mutex};

advent_of_code::solution!(7);

static CARD_RANK1: Lazy<Mutex<HashMap<char, u32>>> = Lazy::new(|| {
    let m: HashMap<char, u32> = [
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]
    .into_iter()
    .collect();
    Mutex::new(m)
});

static CARD_RANK2: Lazy<Mutex<HashMap<char, u32>>> = Lazy::new(|| {
    let m: HashMap<char, u32> = [
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('J', 1),
    ]
    .into_iter()
    .collect();
    Mutex::new(m)
});

#[derive(Debug, Eq)]
struct Hand<'a> {
    cards: &'a str,
    score: u32,
    wild: bool,
    pattern: Vec<u32>,
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Hand<'a> {
    fn score(&self) -> u32 {
        match self.pattern[..] {
            [5] => 7,
            [1, 4] => 6,
            [2, 3] => 5,
            [1, 1, 3] => 4,
            [1, 2, 2] => 3,
            [1, 1, 1, 2] => 2,
            [1, 1, 1, 1, 1] => 1,
            _ => panic!("ohno"),
        }
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let s_score = self.score();
        let o_score = other.score();

        let cmp = s_score.cmp(&o_score);
        if cmp == std::cmp::Ordering::Equal {
            let map = if !self.wild {
                CARD_RANK1.lock().unwrap()
            } else {
                CARD_RANK2.lock().unwrap()
            };

            for (s, o) in self.cards.chars().zip(other.cards.chars()) {
                let s_score: &u32 = map.get(&s).unwrap();
                let o_score: &u32 = map.get(&o).unwrap();

                match s_score.cmp(o_score) {
                    Ordering::Equal => continue,
                    x => return x,
                }
            }
            Ordering::Equal
        } else {
            cmp
        }
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.score == other.score
    }
}

impl<'a> Hand<'a> {
    fn new(cards: &'a str, score: u32, wild: bool) -> Self {
        let groups = cards.chars().sorted().group_by(|c| *c);

        let mut pattern = groups
            .into_iter()
            .map(|(_, group)| group.count() as u32)
            .collect_vec();
        pattern.sort();

        if wild {
            let joker_count = cards.chars().filter(|c| *c == 'J').count();
            if joker_count > 0 {
                pattern = match (joker_count, &pattern[..]) {
                    (1, [1, 1, 1, 1, 1]) => vec![1, 1, 1, 2],
                    (_, [1, 1, 1, 2]) => vec![1, 1, 3],
                    (1, [1, 2, 2]) => vec![2, 3],
                    (2, [1, 2, 2]) => vec![1, 4],
                    (_, [1, 1, 3]) => vec![1, 4],
                    (_, [2, 3]) => vec![5],
                    (_, [1, 4]) => vec![5],
                    (5, [5]) => vec![5],
                    x => panic!("Unknown - {:?}", x),
                }
            }
        }

        Self {
            cards,
            score,
            wild,
            pattern,
        }
    }
}

fn solve(input: &str, wild: bool) -> Option<u32> {
    let mut hands = input
        .trim()
        .lines()
        .map(|line| {
            let (hand, score) = line.split_once(' ').expect("Should be two");
            Hand::new(hand, score.parse().unwrap(), wild)
        })
        .collect_vec();
    hands.sort();

    let result: usize = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.score as usize)
        .sum();

    Some(result as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
