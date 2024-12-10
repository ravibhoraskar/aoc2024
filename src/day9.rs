use anyhow::Result;

use std::io;
use std::io::Read;
use std::iter;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input = input.trim().to_string();
    // println!("Input: {}", input);
    let mut iter = input.chars();
    let mut unwrapped: Vec<i32> = vec![];
    let mut id = 0;
    while let Some(filesize) = iter.next() {
        let gapsize = iter.next().unwrap_or('0');
        unwrapped.append(
            iter::repeat(id)
                .take(filesize.to_digit(10).unwrap() as usize)
                .collect::<Vec<i32>>()
                .as_mut(),
        );
        unwrapped.append(
            iter::repeat(-1)
                .take(gapsize.to_digit(10).unwrap() as usize)
                .collect::<Vec<i32>>()
                .as_mut(),
        );
        id += 1;
    }
    // println!("Unwrapped: {:?}", unwrapped);
    let mut left = unwrapped.iter().position(|&x| x == -1).unwrap();
    let mut right = unwrapped.iter().rposition(|&x| x != -1).unwrap();

    while left < right {
        unwrapped[left] = unwrapped[right];
        unwrapped[right] = -1;
        left = find_next_left(&unwrapped, left);
        right = find_next_right(&unwrapped, right);
    }
    // println!("Unwrapped: {:?}", unwrapped);
    let checksum = unwrapped
        .iter()
        .enumerate()
        .map(|(i, &x)| if x == -1 { 0 } else { i as i64 * x as i64 })
        .sum::<i64>();
    println!("Checksum: {}", checksum);
    Ok(())
}

fn find_next_left(unwrapped: &Vec<i32>, left: usize) -> usize {
    let mut i = left + 1;
    while i < unwrapped.len() && unwrapped[i] != -1 {
        i += 1;
    }
    return i;
}

fn find_next_right(unwrapped: &Vec<i32>, right: usize) -> usize {
    let mut i = right - 1;
    while i >= 0 && unwrapped[i] == -1 {
        i -= 1;
    }
    return i;
}
