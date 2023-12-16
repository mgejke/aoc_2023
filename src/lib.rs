mod day;
pub mod template;

use std::ops::{Add, AddAssign};

pub use day::*;
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn new(grid: Vec<Vec<T>>, width: usize, height: usize) -> Self {
        Self {
            grid,
            width,
            height,
        }
    }

    pub fn get(&self, position: &Point) -> Option<&T> {
        if position.x >= 0
            && position.x < self.width as i32
            && position.y >= 0
            && position.y < self.height as i32
        {
            Some(&self.grid[position.y as usize][position.x as usize])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, position: &Point) -> Option<&mut T> {
        if position.x >= 0
            && position.x < self.width as i32
            && position.y >= 0
            && position.y < self.height as i32
        {
            Some(&mut self.grid[position.y as usize][position.x as usize])
        } else {
            None
        }
    }    
}

pub fn parse_to_vec_vec_grid<T>(input: &str, f: fn(char) -> T) -> Grid<T> {
    let grid = input
        .trim()
        .lines()
        .map(|line| line.chars().map(f).collect_vec())
        .collect_vec();

    let height = grid.len();
    let width = grid[0].len();
    Grid::new(grid, width, height)
}

pub fn vec_vec_grid_with_type<T: Default + Clone>(width: usize, height: usize) -> Grid<T> {
    let grid: Vec<Vec<T>> = vec![
        std::iter::repeat_with(T::default)
            .take(width)
            .collect_vec();
        height
    ];
    Grid::new(grid, width, height)
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
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
