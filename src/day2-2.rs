use anyhow::Result;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut sum = 0;
    for line in stdin.lock().lines() {
        let line = line?;
        let split = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        for i in 0..split.len() {
            let mut split2 = split.clone();
            split2.remove(i);
            if is_safe(&split2) {
                sum += 1;
                break;
            }
        }
    }
    println!("sum:{}", sum);
    Ok(())
}

fn is_safe(split: &Vec<i32>) -> bool {
    let increases = split[0] < split[1];
    for i in 1..split.len() {
        let mut diff = split[i] - split[i - 1];
        if (increases && diff < 0) || (!increases && diff > 0) {
            return false;
        }
        diff = diff.abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    return true;
}
