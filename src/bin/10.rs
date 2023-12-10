use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(10);

fn generate_score_map(
    start: (i32, i32),
    grid: &HashMap<(i32, i32), char>,
) -> (HashMap<(i32, i32), i32>, char) {
    let allowed_left: HashSet<char> = ['-', 'L', 'F'].into_iter().collect();
    let allowed_right: HashSet<char> = ['-', 'J', '7'].into_iter().collect();
    let allowed_up: HashSet<char> = ['|', '7', 'F'].into_iter().collect();
    let allowed_down: HashSet<char> = ['|', 'L', 'J'].into_iter().collect();

    let directions = [
        ((-1, 0), allowed_left),
        ((1, 0), allowed_right),
        ((0, -1), allowed_up),
        ((0, 1), allowed_down),
    ];

    let mut nexts = vec![];
    let mut allowed_directions = vec![];

    for (d, check) in &directions {
        let neighbour = (start.0 + d.0, start.1 + d.1);
        if let Some(pipe) = grid.get(&neighbour) {
            if check.contains(pipe) {
                nexts.push(neighbour);
                allowed_directions.push(d);
            }
        }
    }

    let start_piece = match allowed_directions[..] {
        [(-1, 0), (1, 0)] => '-',
        [(0, -1), (0, 1)] => '|',
        [(-1, 0), (0, -1)] => 'J',
        [(-1, 0), (0, 1)] => '7',
        [(1, 0), (0, -1)] => 'L',
        [(1, 0), (0, 1)] => 'F',
        _ => panic!("ohnoes"),
    };

    let possible_directions: HashMap<char, ((i32, i32), (i32, i32))> = [
        ('|', ((0, 1), (0, -1))),
        ('-', ((-1, 0), (1, 0))),
        ('L', ((1, 0), (0, -1))),
        ('J', ((-1, 0), (0, -1))),
        ('7', ((-1, 0), (0, 1))),
        ('F', ((1, 0), (0, 1))),
        // ('.', ((0, 0), (0, 0))),
    ]
    .into_iter()
    .collect();

    let mut previous = start;

    let mut scores: HashMap<(i32, i32), i32> = HashMap::new();
    for check in nexts {
        let mut score = 0;
        let mut current = check;

        while let Some(pipe) = grid.get(&current) {
            if *pipe == 'S' {
                break;
            }

            let (w1, w2) = possible_directions.get(pipe).unwrap();
            let p1 = (current.0 + w1.0, current.1 + w1.1);
            let p2 = (current.0 + w2.0, current.1 + w2.1);

            scores
                .entry(previous)
                .and_modify(|v| *v = score.min(*v))
                .or_insert(score);

            let next = if p1 == previous { p2 } else { p1 };
            previous = current;
            current = next;
            score += 1;
        }
    }
    (scores, start_piece)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    let mut start = (0_i32, 0_i32);

    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((x as i32, y as i32), c);
            if c == 'S' {
                start = (x as i32, y as i32);
            }
        }
    }

    let (scores, _) = generate_score_map(start, &grid);

    Some(*scores.values().max().unwrap() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    let mut start = (0_i32, 0_i32);

    let mut width = 0;
    let mut height = 0;
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((x as i32, y as i32), c);
            if c == 'S' {
                start = (x as i32, y as i32);
            }
            width = width.max(x);
        }
        height = height.max(y);
    }

    let (scores, start_piece) = generate_score_map(start, &grid);
    if let Some(start_p) = grid.get_mut(&start) {
        *start_p = start_piece;
    }

    let mut count = 0;
    for y in 0..=height {
        let mut row: VecDeque<_> = (0..=width)
            .map(|x| grid.get(&(x as i32, y as i32)).unwrap())
            .collect();

        let mut inside = false;
        let mut x = 0;
        while let Some(c) = row.pop_front() {
            let current = (x, y as i32);
            x += 1;

            if scores.contains_key(&current) {
                if *c == '|' {
                    inside = !inside;
                }

                if *c == 'F' {
                    check_transitions(&mut x, &mut row, &mut inside, '7', 'J');
                }

                if *c == 'L' {
                    check_transitions(&mut x, &mut row, &mut inside, 'J', '7');
                }

            } else if inside {
                count += 1;
            } 

        }
    }

    Some(count)
}

fn check_transitions(x: &mut i32, row: &mut VecDeque<&char>, inside: &mut bool, no_t: char, t: char) {
    loop {
        *x += 1;
        match row.pop_front() {
            Some(x) if *x == no_t => break,
            Some(x) if *x == t => {
                *inside = !*inside;
                break;
            }
            Some(x) if *x == '-' => continue,
            Some(_) => panic!("not possible"),
            None => panic!("not possible"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }
    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(10));
    }
}
