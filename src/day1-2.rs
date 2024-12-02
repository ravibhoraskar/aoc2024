use anyhow::Result;
use std::io::BufRead;
use std::collections::HashMap;


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
    let mut frequency_map = HashMap::new();
    for e in right {
        *frequency_map.entry(e).or_insert(0) += 1;
    }
    let mut sum = 0;
    for e in left {
        sum += frequency_map.get(&e).unwrap_or(&0) * e;
    }

    println!("sum:{}", sum);
    Ok(())
}
