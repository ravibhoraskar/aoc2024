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

        let increases = split[0] < split[1];
        let mut fails = false;
        for i in 1..split.len() {
            let mut diff = split[i] - split[i - 1];
            if (increases && diff < 0) || (!increases && diff > 0) {
                fails = true;
                break;
            }
            diff = diff.abs();
            if diff < 1 || diff > 3 {
                fails = true;
                break;
            }
        }
        if !fails {
            sum += 1;
        }
    }
    println!("sum:{}", sum);
    Ok(())
}
