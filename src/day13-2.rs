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
        let min = mintokens(btna, btnb, prize);
        if min.is_some() {
            sum += min.unwrap();
        }
    }
    println!("{}", sum);
    Ok(())
}

#[memoize]
fn mintokens(btna: (i64, i64), btnb: (i64, i64), prize: (i64, i64)) -> Option<i64> {
    // btna.0 * a + btnb.0 * b = prize.0
    // btna.1 * a + btnb.1 * b = prize.1

    // x = b2 c1 - b1 c2 / (a1 b2 - a2 b1)
    // y = a1 c2 - a2 c1 / (a1 b2 - a2 b1)

    let anum = btnb.1 * prize.0 - btnb.0 * prize.1;
    let bnum = btna.0 * prize.1 - btna.1 * prize.0;
    let denom = btna.0 * btnb.1 - btna.1 * btnb.0;
    if (anum % denom == 0) && (bnum % denom == 0) {
        return Some(3 * (anum / denom) + (bnum / denom));
    } else {
        return None;
    };
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
    (x + 10000000000000, y + 10000000000000)
}
