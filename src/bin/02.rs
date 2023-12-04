use std::ops::Add;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0},
    multi::separated_list1,
    sequence::tuple,
    IResult, Parser,
};
use thiserror::Error;

advent_of_code::solution!(2);

#[derive(Debug)]
struct CubeCollection {
    red: u32,
    green: u32,
    blue: u32,
}

impl PartialEq for CubeCollection {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}

impl Add for CubeCollection {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

struct Game {
    id: u32,
    rounds: Vec<CubeCollection>,
}

fn parse_game(line: &str) -> IResult<&str, u32> {
    tuple((
        tag("Game "),
        digit1,
        tag(": "),
    ))
    .parse(line).map(|(rest, (_, id, _))| { Ok((rest, id.parse().unwrap())) })?
}

fn parse_single(line: &str) -> IResult<&str, CubeCollection> {
    tuple((
        digit1,
        space0,
        alt((tag("red"), tag("green"), tag("blue"))),
    ))
    .parse(line).map(|(rest,(count, _, color) )| {
        let count = count.parse().unwrap();
        match color {
            "red" => Ok((
                rest,
                CubeCollection {
                    red: count,
                    green: 0,
                    blue: 0,
                },
            )),
            "green" => Ok((
                rest,
                CubeCollection {
                    red: 0,
                    green: count,
                    blue: 0,
                },
            )),
            "blue" => Ok((
                rest,
                CubeCollection {
                    red: 0,
                    green: 0,
                    blue: count,
                },
            )),
            _ => panic!("Unknown color"),
        }
    })?
}

fn parse_round(line: &str) -> IResult<&str, CubeCollection> {
    let (rest, cc) = separated_list1(tag(", "), parse_single)(line)?;
    let res = cc.into_iter().fold(
        CubeCollection {
            red: 0,
            green: 0,
            blue: 0,
        },
        |acc, v| acc + v,
    );
    Ok((rest, res))
}

fn parse_rounds(line: &str) -> IResult<&str, Vec<CubeCollection>> {
    separated_list1(tag("; "), parse_round)(line)
}

#[derive(Error, Debug)]
pub enum ParseGameError {
    #[error("Could not parse Game")]
    Parse,
}

impl TryFrom<&str> for Game {
    type Error = ParseGameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (rest, id) = parse_game(value).or(Err(ParseGameError::Parse))?;
        let (_, rounds) = parse_rounds(rest).or(Err(ParseGameError::Parse))?;

        Ok(Game { id, rounds })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let limits = CubeCollection {
        red: 12,
        green: 13,
        blue: 14,
    };

    let a = input
        .trim()
        .split('\n')
        .map(|line| {
            let game: Game = line.trim().try_into().unwrap();
            if game.rounds.iter().any(|round| {
                round.green > limits.green || round.red > limits.red || round.blue > limits.blue
            }) {
                0
            } else {
                game.id
            }
        })
        .sum();
    Some(a)
}

pub fn part_two(input: &str) -> Option<u32> {
    let a = input
        .trim()
        .split('\n')
        .map(|line| {
            let game: Game = line.trim().try_into().unwrap();
            let mut max = CubeCollection {
                red: 0,
                green: 0,
                blue: 0,
            };

            for round in game.rounds {
                max.red = max.red.max(round.red);
                max.green = max.green.max(round.green);
                max.blue = max.blue.max(round.blue);
            }

            max.red * max.green * max.blue
        })
        .sum();

    Some(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_round1() {
        let (_, cc) = parse_round("1 blue, 2 green, 4 red").unwrap();
        assert_eq!(
            cc,
            CubeCollection {
                red: 4,
                green: 2,
                blue: 1
            }
        );
    }

    #[test]
    fn test_parse_round2() {
        let (_, cc) = parse_round("1 blue, 4 red").unwrap();
        assert_eq!(
            cc,
            CubeCollection {
                red: 4,
                green: 0,
                blue: 1
            }
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
