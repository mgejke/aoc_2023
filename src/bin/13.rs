use itertools::Itertools;

advent_of_code::solution!(13);

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn find_mirror(grid: &[Vec<char>], end_condition: u32) -> u32 {
    let width = grid[0].len();

    for x in 1..width {
        let max_range = x.min(width - x);

        let differences = grid.iter().map(|row| {
            let left = row[x - max_range..x].iter();
            let right = row[x..x + max_range].iter().rev();
            left.zip(right).map(|(a, b)| (a != b) as u32)
        });

        let sum: u32 = differences.map(|c| c.sum::<u32>()).sum();

        if sum == end_condition {
            return x as u32;
        }
    }
    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let grids = input.trim().split("\n\n");

    let result = grids.map(|block| {
        let grid = block
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let vert = find_mirror(&grid, 0);
        if vert > 0 {
            vert
        } else {
            find_mirror(&transpose(grid), 0) * 100
        }
    });
    Some(result.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let grids = input.trim().split("\n\n");

    let result = grids.map(|block| {
        let grid = block
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let vert = find_mirror(&grid, 1);
        if vert > 0 {
            vert
        } else {
            find_mirror(&transpose(grid), 1) * 100
        }
    });

    Some(result.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
