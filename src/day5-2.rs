use anyhow::Result;

use std::cmp::Ordering;
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
    for mut update in updates {
        if !is_ok(&orderings, &update) {
            update.sort_by(|a, b| {
                for ordering in &orderings {
                    if ordering[0] == *a && ordering[1] == *b {
                        return Ordering::Less;
                    } else if ordering[0] == *b && ordering[1] == *a {
                        return Ordering::Greater;
                    }
                }
                return Ordering::Equal;
            });
            println!("Update: {:?}", update);
            sum += update[update.len() / 2];
        }
    }
    println!("Sum: {}", sum);
    Ok(())
}

fn is_ok(orderings: &Vec<Vec<i32>>, update: &Vec<i32>) -> bool {
    for ordering in orderings {
        let pos0 = update.iter().position(|n| *n == ordering[0]);
        let pos1 = update.iter().position(|n| *n == ordering[1]);
        if pos0.is_none() || pos1.is_none() {
            continue;
        }
        if pos0.unwrap() > pos1.unwrap() {
            return false;
        }
    }
    return true;
}
