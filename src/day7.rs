use anyhow::Result;
use memoize::memoize;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut sum = 0;
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let (first, rest) = line.split_once(": ").unwrap();
        let first = first.parse::<i64>()?;
        if can_it(first, rest.to_string()) {
            sum += first;
        }
    }
    println!("Sum: {}", sum);
    Ok(())
}

#[memoize]
fn can_it(total: i64, parts: String) -> bool {
    // println!("  can_it({}, {})", total, parts);
    match reverse(&parts).split_once(" ") {
        None => total == parts.parse::<i64>().unwrap(),
        Some((first, rest)) => {
            let first = reverse(&first.to_string()).parse::<i64>().unwrap();
            if can_it(total - first, reverse(&rest.to_string())) {
                true
            } else if total % first == 0 {
                can_it(total / first, reverse(&rest.to_string()))
            } else {
                false
            }
        }
    }
}

fn reverse(s: &String) -> String {
    s.chars().rev().collect()
}
