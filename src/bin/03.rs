advent_of_code::solution!(3);

use std::collections::HashMap;

#[derive(Default, Debug)]
struct PartNumber {
    x1: i32,
    x2: i32,
    y: i32,
    value: i32,
}

fn check_vicinity(part: &PartNumber, map: &HashMap<(i32, i32), char>) -> Option<i32> {
    for y in (part.y - 1)..=(part.y + 1) {
        for x in (part.x1 - 1)..=(part.x2 + 1) {
            if let Some(c) = map.get(&(x, y)) {
                if !c.is_ascii_digit() && *c != '.' {
                    return Some(part.value);
                }
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<i32> {
    let (map, parts) = extract_parts_and_map(input);

    let value: i32 = parts
        .iter()
        .filter_map(|p| {
            check_vicinity(p, &map)
        })
        .sum();
    Some(value)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, parts) = extract_parts_and_map(input);

    let mut sum: i32 = 0;
    for ((x,y), value) in map.iter() {
        if *value != '*' {
            continue;
        }

        let mut adj = vec![];
        for part in &parts {
            if (part.y - y).abs() > 1 {
                continue;
            }
            if *x >= part.x1 - 1 && *x <= part.x2 + 1 {
                adj.push(part.value);
            } 
        }
        if adj.len() == 2 {
            sum += adj[0] * adj[1];
        }
    }
    Some(sum as u32)
}

fn extract_parts_and_map(input: &str) -> (HashMap<(i32, i32), char>, Vec<PartNumber>) {
    let mut token: HashMap<(i32, i32), char> = HashMap::new();
    let mut parts: Vec<PartNumber> = Vec::new();

    let mut maxx = 0;
    for (y, line) in input.lines().enumerate() {
        let mut ongoing = false;
        let mut digits = vec![];
        let mut part = PartNumber {
            x1: 0,
            x2: 0,
            y: 0,
            value: 0,
        };
        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_digit() && !ongoing {
                digits.push(c);
                part.x1 = x as i32;
                part.y = y as i32;
                ongoing = true;
            } else if c.is_ascii_digit() && ongoing {
                digits.push(c);
            } else if ongoing {
                part.x2 = (x - 1) as i32;
                part.value = digits.iter().collect::<String>().parse().unwrap();
                ongoing = false;
                parts.push(part);
                part = PartNumber::default();
                digits.clear();
            }
            token.insert((x as i32, y as i32), c);
            maxx = maxx.max(x);
        }
        if ongoing {
            part.x2 = (maxx - 1) as i32;
            part.value = digits.iter().collect::<String>().parse().unwrap();
            parts.push(part);
            digits.clear();
        }
    }
    (token, parts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
