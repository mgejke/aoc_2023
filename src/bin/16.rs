use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::{Add, AddAssign},
};

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

advent_of_code::solution!(16);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn dir(&self) -> Point {
        let map: HashMap<Direction, Point> = [
            (Direction::Up, Point { x: 0, y: -1 }),
            (Direction::Down, Point { x: 0, y: 1 }),
            (Direction::Left, Point { x: -1, y: 0 }),
            (Direction::Right, Point { x: 1, y: 0 }),
        ]
        .into_iter()
        .collect();

        map[self]
    }
}

#[derive(Clone)]
struct Ray {
    position: Point,
    direction: Direction,
}

impl Ray {
    fn new(position: Point, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }

    fn step(&mut self) {
        self.position += self.direction.dir();
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

enum Object {
    Empty,
    HSplitter,
    VSplitter,
    FMirror,
    BMirror,
}

fn in_bounds(ray: &Ray, width: usize, height: usize) -> bool {
    ray.position.x >= 0
        && ray.position.x < width as i32
        && ray.position.y >= 0
        && ray.position.y < height as i32
}

fn parse_grid<T>(input: &str, f: fn(char) -> T) -> Vec<Vec<T>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().map(f).collect_vec())
        .collect_vec()
}

fn calculate_score(width: usize, height: usize, start_ray: Ray, grid: &[Vec<Object>]) -> usize {
    let mut visited: Vec<Vec<HashSet<Direction>>> = vec![
        std::iter::repeat_with(HashSet::new)
            .take(width)
            .collect_vec();
        height
    ];
    let mut rays = VecDeque::from_iter([start_ray]);
    while let Some(mut ray) = rays.pop_front() {
        loop {
            ray.step();
            if !in_bounds(&ray, width, height) {
                break;
            }

            let v = &mut visited[ray.position.y as usize][ray.position.x as usize];
            if v.contains(&ray.direction) {
                break;
            }
            v.insert(ray.direction);

            let current = &grid[ray.position.y as usize][ray.position.x as usize];
            match (current, &ray.direction) {
                (Object::Empty, _) => continue,
                (Object::HSplitter, Direction::Up | Direction::Down) => {
                    rays.push_back(Ray::new(ray.position, Direction::Left));
                    rays.push_back(Ray::new(ray.position, Direction::Right));
                    break;
                }
                (Object::VSplitter, Direction::Left | Direction::Right) => {
                    rays.push_back(Ray::new(ray.position, Direction::Up));
                    rays.push_back(Ray::new(ray.position, Direction::Down));
                    break;
                }

                (Object::HSplitter, Direction::Left | Direction::Right) => continue,
                (Object::VSplitter, Direction::Up | Direction::Down) => continue,

                (Object::FMirror, Direction::Up) => ray.direction = Direction::Right,
                (Object::FMirror, Direction::Down) => ray.direction = Direction::Left,
                (Object::FMirror, Direction::Left) => ray.direction = Direction::Down,
                (Object::FMirror, Direction::Right) => ray.direction = Direction::Up,

                (Object::BMirror, Direction::Up) => ray.direction = Direction::Left,
                (Object::BMirror, Direction::Down) => ray.direction = Direction::Right,
                (Object::BMirror, Direction::Left) => ray.direction = Direction::Up,
                (Object::BMirror, Direction::Right) => ray.direction = Direction::Down,
            }
        }
    }
    visited.iter().flatten().filter(|hm| !hm.is_empty()).count()
}

fn char_to_object(c: char) -> Object {
    match c {
        '|' => Object::VSplitter,
        '-' => Object::HSplitter,
        '/' => Object::FMirror,
        '\\' => Object::BMirror,
        '.' => Object::Empty,
        _ => panic!("unknown object"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input, char_to_object);
    let height = grid.len();
    let width = grid[0].len();
    let sum = calculate_score(
        width,
        height,
        Ray::new(Point::new(-1, 0), Direction::Right),
        &grid,
    );
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_grid(input, char_to_object);
    let height = grid.len();
    let width = grid[0].len();

    let up_down_max = (0_i32..width as i32)
        .into_par_iter()
        .map(|x| {
            let up = calculate_score(
                width,
                height,
                Ray {
                    position: Point::new(x, height as i32),
                    direction: Direction::Up,
                },
                &grid,
            );
            let down = calculate_score(
                width,
                height,
                Ray {
                    position: Point::new(x, -1),
                    direction: Direction::Down,
                },
                &grid,
            );
            up.max(down)
        })
        .max()
        .unwrap();

    let left_right_max = (0_i32..height as i32)
        .into_par_iter()
        .map(|y| {
            let right = calculate_score(
                width,
                height,
                Ray {
                    position: Point::new(-1, y),
                    direction: Direction::Right,
                },
                &grid,
            );
            let left = calculate_score(
                width,
                height,
                Ray {
                    position: Point::new(width as i32, y),
                    direction: Direction::Left,
                },
                &grid,
            );
            right.max(left)
        })
        .max()
        .unwrap();

    Some(up_down_max.max(left_right_max) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
