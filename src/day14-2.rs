use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let re = Regex::new(r"(-?[0-9]+)").unwrap();
    let (height, width) = (103, 101);
    let mut robots: Vec<((i64, i64), (i64, i64))> = vec![];
    for line in stdin.lock().lines() {
        let line = line?;
        let mut caps = re.captures_iter(&line);
        let p = (
            caps.next().unwrap()[0].parse::<i64>().unwrap(),
            caps.next().unwrap()[0].parse::<i64>().unwrap(),
        );
        let v = (
            caps.next().unwrap()[0].parse::<i64>().unwrap(),
            caps.next().unwrap()[0].parse::<i64>().unwrap(),
        );
        robots.push((p, v));
    }
    for i in 0..100000 {
        let clusters = get_clusters(&robots.clone().into_iter().map(|r| r.0).collect::<Vec<_>>());
        let highest_frequency = highest_frequency(&clusters);
        if highest_frequency > 10 {
            for i in 0..height {
                for j in 0..width {
                    if clusters.contains_key(&(j, i)) {
                        print!("*");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }
            println!("\n{}: {}", i, highest_frequency);
            println!(
                "{:#?}",
                robots.clone().into_iter().map(|r| r.0).collect::<Vec<_>>()
            );
            break;
        }
        let mut newrobots = vec![];
        for robot in robots {
            let newrobot = (
                (
                    (((robot.0 .0 + robot.1 .0) % width) + width) % width,
                    (((robot.0 .1 + robot.1 .1) % height) + height) % height,
                ),
                robot.1,
            );
            newrobots.push(newrobot);
        }
        robots = newrobots;
    }
    Ok(())
}

fn get_clusters(robots: &Vec<(i64, i64)>) -> HashMap<(i64, i64), i64> {
    let mut clusters = HashMap::new();
    let mut cur_cluster = 0;
    for robot in robots {
        let mut inserted = false;
        for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let neighbor = (robot.0 + di, robot.1 + dj);
            if clusters.contains_key(&neighbor) {
                clusters.insert(*robot, clusters[&neighbor]);
                inserted = true;
                break;
            }
        }
        if !inserted {
            clusters.insert(*robot, cur_cluster);
            cur_cluster += 1;
        }
    }
    println!("{:#?}", clusters);
    clusters
}

fn highest_frequency(map: &HashMap<(i64, i64), i64>) -> i64 {
    let mut frequency_map = HashMap::new();
    for (_, value) in map {
        *frequency_map.entry(value.clone()).or_insert(0) += 1;
    }
    *frequency_map.values().max().unwrap()
}
