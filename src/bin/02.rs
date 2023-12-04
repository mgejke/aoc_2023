use itertools::Itertools;

advent_of_code::solution!(2);


#[derive(Debug)]
struct CubeCollection {
    red: u32,
    green: u32,
    blue: u32
}

struct Game {
    id: u32,
    rounds: Vec<CubeCollection>
}


impl From<&str> for CubeCollection {
    fn from(value: &str) -> Self {
        let gems = value.trim().split(',').map(|v| v.trim()).collect_vec();

        let mut cc = CubeCollection { red: 0, green: 0, blue: 0 };
        gems.iter().map(|cubes| {
            match cubes.split_once(' ') {
                Some((v, "red")) => {
                    cc.red += v.parse::<u32>().unwrap();
                },
                Some((v, "green")) => {
                    cc.green += v.parse::<u32>().unwrap();
                },
                Some((v, "blue")) => {
                    cc.blue += v.parse::<u32>().unwrap();
                    
                },
                Some(_) => panic!("wtf"),
                None => todo!(),
            }
        }).collect_vec();
        cc
    }
}


impl From<&str> for Game {
    fn from(value: &str) -> Self {
        
        let (game, cubes) = value.split_once(':').unwrap();
        let (_, id) = game.split_once(' ').unwrap();
        
        let rounds: Vec<CubeCollection> = cubes.trim().split(';').map(|round| {
            round.into()
        }).collect_vec();

        Self { id: id.parse().unwrap(), rounds }

    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let limits = CubeCollection { red: 12, green: 13, blue: 14};
    
    let a = input.trim().split('\n').map(|line| {
        let game : Game = line.trim().into();
        if game.rounds.iter().any(|round| {
            round.green > limits.green || round.red > limits.red || round.blue > limits.blue
        }) {
            0
        }
        else {
            game.id
        }
    }).sum();
    Some(a)
}

pub fn part_two(input: &str) -> Option<u32> {
    let a = input.trim().split('\n').map(|line| {
        
        let game : Game = line.trim().into();
        let mut max = CubeCollection { red: 0, green: 0, blue: 0};

        for round in game.rounds {
            max.red = max.red.max(round.red);
            max.green = max.green.max(round.green);
            max.blue = max.blue.max(round.blue);
        }

        max.red * max.green * max.blue


    }).sum();

    Some(a)
}

#[cfg(test)]
mod tests {
    use super::*;

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
