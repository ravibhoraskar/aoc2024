use anyhow::Result;
use std::io::BufRead;
use std::collections::HashSet;
use std::iter::FromIterator;


fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let grid: Vec<Vec<_>> = stdin
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                let reachables = get_reachables(&grid, i, j);
                let set: HashSet<(usize, usize)> = HashSet::from_iter(reachables.iter().cloned());
                sum += set.len();
            }
        }
    }
    println!("{}", sum);
    Ok(())
}

fn get_reachables(grid: &Vec<Vec<usize>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    if grid[i][j] == 9 {
        return vec![(i, j)];
    }
    let mut reachables = vec![];
    for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        if i as i32 + di < 0
            || i as i32 + di >= grid.len() as i32
            || j as i32 + dj < 0
            || j as i32 + dj >= grid[i].len() as i32
        {
            continue;
        }
        let (x, y) = ((i as i32 + di) as usize, (j as i32+ dj) as usize);
        if grid[x][y] == grid[i][j] + 1 {
            reachables.extend(get_reachables(grid, x, y));
        }
    }
    return reachables;
}
