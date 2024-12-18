use anyhow::Result;

use std::io;
use std::io::Read;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut split = input.split("\n\n");

    let mut grid = split
        .next()
        .unwrap()
        .split("\n")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let moves = split
        .next()
        .unwrap()
        .chars()
        .filter(|&c| c != '\n')
        .collect::<Vec<_>>();

    let mut pos = find_robot(&grid).unwrap();

    for mv in moves {
        match mv {
            '>' => {
                pos = do_move(&mut grid, 0, 1, pos);
            }
            '<' => {
                pos = do_move(&mut grid, 0, -1, pos);
            }
            '^' => {
                pos = do_move(&mut grid, -1, 0, pos);
            }
            'v' => {
                pos = do_move(&mut grid, 1, 0, pos);
            }
            _ => {}
        }
    }

    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                sum += i * 100 + j;
            }
        }
    }
    println!("Sum: {}", sum);

    for line in grid {
        for c in line {
            print!("{}", c);
        }
        println!();
    }

    Ok(())
}

fn find_robot(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '@' {
                return Some((i, j));
            }
        }
    }
    None
}

fn do_move(grid: &mut Vec<Vec<char>>, di: i32, dj: i32, init: (usize, usize)) -> (usize, usize) {
    let mut i = init.0 as i32;
    let mut j = init.1 as i32;
    while within_bounds(grid, i, j) && grid[i as usize][j as usize] != '.' {
        i = i + di;
        j = j + dj;
    }
    if !within_bounds(grid, i, j) {
        return init;
    }
    while i != (init.0 as i32) || j != (init.1 as i32) {
        grid[i as usize][j as usize] = grid[(i - di) as usize][(j - dj) as usize];
        i = i - di;
        j = j - dj;
    }
    grid[i as usize][j as usize] = '.';
    ((i + di) as usize, (j + dj) as usize)
}

fn within_bounds(grid: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    i >= 0
        && (i as usize) < grid.len()
        && j >= 0
        && (j as usize) < grid[0].len()
        && grid[i as usize][j as usize] != '#'
}
