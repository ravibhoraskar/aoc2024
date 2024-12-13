use anyhow::Result;

use memoize::memoize;
use std::io;
use std::io::Read;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let split = input.split("\n\n");

    let mut sum = 0;
    for input in split {
        let mut split = input.split("\n");
        let btna = getbtn(split.next().unwrap());
        let btnb = getbtn(split.next().unwrap());
        let prize = getprize(split.next().unwrap());
        let min = mintokens((0, 0), btna, btnb, prize);
        if min.is_some() {
            sum += min.unwrap();
        }
    }
    println!("{}", sum);
    Ok(())
}

#[memoize]
fn mintokens(
    cur: (i64, i64),
    btna: (i64, i64),
    btnb: (i64, i64),
    prize: (i64, i64),
) -> Option<i64> {
    if cur == prize {
        return Some(0);
    }
    if cur.0 > prize.0 || cur.1 > prize.1 {
        return None;
    }
    let mina = mintokens((cur.0 + btna.0, cur.1 + btna.1), btna, btnb, prize);
    let minb = mintokens((cur.0 + btnb.0, cur.1 + btnb.1), btna, btnb, prize);
    if mina.is_none() && minb.is_none() {
        return None;
    }
    if mina.is_none() {
        return Some(minb.unwrap() + 1);
    }
    if minb.is_none() {
        return Some(mina.unwrap() + 3);
    }
    let mina = mina.unwrap() + 3;
    let minb = minb.unwrap() + 1;
    return Some(std::cmp::min(mina, minb));
}

fn getbtn(input: &str) -> (i64, i64) {
    let mut split = input.split(" ");
    split.next();
    split.next();
    let mut x = split.next().unwrap().split(',').next().unwrap().split('+');
    x.next();
    let x = x.next().unwrap().parse::<i64>().unwrap();
    let mut y = split.next().unwrap().split('+');
    y.next();
    let y = y.next().unwrap().parse::<i64>().unwrap();
    (x, y)
}

fn getprize(input: &str) -> (i64, i64) {
    let mut split = input.split(" ");
    split.next();
    let mut x = split.next().unwrap().split(',').next().unwrap().split('=');
    x.next();
    let x = x.next().unwrap().parse::<i64>().unwrap();
    let mut y = split.next().unwrap().split('=');
    y.next();
    let y = y.next().unwrap().parse::<i64>().unwrap();
    (x, y)
}
