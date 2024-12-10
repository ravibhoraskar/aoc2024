use anyhow::Result;

use std::io;
use std::io::Read;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input = input.trim().to_string();
    // println!("Input: {}", input);
    let mut iter = input.chars();
    let mut unwrapped: Vec<(i64, i64)> = vec![];
    let mut id = 0;
    while let Some(filesize) = iter.next() {
        let filesize = filesize.to_digit(10).unwrap() as i64;
        let gapsize = iter.next().unwrap_or('0').to_digit(10).unwrap() as i64;
        if filesize > 0 {
            unwrapped.push((id, filesize));
        }
        if gapsize > 0 {
            unwrapped.push((-1, gapsize));
        }
        id += 1;
    }
    let mut currentid = id - 1;
    while currentid >= 0 {
        // println!("Current ID: {}", currentid);
        // println!("Unwrapped: {:?}", unwrapped);
        let right = unwrapped.iter().position(|&x| x.0 == currentid).unwrap();
        let (id, length) = unwrapped[right];
        let mut i = 0;
        while i < right {
            let (lid, llength) = unwrapped[i];
            if lid != -1 {
            } else if llength < length {
            } else {
                unwrapped[right] = (-1, length);
                unwrapped.insert(i, (id, length));
                unwrapped[i + 1] = (-1, llength - length);
                unwrapped = clean_up(unwrapped);
                break;
            }
            i += 1;
        }
        currentid -= 1;
    }

    // println!("Unwrapped: {:?}", unwrapped);

    let mut checksum:i64 = 0;
    let mut lengthsofar = 0;
    for (id, length) in unwrapped {
        if id != -1 {
            for i in 0..length {
                checksum += id * (lengthsofar + i);
            }
        }
        lengthsofar += length;
    }
    println!("Checksum: {}", checksum);
    Ok(())
}

fn clean_up(mut unwrapped: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut i = 0;
    while i < unwrapped.len() {
        let (id, length) = unwrapped[i];
        if id != -1 {
            i += 1;
            continue;
        } else if i == unwrapped.len() - 1 {
            i += 1;
            continue;
        } else {
            let (next_id, next_length) = unwrapped[i + 1];
            if next_id != -1 {
                i += 1;
                continue;
            }
            unwrapped[i] = (-1, length + next_length);
            unwrapped.remove(i + 1);
        }
    }
    unwrapped
}
