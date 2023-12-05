use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_while1},
    character::{
        self,
        complete::{multispace0, multispace1, space1},
    },
    multi::{many1, separated_list1},
    sequence::{terminated, tuple},
    IResult, Parser,
};
use rayon::prelude::*;
use std::ops::RangeInclusive;

advent_of_code::solution!(5);

#[derive(Debug)]
struct SeedList {
    seeds: Vec<i64>,
}

#[derive(Debug)]
struct TranslationMap {
    name: String,
    ranges: Vec<(RangeInclusive<i64>, i64)>,
}

impl TranslationMap {
    fn translate(&self, value: i64) -> i64 {
        for (range, offset) in &self.ranges {
            if range.contains(&value) {
                return value - (range.start() - *offset);
            }
        }
        value
    }
}

impl PartialEq for TranslationMap {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.ranges == other.ranges
    }
}

impl PartialEq for SeedList {
    fn eq(&self, other: &Self) -> bool {
        self.seeds == other.seeds
    }
}

fn parse_all(line: &str) -> (SeedList, Vec<TranslationMap>) {
    let (rest, seed_list) = parse_seed_list(line).expect("Couldn't parse seed list");
    let (_, maps) = many1(parse_map)(rest).expect("Couldn't parse translation maps");
    (seed_list, maps)
}

fn parse_map_name(line: &str) -> IResult<&str, &str> {
    let (rest, (name, _, _)) =
        tuple((take_while1(|c| c != ':'), tag(":"), multispace0)).parse(line)?;
    Ok((rest, name))
}

fn parse_seed_list(line: &str) -> IResult<&str, SeedList> {
    tuple((tag("seeds: "), parse_space_separated))
        .parse(line)
        .map(|(rest, (_, seeds))| Ok((rest, SeedList { seeds })))?
}

fn parse_space_separated(line: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, character::complete::i64)(line)
}

fn parse_map_range(line: &str) -> IResult<&str, (RangeInclusive<i64>, i64)> {
    tuple((
        terminated(character::complete::i64, multispace1),
        terminated(character::complete::i64, multispace1),
        terminated(character::complete::i64, multispace1),
    ))(line)
    .map(|(rest, (dest, src, length))| Ok((rest, (src..=src + length, dest))))?
}

fn parse_map(line: &str) -> IResult<&str, TranslationMap> {
    tuple((parse_map_name, many1(parse_map_range)))
        .parse(line)
        .map(|(rest, (name, ranges))| {
            Ok((
                rest,
                TranslationMap {
                    name: name.to_string(),
                    ranges,
                },
            ))
        })?
}

pub fn part_one(input: &str) -> Option<u32> {
    let (seed_list, maps) = parse_all(input);
    let result = seed_list.seeds.into_iter().fold(i64::MAX, |acc, mut seed| {
        for map in &maps {
            seed = map.translate(seed);
        }
        acc.min(seed)
    });
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (seed_list, maps) = parse_all(input);
    let ranges: Vec<(i64, i64)> = seed_list.seeds.into_iter().tuples().collect_vec();
    let result = ranges
        .into_par_iter()
        .fold_with(i64::MAX, |mut acc, (start, range)| {
            for mut seed in start..start + range {
                for map in &maps {
                    seed = map.translate(seed);
                }
                acc = acc.min(seed);
            }
            acc
        })
        .min()
        .unwrap();
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_full() {
        let data = advent_of_code::template::read_file("examples", DAY);
        let _ = parse_all(&data);
    }

    #[test]
    fn test_parse_map() {
        let result = parse_map(
            "soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15 ",
        );
        assert_eq!(
            result,
            Ok((
                "",
                TranslationMap {
                    name: "soil-to-fertilizer map".to_string(),
                    ranges: vec![(15..=15 + 37, 0), (52..=52 + 2, 37), (0..=15, 39)]
                }
            ))
        );
    }

    #[test]
    fn test_parse_map_range() {
        let result = parse_map_range("50 98 2 ");
        assert_eq!(result, Ok(("", (98..=98 + 2, 50))));
    }

    #[test]
    fn test_parse_map_name() {
        let result = parse_map_name("seed-to-soil map:");
        assert_eq!(result, Ok(("", "seed-to-soil map")));
    }

    #[test]
    fn test_parse_seed_list() {
        let result = parse_seed_list("seeds: 79 14 55 13");
        assert_eq!(
            result,
            Ok((
                "",
                SeedList {
                    seeds: vec![79, 14, 55, 13]
                }
            ))
        );
    }

    #[test]
    fn test_parse_separated_list() {
        let result = parse_space_separated("79 14 55 13");
        assert_eq!(result, Ok(("", vec![79, 14, 55, 13])));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
