use std::{ops::Deref, str::Chars};

use crate::Solution;

const INPUT: &str = include_str!("../inputs/day4_input.txt");

pub struct Part1;
pub struct Part2;

#[derive(Debug, Clone)]
struct Grid<'a> {
    original: Chars<'a>,
    /// Index by data[row][col]
    data: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    current_row: usize,
    current_col: usize,
}

impl Deref for Grid<'_> {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

fn parse_input(input: &str) -> Grid {
    let rows = input.lines().count();
    let cols = input.lines().next().expect("first row").chars().count();
    let mut grid = vec![];

    for row in input.lines() {
        let col = row.chars().collect();
        grid.push(col);
    }

    Grid {
        original: input.chars(),
        data: grid,
        rows,
        cols,
        current_row: 0,
        current_col: 0,
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    fn next_pos(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        let row_sub = row.checked_sub(1);
        let col_sub = col.checked_sub(1);

        match self {
            Direction::Up => row_sub.map(|row| (row, col)),
            Direction::UpRight => row_sub.map(|row| (row, col + 1)),
            Direction::Right => Some((row, col + 1)),
            Direction::DownRight => Some((row + 1, col + 1)),
            Direction::Down => Some((row + 1, col)),
            Direction::DownLeft => col_sub.map(|col| (row + 1, col)),
            Direction::Left => col_sub.map(|col| (row, col)),
            Direction::UpLeft => col_sub.and_then(|col| row_sub.map(|row| (row, col))),
        }
    }
}

#[derive(Debug, Clone)]
struct GridItem {
    value: char,
    row: usize,
    col: usize,
}

impl Iterator for Grid<'_> {
    type Item = GridItem;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.original.next();

        if matches!(value, Some('\n')) {
            self.current_row += 1;
            self.current_col = 0;
            return self.next();
        }
        self.current_col += 1;

        value.map(|c| GridItem {
            value: c,
            row: self.current_row,
            col: self.current_col - 1,
        })
    }
}

impl Solution for Part1 {
    const FOR: &'static str = "Day 4 part 1";
    const INPUT: &'static str = INPUT;
    type SolutionOutput = usize;
    fn solution(&self, input: &str) -> Self::SolutionOutput {
        let grid = parse_input(input);
        let mut count = 0;

        for item in grid.clone() {
            match item.value {
                'X' => {
                    let row = item.row;
                    let col = item.col;
                    let res = check_directions(&grid, row, col);
                    count += res;
                }
                _ => continue,
            };
        }

        count
    }
}

const ALL_DIRECTIONS: &[Direction] = &[
    Direction::Up,
    Direction::UpRight,
    Direction::Right,
    Direction::DownRight,
    Direction::Down,
    Direction::DownLeft,
    Direction::Left,
    Direction::UpLeft,
];

fn check_directions(grid: &Grid, row: usize, col: usize) -> usize {
    let mut count = 0;
    for direction in ALL_DIRECTIONS {
        if check_xmas(grid, row, col, direction, 'X', 0) {
            count += 1;
        }
    }

    count
}

fn next_char(cur: char) -> char {
    // XMAS
    match cur {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        _ => unreachable!(),
    }
}

fn check_xmas(
    grid: &Grid,
    row: usize, // Valid row
    col: usize, // Valid col
    direction: &Direction,
    char_to_check: char,
    so_far: usize, // How many have we checked
) -> bool {
    let value = grid[row][col];

    if value != char_to_check {
        return false;
    }

    if char_to_check == 'S' && so_far == 3 {
        return true;
    }

    let maybe_next_pos = direction.next_pos(row, col);
    if maybe_next_pos.is_none() {
        return false;
    }

    if !next_is_in_bounds(grid, row, col, direction) {
        return false;
    }

    let (row, col) = maybe_next_pos.unwrap();

    return check_xmas(
        grid,
        row,
        col,
        direction,
        next_char(char_to_check),
        so_far + 1,
    );
}

fn next_is_in_bounds(grid: &Grid, row: usize, col: usize, direction: &Direction) -> bool {
    if row == 0
        && matches!(
            direction,
            Direction::UpLeft | Direction::UpRight | Direction::Up
        )
    {
        return false;
    }

    if col == 0
        && matches!(
            direction,
            Direction::UpLeft | Direction::Left | Direction::DownLeft
        )
    {
        return false;
    }

    if col + 1 >= grid.cols
        && matches!(
            direction,
            Direction::UpRight | Direction::Right | Direction::DownRight
        )
    {
        return false;
    }

    if row + 1 >= grid.rows
        && matches!(
            direction,
            Direction::Down | Direction::DownLeft | Direction::DownRight
        )
    {
        return false;
    }

    return true;
}

