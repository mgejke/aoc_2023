use nom::{
    bytes::complete::take_till,
    character::{
        self,
        complete::{digit1, space1},
    },
    multi::separated_list1,
    sequence::tuple,
    IResult, Parser,
};

advent_of_code::solution!(6);

fn parse_to_int_vec(line: &str) -> Vec<i64> {
    let (_, (_, vec)) = tuple((
        take_till(|c: char| c.is_ascii_digit()),
        parse_space_separated_int,
    ))
    .parse(line)
    .expect("Error");
    vec
}

fn parse_to_str_vec(line: &str) -> Vec<&str> {
    let (_, (_, vec)) = tuple((
        take_till(|c: char| c.is_ascii_digit()),
        parse_space_separated_str,
    ))
    .parse(line)
    .expect("Error");
    vec
}

fn parse_space_separated_int(line: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, character::complete::i64)(line)
}

fn parse_space_separated_str(line: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(space1, digit1)(line)
}

fn ways_to_win(time: i64, distance: i64) -> usize {
    (0..time)
        .filter_map(|t| {
            if t * (time - t) > distance {
                Some(1)
            } else {
                None
            }
        })
        .count()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (time, distance) = input.trim().split_once('\n').unwrap();
    let times = parse_to_int_vec(time);
    let distances = parse_to_int_vec(distance);

    let a: usize = times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| ways_to_win(time, distance))
        .product();

    Some(a as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (time, distance) = input.trim().split_once('\n').unwrap();
    let time = parse_to_str_vec(time).join("").parse().unwrap();
    let distance = parse_to_str_vec(distance).join("").parse().unwrap();

    Some(ways_to_win(time, distance) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int_vec() {
        let vec = parse_to_int_vec("Time:      7  15   30");
        assert_eq!(vec, vec![7, 15, 30]);
    }

    #[test]
    fn test_parse_str_vec() {
        let vec = parse_to_str_vec("Time:      7  15   30");
        assert_eq!(vec, vec!["7", "15", "30"]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
