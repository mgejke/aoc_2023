use itertools::Itertools;

advent_of_code::solution!(9);

fn interpolate_end(all: Vec<Vec<i64>>) -> i64 {
    let mut prev: i64 = 0;
    let mut new_last: i64 = 0;
    for row in all.iter().rev() {
        new_last = row.last().unwrap() + prev;
        prev = new_last;
    }
    new_last
}

fn interpolate_begin(all: Vec<Vec<i64>>) -> i64 {
    let mut prev: i64 = 0;
    let mut new_first: i64 = 0;
    for row in all.iter().rev() {
        new_first = row.first().unwrap() - prev;
        prev = new_first;
    }
    new_first
}

fn generate_rows(line: &str) -> Vec<Vec<i64>> {
    let numbers: Vec<i64> = line
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect_vec();

    let mut all: Vec<Vec<i64>> = vec![numbers];
    while all.last().unwrap().iter().any(|v| *v != 0) {
        all.push(
            all.last()
                .unwrap()
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect_vec(),
        );
    }
    all
}

pub fn part_one(input: &str) -> Option<i64> {
    let vectors = input.trim().lines().map(|line| {
        let all = generate_rows(line);
        interpolate_end(all)
    });

    Some(vectors.sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    let vectors = input.trim().lines().map(|line| {
        let all = generate_rows(line);
        interpolate_begin(all)
    });

    Some(vectors.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
