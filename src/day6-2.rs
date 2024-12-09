use anyhow::Result;
use std::io::BufRead;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let grid: Vec<Vec<_>> = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let (mut x, mut y) = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '^' {
                x = i as i32;
                y = j as i32;
            }
        }
    }
    let dir = Direction::Up;

    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let mut gridclone = grid.clone();
            gridclone[i][j] = '#';
            if does_it_loop(gridclone, x, y, dir) {
                count += 1;
            }
        }
    }
    println!("{}", count);

    Ok(())
}

fn does_it_loop(mut grid: Vec<Vec<char>>, mut x: i32, mut y: i32, mut dir: Direction) -> bool {
    let mut loop_count = 0;
    while x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32 {
        if loop_count > grid.len() * grid[0].len() {
            return true;
        }
        loop_count += 1;
        if grid[x as usize][y as usize] == '#' {
            // move back one step
            match dir {
                Direction::Up => x += 1,
                Direction::Right => y -= 1,
                Direction::Down => x -= 1,
                Direction::Left => y += 1,
            }
            dir = match dir {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            }
        }
        grid[x as usize][y as usize] = 'X';
        match dir {
            Direction::Up => x -= 1,
            Direction::Right => y += 1,
            Direction::Down => x += 1,
            Direction::Left => y -= 1,
        }
    }
    false
}