impl Solution for Part2 {
    const FOR: &'static str = "Day 4 part 2";
    const INPUT: &'static str = INPUT;
    type SolutionOutput = usize;
    fn solution(&self, input: &str) -> Self::SolutionOutput {
        let grid = parse_input(input);
        let mut count = 0;

        for item in grid.clone() {
            match item.value {
                'A' => count += check_xmas_pt2(&grid, item.row, item.col),
                _ => continue,
            };
        }

        count
    }
}

fn check_xmas_pt2(grid: &Grid, row: usize, col: usize) -> usize {
    let square_grid = grid.square((row, col));

    if square_grid.is_none() {
        return 0;
    }

    let mut square_grid = square_grid.unwrap();

    for _ in 0..4 {
        if is_xmas(&square_grid) {
            return 1;
        }
        square_grid = square_grid.rotate_right()
    }

    0
}

/// is:
/// M.M
/// .A.
/// S.S
/// or
fn is_xmas(square_grid: &Grid) -> bool {
    if !matches!(square_grid[0].as_slice(), &['M', _, 'M']) {
        return false;
    }

    if !matches!(square_grid[1].as_slice(), &[_, 'A', _]) {
        return false;
    }

    if !matches!(square_grid[2].as_slice(), &['S', _, 'S']) {
        return false;
    }

    true
}

impl Grid<'_> {
    fn square(&self, (row, col): (usize, usize)) -> Option<Grid> {
        if row == 0 {
            return None;
        }

        if col == 0 {
            return None;
        }

        if row + 1 >= self.rows {
            return None;
        }

        if col + 1 >= self.cols {
            return None;
        }

        let mut data = vec![];

        for i in 0..3 {
            let row = &self[row + i - 1][col - 1..=col + 1];
            data.push(row.to_vec());
        }

        Some(Grid {
            original: "".chars(),
            data,
            rows: 3,
            cols: 3,
            current_row: 0,
            current_col: 0,
        })
    }

    fn rotate_right(mut self) -> Self {
        let mut new_data = vec![];

        for i in 0..self.cols {
            let mut row = vec![];
            for j in (0..self.rows).rev() {
                row.push(self[j][i]);
            }
            new_data.push(row);
        }

        self.data = new_data;
        self.rows = self.data.len();
        self.cols = self.data[0].len();
        self
    }
}

#[cfg(test)]
mod should {

    use super::*;
    const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn compute_part1_example() {
        let output = Part1.solution(EXAMPLE);
        assert_eq!(output, 18);
    }

    #[test]
    fn check_for_xmas() {
        let input = "SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let grid = parse_input(input);
        assert_eq!(grid[3][3], 'X');

        let is_xmas = check_xmas(&grid, 3, 3, &Direction::UpLeft, 'X', 0);
        assert!(is_xmas);

        let is_xmas = check_xmas(&grid, 3, 3, &Direction::UpRight, 'X', 0);
        assert!(is_xmas);
    }

    #[test]
    fn compute_part2_example() {
        let output = Part2.solution(EXAMPLE);
        assert_eq!(output, 9);
    }

    #[test]
    fn make_square_grid() {
        let grid = parse_input(EXAMPLE);
        let square_grid = grid.square((0, 0));
        assert!(square_grid.is_none());

        let square_grid = grid.square((1, 1));
        assert!(square_grid.is_some());
        assert_eq!(square_grid.unwrap()[0].len(), 3);

        let square_grid = grid.square((3, 3));
        assert!(square_grid.is_some());

        let square_grid = grid.square((10, 10));
        assert!(square_grid.is_none());

        let square_grid = grid.square((8, 8));
        assert!(square_grid.is_some());
    }

    #[test]
    fn rotate_grid() {
        let grid = parse_input("AAA\nBBB\nCCC");
        let rotated = grid.rotate_right();
        assert_eq!(rotated[0][0], 'C');
        assert_eq!(rotated[0][1], 'B');
        assert_eq!(rotated[0][2], 'A');
    }

    #[test]
    fn see_xmas() {
        let input = "MSM\nSAX\nSAS";

        let grid = parse_input(input);
        let is_xmas = is_xmas(&grid);
        assert!(is_xmas);
    }

    #[test]
    fn see_xmas_from_example_input() {
        let grid = parse_input(EXAMPLE);
        let square_grid = grid.square((1, 2)).unwrap().rotate_right();
        let is_xmas = is_xmas(&square_grid);
        assert!(is_xmas);
    }
}
