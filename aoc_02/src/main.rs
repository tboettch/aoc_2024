use std::{fs::File, io::{self, prelude::*, BufReader}};
use Direction::*;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
enum Direction { INC, DEC }

fn main() -> io::Result<()> {
    let filename = "input.txt";
    // let filename = "example.txt";
    let f = File::open(filename)?;
    let report = read_data(BufReader::new(f))?;
    let safe = count_safe(&report);
    println!("safe={safe}");
    Ok(())
}

fn count_safe(data: &Vec<Vec<u64>>) -> u64 {
    data.iter()
        // .inspect(|x| println!("{x:?}, {}", is_safe(*x)))
        .filter(|x| is_safe(*x))
        .count() as u64
}

fn is_safe(data: &Vec<u64>) -> bool {
    if data.len() <= 1 {
        return true;
    }
    let mut prev_direction: Option<Direction> = None;
    for i in 0..(data.len() - 1) {
        let (direction, d) = diff(data[i], data[i+1]);
        if d < 1 || d > 3 {
            return false;
        }
        if let Some(prev_direction) = prev_direction {
            if prev_direction != direction {
                return false;
            }
        } else {
            prev_direction = Some(direction);
        }
    }
    true
}

fn diff(x: u64, y: u64) -> (Direction, u64) {
    if x > y { (DEC, x - y) } else { (INC, y - x) }
}

fn read_data<R: BufRead>(input: R) -> io::Result<Vec<Vec<u64>>> {
    let mut r = Vec::new();
    for line in input.lines() {
        let line = line?;
        let line_iter = line.split_whitespace();
        let level = line_iter.map(parse_u64).collect::<io::Result<Vec<_>>>()?;
        r.push(level);
    }
    Ok(r)
}

fn parse_u64(val: &str) -> io::Result<u64> {
    val.parse().map_err(io::Error::other)
}