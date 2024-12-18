use anyhow::Result;
use priority_queue::PriorityQueue;
use std::collections::HashSet;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let gridlen = 71;
    let numtoread = 1024;
    let corruptedbytes: Vec<_> = stdin
        .lock()
        .lines()
        .take(numtoread)
        .map(|x| {
            let v: Vec<_> = x
                .unwrap()
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            (v[0], v[1])
        })
        .collect();
    println!("Shortest path: {}", -shortest_path(corruptedbytes, gridlen));
    Ok(())
}

fn shortest_path(corruptedbytes: Vec<(i64, i64)>, gridlen: i64) -> i64 {
    let mut pq = PriorityQueue::new();
    let mut visited = HashSet::new();
    pq.push((0, 0), 0);
    visited.insert((0, 0));

    while !visited.contains(&(gridlen - 1, gridlen - 1)) {
        let ((i, j), d) = pq.pop().unwrap();
        if i < 0 || j < 0 || i >= gridlen || j >= gridlen || corruptedbytes.contains(&(i, j)) {
            continue;
        }
        for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let newi = i + di;
            let newj = j + dj;
            if visited.contains(&(newi, newj)) {
                continue;
            }
            pq.push((newi, newj), d - 1);
            visited.insert((newi, newj));
        }
    }
    *pq.get(&(gridlen - 1, gridlen - 1)).unwrap().1
}
