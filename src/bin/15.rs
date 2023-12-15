use std::collections::VecDeque;

advent_of_code::solution!(15);

#[derive(Clone, Debug)]
struct Lens<'a> {
    name: &'a str,
    count: u32,
}

impl<'a> PartialEq for Lens<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<'a> Lens<'a> {
    fn new(name: &'a str, count: u32) -> Self {
        Self { name, count }
    }
}

#[derive(Debug)]
enum CommandType<'a> {
    Insert(u32, Lens<'a>),
    Remove(u32, Lens<'a>),
}

fn hash(string: &str) -> u32 {
    string.as_bytes().iter().fold(0, |mut acc: u32, value| {
        acc += *value as u32;
        acc *= 17;
        acc % 256
    })
}

fn find_lens_index(vecdeq: &VecDeque<Lens>, lens: &Lens) -> Option<usize> {
    vecdeq.iter().position(|l| l == lens)
}

fn perform_commands<'a>(
    commands: impl Iterator<Item = CommandType<'a>>,
) -> Vec<VecDeque<Lens<'a>>> {
    let mut store = vec![VecDeque::with_capacity(10); 256];
    for command in commands {
        match command {
            CommandType::Insert(box_index, lens) => {
                if let Some(lens_index) = find_lens_index(&store[box_index as usize], &lens) {
                    store[box_index as usize][lens_index] = lens;
                } else {
                    store[box_index as usize].push_back(lens);
                }
            }
            CommandType::Remove(box_index, lens) => {
                if let Some(lens_index) = find_lens_index(&store[box_index as usize], &lens) {
                    store[box_index as usize].remove(lens_index);
                }
            }
        }
    }
    store
}
fn calculate_focus_power(lens_boxes: Vec<VecDeque<Lens<'_>>>) -> usize {
    lens_boxes
        .iter()
        .enumerate()
        .flat_map(|(outer, vec)| {
            vec.iter()
                .enumerate()
                .map(move |(inner, lens)| (outer + 1) * (inner + 1) * lens.count as usize)
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.trim().split(',').map(hash).sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let commands = input.trim().split(',').map(|line| {
        if line.contains('=') {
            let (name, value) = line.split_once('=').unwrap();
            CommandType::Insert(hash(name), Lens::new(name, value.parse().unwrap()))
        } else {
            let name = &line[..line.len() - 1];
            CommandType::Remove(hash(name), Lens::new(name, 0))
        }
    });

    let lens_boxes = perform_commands(commands);
    let focus_power = calculate_focus_power(lens_boxes);
    Some(focus_power as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
