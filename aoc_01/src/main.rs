use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let (mut left, mut right) = read_data(reader)?;
    left.sort_unstable();
    right.sort_unstable();
    let diff = calculate_diff(&left, &right);
    println!("diff={diff}");
    let similarity = calculate_similarity(&left, &right);
    println!("similarity={similarity}");
    Ok(())
}

fn read_data<R: BufRead>(input: R) -> io::Result<(Vec<i64>, Vec<i64>)> {
    let mut l = Vec::new();
    let mut r = Vec::new();
    for line in input.lines() {
        let line = line?;
        let mut line_iter = line.split_whitespace();
        l.push(parse_i64(line_iter.next())?);
        r.push(parse_i64(line_iter.next())?);
    }
    Ok((l, r))
}

fn calculate_diff(left: &[i64], right: &[i64]) -> i64 {
    left.iter().zip(right.iter()).map(|(x,y)| dist(*x, *y)).sum()
}

fn calculate_similarity(left: &[i64], right: &[i64]) -> i64 {
    let counts = calculate_counts(right);
    let mut sum = 0;
    for val in left {
        sum += val * counts.get(val).unwrap_or(&0);
    }
    sum
}

fn calculate_counts(data: &[i64]) -> HashMap<i64, i64> {
    let mut r = HashMap::new();
    for val in data.iter() {
        *r.entry(*val).or_insert(0) += 1i64;
    }
    r
}

fn parse_i64(val: Option<&str>) -> io::Result<i64> {
    val.ok_or(io::Error::new(io::ErrorKind::Other, "Expected another integer on the line")).and_then(|x| x.parse().map_err(io::Error::other))
}

fn dist(x: i64, y: i64) -> i64 {
    if x > y { x - y} else { y - x}
}
