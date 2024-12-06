use std::{fs::File, io::{self, prelude::*, BufReader}};
use Direction::*;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
enum Direction { INC, DEC }

fn main() -> io::Result<()> {
    let report;
    {
        let filename = "input.txt";
        // let filename = "example.txt";
        let f = File::open(filename)?;
        report = read_data(BufReader::new(f))?;
    }
    let safe = count_safe(&report);
    println!("safe={safe}");
    let safe_flexible = count_safe_flexible(&report);
    println!("safe_flexible={safe_flexible}");
    Ok(())
}

fn count_safe(data: &Vec<Vec<u64>>) -> u64 {
    data.iter()
        // .inspect(|x| println!("{x:?}, {}", is_safe(*x)))
        .filter(|x| is_safe(*x))
        .count() as u64
}

fn count_safe_flexible(data: &Vec<Vec<u64>>) -> u64 {
    data.iter()
        // .inspect(|x| println!("{x:?}, {}", is_safe_flexible(*x)))
        .filter(|x| is_safe_flexible(*x))
        .count() as u64
}

fn is_safe(data: &Vec<u64>) -> bool {
    if data.len() <= 1 {
        return true;
    }
    let mut base_direction: Option<Direction> = None;
    for i in 0..(data.len() - 1) {
        let (direction, d) = diff(data[i], data[i+1]);
        if d < 1 || d > 3 {
            return false;
        }
        if let Some(base_direction) = base_direction {
            if base_direction != direction {
                return false;
            }
        } else {
            base_direction = Some(direction);
        }
    }
    true
}

fn diff(x: u64, y: u64) -> (Direction, u64) {
    if x > y { (DEC, x - y) } else { (INC, y - x) }
}

fn is_safe_flexible(data: &Vec<u64>) -> bool {
    subsets(data).any(|x| is_safe(&x))
}

fn subsets(data: &Vec<u64>) -> impl Iterator<Item = Vec<u64>> + '_ {
    std::iter::once(data.clone()).chain((0..data.len()).map(|i| {
        let mut r = Vec::with_capacity(data.len() - 1);
        r.extend(&data[0..i]);
        r.extend(&data[i+1..]);
        assert_eq!(r.len(), data.len() - 1);
        r
    }))
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