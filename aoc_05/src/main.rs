use std::{collections::{HashMap, HashSet}, fs::File, io::{self, prelude::*, BufReader}, num::ParseIntError};
use winnow::{ascii::digit1, combinator::{repeat, separated, separated_pair, terminated}, prelude::*, token::literal};

type Rules = HashMap<u32, HashSet<u32>>;

fn main() -> io::Result<()> {
    let filename = "example.txt";
    // let filename = "input.txt";
    let (rules, data) = read_input(filename)?;
    println!("rules={rules:#?}");
    println!("data={data:#?}");
    Ok(())
}

fn read_input(filename: &str) -> io::Result<(Rules, Vec<Vec<u32>>)> {
    let mut buf = String::new();
    File::open(filename)?.read_to_string(&mut buf)?;

    full_input.parse(&buf).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, e.to_string())
    })
}

fn full_input(input: &mut &str) -> PResult<(Rules, Vec<Vec<u32>>)> {
    let rules: Vec<(u32, u32)> = repeat(1.., terminated(rule, literal('\n'))).parse_next(input)?;
    let rules = rules.into_iter().fold(HashMap::new(), |mut acc: HashMap<u32, HashSet<u32>>, (k, v)| {
        acc.entry(k).or_default().insert(v);
        acc
    });
    literal('\n').parse_next(input)?;
    let data: Vec<Vec<u32>> = repeat(1.., terminated(pages, literal('\n'))).parse_next(input)?;
    Ok((rules, data))
}

fn rule(input: &mut &str) -> PResult<(u32, u32)> {
    separated_pair(parse_u32, literal('|'), parse_u32)
        .parse_next(input)
}

fn pages(input: &mut &str) -> PResult<Vec<u32>> {
    separated(1.., parse_u32, literal(','))
        .parse_next(input)
}

fn parse_u32(input: &mut &str) -> PResult<u32> {
    digit1.try_map(|x: &str| x.parse()).parse_next(input)
}
