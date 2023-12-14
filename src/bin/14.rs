use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use itertools::Itertools;

advent_of_code::solution!(14);

#[derive(Hash)]
struct Grid {
    grid: Vec<Vec<char>>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>, x: i32, y: i32) -> Self {
        Self {
            grid,
            width: x,
            height: y,
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<&char> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(&self.grid[y as usize][x as usize])
        } else {
            None
        }
    }

    fn swap(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        if (x1 >= 0 && x1 < self.width && y1 >= 0 && y1 < self.height)
            && (x2 >= 0 && x2 < self.width && y2 >= 0 && y2 < self.height)
        {
            (
                self.grid[y1 as usize][x1 as usize],
                self.grid[y2 as usize][x2 as usize],
            ) = (
                self.grid[y2 as usize][x2 as usize],
                self.grid[y1 as usize][x1 as usize],
            );
        }
    }
}

fn parse_grid(input: &str) -> Grid {
    let mut maxx = usize::MIN;
    let mut maxy = usize::MIN;
    let mut grid = Vec::new();
    for (y, lines) in input.trim().lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in lines.chars().enumerate() {
            row.push(c);
            maxx = maxx.max(x);
        }
        grid.push(row);
        maxy = maxy.max(y);
    }
    Grid::new(grid, maxx as i32 + 1, maxy as i32 + 1)
}

fn handle_gravity(grid: &mut Grid, gravity: (i32, i32)) {
    let (yrange, xrange) = match gravity {
        (0, -1) => (
            (1..grid.height).collect_vec(),
            (0..grid.width).collect_vec(),
        ),
        (0, 1) => (
            (0..grid.height - 1).rev().collect_vec(),
            (0..grid.width).collect_vec(),
        ),

        (-1, 0) => (
            (0..grid.height).collect_vec(),
            (1..grid.width).collect_vec(),
        ),
        (1, 0) => (
            (0..grid.height).collect_vec(),
            (0..grid.width - 1).rev().collect_vec(),
        ),
        _ => panic!("not possible"),
    };

    if gravity.0 > 0 {
        loop {
            let mut updated = false;
            for y in &yrange {
                for x in &xrange {
                    updated |= check_update(*x, *y, gravity, grid);
                }
            }
            if !updated {
                break;
            }
        }
    } else {
        loop {
            let mut updated = false;
            for x in &xrange {
                for y in &yrange {
                    updated |= check_update(*x, *y, gravity, grid);
                }
            }
            if !updated {
                break;
            }
        }
    }
}

fn check_update(x: i32, y: i32, gravity: (i32, i32), grid: &mut Grid) -> bool {
    let check_x = x + gravity.0;
    let check_y = y + gravity.1;

    if let (Some('O'), Some('.')) = (grid.get(x, y), grid.get(check_x, check_y)) {
        grid.swap(x, y, check_x, check_y);
        true
    } else {
        false
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn calc_load(grid: &Grid) -> i32 {
    let mut sum = 0;
    for x in 0..grid.width {
        let value: i32 = (0..grid.height)
            .filter_map(|y| {
                if let Some('O') = grid.get(x, y) {
                    let res = grid.height - y;
                    Some(res)
                } else {
                    None
                }
            })
            .sum();
        sum += value;
    }
    sum
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = parse_grid(input);
    handle_gravity(&mut grid, (0, -1));
    let sum = calc_load(&grid);
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = parse_grid(input);

    let mut seen_hashes: HashMap<u64, i32> = HashMap::new();
    let mut scores = Vec::new();
    let loops = 1_000_000_000;

    for i in 0..loops {
        handle_gravity(&mut grid, (0, -1));
        handle_gravity(&mut grid, (-1, 0));
        handle_gravity(&mut grid, (0, 1));
        handle_gravity(&mut grid, (1, 0));

        let hash = calculate_hash(&grid);

        if let Some(index) = seen_hashes.get(&hash) {
            let iterations_left = loops - index;
            let left_until_loop = iterations_left % (i - index) - 1;
            let end_index = index + left_until_loop;

            return Some(scores[end_index as usize] as u32);
        }

        let sum = calc_load(&grid);
        scores.push(sum);

        seen_hashes.insert(hash, i);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
