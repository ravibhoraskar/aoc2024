use anyhow::Result;
use regex::Regex;
use std::io;
use std::io::Read;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let sum = re
        .find_iter(&input)
        .map(|m| m.as_str())
        .map(|m| {
            let re = Regex::new(r"[0-9]+").unwrap();
            re.find_iter(m)
                .map(|m| m.as_str().parse::<i64>().unwrap() as i64)
                .product::<i64>()
        })
        .sum::<i64>();
    println!("{:?}", sum);
    Ok(())
}
