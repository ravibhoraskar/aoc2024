use anyhow::Result;
use regex::Regex;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let re = Regex::new(r"(-?[0-9]+)").unwrap();
    let (height, width, iterations) = (103, 101, 100);
    let mut robots = vec![];
    for line in stdin.lock().lines() {
        let line = line?;
        let mut caps = re.captures_iter(&line);
        let p = (
            &caps.next().unwrap()[0].parse::<i64>().unwrap(),
            &caps.next().unwrap()[0].parse::<i64>().unwrap(),
        );
        let v = (
            &caps.next().unwrap()[0].parse::<i64>().unwrap(),
            &caps.next().unwrap()[0].parse::<i64>().unwrap(),
        );
        let newp = (
            (((p.0 + v.0 * iterations) % width) + width) % width,
            (((p.1 + v.1 * iterations) % height) + height) % height,
        );
        robots.push(newp);
    }
    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    for (x, y) in robots {
        if x < width / 2 {
            if y < height / 2 {
                q1 += 1;
            } else if y > height / 2 {
                q3 += 1;
            }
        } else if x > width / 2 {
            if y < height / 2 {
                q2 += 1;
            } else if y > height / 2 {
                q4 += 1;
            }
        }
    }
    println!("{} ", q1 * q2 * q3 * q4);
    Ok(())
}
