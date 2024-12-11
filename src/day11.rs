use anyhow::Result;
use memoize::memoize;
use std::io;
use std::io::Read;
//use std::iter;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input = input.trim().to_string();
    let mut sum = 0;
    for num in input.split(' ') {
        sum += stones(num.parse::<i64>().unwrap(), 25);
    }
    println!("Sum: {}", sum);
    Ok(())
}

#[memoize]
fn stones(n: i64, k: i64) -> i64 {
    if k == 0 {
        1
    } else if n == 0 {
        stones(1, k - 1)
    } else if n.to_string().len() % 2 == 0 {
        let nstring = n.to_string();
        stones(nstring[0..nstring.len() / 2].parse::<i64>().unwrap(), k - 1)
            + stones(nstring[nstring.len() / 2..].parse::<i64>().unwrap(), k - 1)
    } else {
        stones(n * 2024, k - 1)
    }
}
