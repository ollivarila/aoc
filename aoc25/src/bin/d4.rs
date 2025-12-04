fn main() {
    solve_pt1();
    solve_pt2();
}

aoc25::input!("d4");

fn solve_pt1() {
    let solution = accessible_paper_rolls(INPUT, true);
    println!("{solution}");
}

fn solve_pt2() {
    let solution = accessible_paper_rolls(INPUT, false);
    println!("{solution}");
}

fn accessible_paper_rolls(input: &str, is_pt1: bool) -> u32 {
    let mut grid = make_grid(input);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;
    let mut removed = false;

    loop {
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == 0 {
                    continue;
                }

                let neighbors = compute_neighbors(&grid, row, col);
                if neighbors < 4 {
                    count += 1;
                    if !is_pt1 {
                        grid[row][col] = 0;
                        removed = true;
                    }
                }
            }
        }

        if !removed || is_pt1 {
            break;
        }

        removed = false;
    }
    count
}

fn compute_neighbors(grid: &Vec<Vec<u8>>, row: usize, col: usize) -> u8 {
    let cols = grid[0].len() - 1;
    let rows = grid.len() - 1;
    let mut n = 0;

    let start_row = row.checked_sub(1).unwrap_or(0);
    let start_col = col.checked_sub(1).unwrap_or(0);
    let end_row = rows.min(row + 1);
    let end_col = cols.min(col + 1);

    for i in start_row..=end_row {
        for j in start_col..=end_col {
            if i == row && j == col {
                continue;
            }

            n += grid[i][j];
        }
    }

    n
}

fn make_grid(input: &str) -> Vec<Vec<u8>> {
    let mut acc = vec![];

    for row in input.lines() {
        let mut row_acc = Vec::with_capacity(row.len());
        for c in row.chars() {
            let n = match c {
                '.' => 0,
                '@' => 1,
                _ => unreachable!(),
            };
            row_acc.push(n);
        }
        acc.push(row_acc);
    }

    acc
}
