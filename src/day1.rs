use anyhow::Result;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut left = vec![];
    let mut right = vec![];
    for line in stdin.lock().lines() {
        let line = line?;
        let mut split = line.split_whitespace();
        let first = split.next().unwrap().parse::<i32>().unwrap();
        left.push(first);
        let second = split.next().unwrap().parse::<i32>().unwrap();
        right.push(second);
        // println!("first:{} second:{}", first, second);
    }
    left.sort();
    right.sort();
    let mut sum = 0;
    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs();
    }
    println!("sum:{}", sum);
    Ok(())
}
