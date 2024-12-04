use anyhow::Result;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let grid: Vec<Vec<_>> = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            sum += find_xmas(&grid, i, j, 0, 1).unwrap_or(0); // right to left
            sum += find_xmas(&grid, i, j, 0, -1).unwrap_or(0); // left to right
            sum += find_xmas(&grid, i, j, 1, 0).unwrap_or(0); // top to bottom
            sum += find_xmas(&grid, i, j, -1, 0).unwrap_or(0); // bottom to top
            sum += find_xmas(&grid, i, j, 1, 1).unwrap_or(0); // down right diagonal
            sum += find_xmas(&grid, i, j, 1, -1).unwrap_or(0); // down left diagonal
            sum += find_xmas(&grid, i, j, -1, -1).unwrap_or(0); // up left diagonal
            sum += find_xmas(&grid, i, j, -1, 1).unwrap_or(0); // up right diagonal
        }
    }
    println!("{}", sum);
    Ok(())
}

fn find_xmas(
    grid: &Vec<Vec<char>>,
    ii: usize,
    jj: usize,
    i_increment: i32,
    j_increment: i32,
) -> Option<i32> {
    let i = ii as i32;
    let j = jj as i32;
    if grid[ii][jj] == 'X'
        && grid
            .get((i + i_increment) as usize)?
            .get((j + j_increment) as usize)?
            == 'M'
        && grid
            .get((i + 2 * i_increment) as usize)?
            .get((j + 2 * j_increment) as usize)?
            == 'A'
        && grid
            .get((i + 2 * i_increment) as usize)?
            .get((j + 2 * j_increment) as usize)?
            == 'S'
    {
        Some(1)
    }
    None
}
