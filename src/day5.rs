use anyhow::Result;

use std::io;
use std::io::Read;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut split = input.split("\n\n");

    let orderings: Vec<Vec<i32>> = split
        .next()
        .unwrap()
        .split("\n")
        .map(|line| line.split("|").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();

    let updates: Vec<Vec<i32>> = split
        .next()
        .unwrap()
        .trim()
        .split("\n")
        .map(|line| line.split(",").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();
    let mut sum = 0;
    for update in updates {
        sum += middle_number_if_ok(&orderings, update).unwrap_or(0);
    }
    println!("Sum: {}", sum);
    Ok(())
}

fn middle_number_if_ok(orderings: &Vec<Vec<i32>>, update: Vec<i32>) -> Option<i32> {
    for ordering in orderings {
        let pos0 = update.iter().position(|n| *n == ordering[0]);
        let pos1 = update.iter().position(|n| *n == ordering[1]);
        if pos0.is_none() || pos1.is_none() {
            continue;
        }
        if pos0.unwrap() > pos1.unwrap() {
            return None;
        }
    }
    return Some(update[update.len() / 2]);
}
