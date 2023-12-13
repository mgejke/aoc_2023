use itertools::Itertools;
use rayon::prelude::*;
use memoize::memoize;
advent_of_code::solution!(12);

// fn get_groups(row: &[char]) -> Vec<u32> {
//     row.iter()
//         .group_by(|c| **c != '.')
//         .into_iter()
//         .filter_map(|(c, group)| {
//             if c {
//                 Some(group.collect_vec().len() as u32)
//             } else {
//                 None
//             }
//         })
//         .collect_vec()
// }

// fn completed_groups(row: &[char]) -> Vec<u32> {
//     if let Some((q, _)) = row.iter().find_position(|c| **c == '?') {
//         let groups = get_groups(&row[..q]);
//         // if let Some((_, groups)) = groups.split_last() {
//         //     return groups.to_vec();
//         // }
//         // return vec![];
//         return groups;
//     }
//     get_groups(row)
// }

// fn recurse(row: Vec<char>, solution: &Vec<u32>) -> u32 {
//     let mut sum = 0;
//     if let Some((q, _)) = row.iter().find_position(|c| **c == '?') {
//         let mut r1 = row.clone();
//         r1[q] = '#';

//         let completed = completed_groups(&r1);
//         if completed
//             .iter()
//             .zip(solution)
//             .enumerate()
//             .all(|(i, (a, b))| a == b || ((i + 1) == completed.len() && a < b))
//         {
//             sum += recurse(r1, solution);
//         }

//         let mut r2 = row.clone();
//         r2[q] = '.';
//         let completed = completed_groups(&r2);
//         if completed
//             .iter()
//             .zip(solution)
//             .enumerate()
//             .all(|(i, (a, b))| a == b || ((i + 1) == completed.len() && a < b))
//         {
//             sum += recurse(r2, solution);
//         }

//         return sum;
//     }

//     if *solution == get_groups(&row) {
//         1
//     } else {
//         0
//     }
// }

#[memoize]
fn solve1(records: Vec<char>, counts: Vec<usize>, num_in_group: usize) -> usize {
    if records.is_empty() {
        return (counts.is_empty() && num_in_group == 0) as usize;
    }

    let next = match records.first() {
        Some('?') => vec!['.', '#'],
        Some(x) => vec![*x],
        None => panic!("not possible"),
    };
    
    let mut solutions: usize = 0;
    for c in next {
        if c == '#' {
            solutions += solve1(records[1..].to_vec(), counts.clone(), num_in_group + 1)
        }
        else if num_in_group > 0 {
            if let Some(count) = counts.first() {
                if *count == num_in_group {
                    solutions += solve1(records[1..].to_vec(), counts[1..].to_vec(), 0)
                }
            }
        }
        else {
            solutions += solve1(records[1..].to_vec(), counts[..].to_vec(), 0)
        }
    }
    solutions
}


pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .trim()
        .par_lines()
        .map(| line| {
            // println!("Part 1 - {}", i);

            let (records, groups) = line.split_once(' ').unwrap();
            let records = format!("{records}.");

            let groups: Vec<usize> = groups.split(',').map(|c| c.parse().unwrap()).collect_vec();
            // let possible = recurse(records.chars().collect_vec(), &groups);
            let res = solve1(records.chars().collect_vec(), groups, 0);

            res as u64
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input
        .trim()
        .par_lines()
        .map(|line| {

            let (records, groups) = line.split_once(' ').unwrap();

            let records = std::iter::once(records).cycle().take(5).join("?");
            let records = format!("{records}.");
            let groups: Vec<usize> = groups.split(',').map(|c| c.parse().unwrap()).collect_vec().repeat(5);

            let res = solve1(records.chars().collect_vec(), groups, 0);
            res as u64
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
