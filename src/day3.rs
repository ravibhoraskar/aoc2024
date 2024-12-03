use anyhow::Result;
use std::io::BufRead;
use regex::Regex;


fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let input = stdin.lock().lines().next().unwrap()?;
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let muls: Vec<&str> =  re.find_iter(&input).map(|m| m.as_str()).collect();
    println!("{:?}", dates);
    Ok(())
}
