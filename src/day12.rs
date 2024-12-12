use anyhow::Result;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let grid: Vec<Vec<_>> = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let mut seen: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !seen[i][j] {
                let (area, perimeter) = process(&grid, &mut seen, i, j);
                sum += area * perimeter;
            }
        }
    }

    println!("{}", sum);
    Ok(())
}

fn process(grid: &Vec<Vec<char>>, seen: &mut Vec<Vec<bool>>, i: usize, j: usize) -> (i32, i32) {
    let mut area = 1;
    let mut perimeter = 0;
    seen[i][j] = true;
    for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let ni = i as i32 + di;
        let nj = j as i32 + dj;
        if ni < 0
            || ni >= grid.len() as i32
            || nj < 0
            || nj >= grid[0].len() as i32
            || grid[ni as usize][nj as usize] != grid[i][j]
        {
            perimeter += 1;
            continue;
        }
        if !seen[ni as usize][nj as usize] {
            let (a, p) = process(grid, seen, ni as usize, nj as usize);
            area += a;
            perimeter += p;
        }
    }
    (area, perimeter)
}
