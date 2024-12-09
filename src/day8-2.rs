use anyhow::Result;
use std::collections::HashMap;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let grid: Vec<Vec<_>> = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut antennas = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch != '.' {
                let entry = antennas.entry(ch).or_insert(vec![]);
                entry.push((i, j));
            }
        }
    }

    let mut newgrid = vec![vec!['.'; grid[0].len()]; grid.len()];
    for (_, coords) in antennas {
        for i in 0..coords.len() {
            for j in 0..i {
                let (x1, y1) = coords[i];
                let (x2, y2) = coords[j];
                let mut inc = 0;
                while in_range(x1, x2, y1, y2, inc, &grid) {
                    newgrid[x_idx(x1, x2, inc)][y_idx(y1, y2, inc)] = '#';
                    inc += 1;
                }
                inc = -1;
                while in_range(x1, x2, y1, y2, inc, &grid) {
                    newgrid[x_idx(x1, x2, inc)][y_idx(y1, y2, inc)] = '#';
                    inc -= 1;
                }
            }
        }
    }
    println!(
        "Sum: {}",
        newgrid
            .iter()
            .map(|row| row.iter().filter(|&&c| c == '#').count())
            .sum::<usize>()
    );

    Ok(())
}

fn in_range(x1: usize, x2: usize, y1: usize, y2: usize, inc: i32, grid: &Vec<Vec<char>>) -> bool {
    let (x1, x2, y1, y2) = (x1 as i32, x2 as i32, y1 as i32, y2 as i32);
    let x = x2 + inc * (x2 - x1);
    let y = y2 + inc * (y2 - y1);
    x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32
}

fn x_idx(x1: usize, x2: usize, inc: i32) -> usize {
    let (x1, x2) = (x1 as i32, x2 as i32);
    let x = x2 + inc * (x2 - x1);
    x as usize
}

fn y_idx(y1: usize, y2: usize, inc: i32) -> usize {
    let (y1, y2) = (y1 as i32, y2 as i32);
    let y = y2 + inc * (y2 - y1);
    y as usize
}
