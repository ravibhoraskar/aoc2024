use anyhow::Result;
use std::io::BufRead;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut grid: Vec<Vec<_>> = stdin
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
    let mut dir = Direction::Up;
    // println!("Init: {} {} {:?}", x, y, dir);

    while x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32 {
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
        // println!("{} {} {:?}", x, y, dir);
    }

    // for row in &grid {
    //     println!("{}", row.into_iter().collect::<String>());
    // }

    let s: usize = grid
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|c| if c == 'X' { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum();
    println!("Sum: {}", s);

    Ok(())
}
