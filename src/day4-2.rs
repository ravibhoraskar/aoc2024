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
            sum += find_xmas(&grid, i, j).unwrap_or(0);
        }
    }
    println!("{}", sum);
    Ok(())
}

fn find_xmas(grid: &Vec<Vec<char>>, ii: usize, jj: usize) -> Option<i32> {
    let i = ii as i32;
    let j = jj as i32;
    if *grid.get((i + 1) as usize)?.get((j + 1) as usize)? == 'A'
        && ((*grid.get(ii)?.get(jj)? == 'M'
            && *grid.get((i + 2) as usize)?.get((j + 2) as usize)? == 'S'
            && *grid.get((i + 2) as usize)?.get((j) as usize)? == 'M'
            && *grid.get((i) as usize)?.get((j + 2) as usize)? == 'S')
            || (*grid.get(ii)?.get(jj)? == 'M'
                && *grid.get((i + 2) as usize)?.get((j + 2) as usize)? == 'S'
                && *grid.get((i + 2) as usize)?.get((j) as usize)? == 'S'
                && *grid.get((i) as usize)?.get((j + 2) as usize)? == 'M')
            || (*grid.get(ii)?.get(jj)? == 'S'
                && *grid.get((i + 2) as usize)?.get((j + 2) as usize)? == 'M'
                && *grid.get((i + 2) as usize)?.get((j) as usize)? == 'S'
                && *grid.get((i) as usize)?.get((j + 2) as usize)? == 'M')
            || (*grid.get(ii)?.get(jj)? == 'S'
                && *grid.get((i + 2) as usize)?.get((j + 2) as usize)? == 'M'
                && *grid.get((i + 2) as usize)?.get((j) as usize)? == 'M'
                && *grid.get((i) as usize)?.get((j + 2) as usize)? == 'S'))
    {
        // println!("{} {}", ii, jj);
        Some(1)
    } else {
        None
    }
}
