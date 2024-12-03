use anyhow::Result;
use regex::Regex;
use std::io;
use std::io::Read;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don\'t\(\)").unwrap();
    let parts = re
        .find_iter(&input)
        .map(|m| m.as_str())
        .collect::<Vec<&str>>();
    //    println!("{:?}", parts);

    let mut enabled = true;
    let mut sum = 0;
    for part in parts {
        match part {
            "do()" => {
                enabled = true;
            }
            "don't()" => {
                enabled = false;
            }
            _ => {
                if !enabled {
                    continue;
                }
                let re = Regex::new(r"[0-9]+").unwrap();
                sum += re
                    .find_iter(part)
                    .map(|m| m.as_str().parse::<i64>().unwrap() as i64)
                    // .map(|m| {
                    //     println!("product: {:?}", m);
                    //     m
                    // })
                    .product::<i64>();
            }
        }
    }
    println!("sum: {:?}", sum);
    Ok(())
}
