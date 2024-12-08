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
        // println!("");
    }
    println!("Sum: {}", sum);
    Ok(())
}

#[memoize]
fn can_it(total: i64, parts: String) -> bool {
    // println!("  can_it({}, {})", total, parts);
    let total_string = total.to_string();
    match reverse(&parts).split_once(" ") {
        None => match parts.parse::<i64>() {
            Err(_) => false,
            Ok(x) => total == x,
        },
        Some((first, rest)) => {
            let first_string = reverse(&first.to_string());
            let first = first_string.parse::<i64>().unwrap();
            // println!(
            //     "    first_string: {}, first: {}, total_string: {}, endswith?:{}",
            //     first_string,
            //     first,
            //     total_string,
            //     total_string.ends_with(&first_string)
            // );
            if total - first >= 0 && can_it(total - first, reverse(&rest.to_string())) {
                true
            } else if total % first == 0 && can_it(total / first, reverse(&rest.to_string())) {
                true
            } else if total_string.ends_with(&first_string)
                && !(total_string == first_string)
                && can_it(
                    total_string
                        .strip_suffix(&first_string)
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    reverse(&rest.to_string()),
                )
            {
                true
            } else {
                false
            }
        }
    }
}

fn reverse(s: &String) -> String {
    s.chars().rev().collect()
}
