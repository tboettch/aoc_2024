use std::{fs::File, io::{self, prelude::*}};
use winnow::{ascii::digit1, combinator::{repeat, separated, terminated, Repeat}, prelude::*, token::literal};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Equation {
    total: u64,
    components: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Op {Add, Mul}

impl Op {
    const ALL: [Op; 2] = [Op::Add, Op::Mul];

    fn apply(&self, l: u64, r: u64) -> u64 {
        match self {
            Op::Add => l + r,
            Op::Mul => l * r,
        }
    }
}

fn main() -> io::Result<()> {
    let filename = "example.txt";
    // let filename = "input.txt";
    let equations = read_input(filename);
    println!("{equations:?}");
    Ok(())
}

fn read_input(filename: &str) -> io::Result<Vec<Equation>> {
    let mut buf = String::new();
    File::open(filename)?.read_to_string(&mut buf)?;
    equations.parse(&buf).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, e.to_string())
    })
}

fn equations(input: &mut &str) -> PResult<Vec<Equation>> {
    repeat(1.., terminated(equation, literal('\n'))).parse_next(input)
}

fn equation(input: &mut &str) -> PResult<Equation> {
    let (total, _, components) = (parse_u64, literal(": "), separated(2.., parse_u64, " ")).parse_next(input)?;
    Ok(Equation { total, components })
}

fn parse_u64(input: &mut &str) -> PResult<u64> {
    digit1.try_map(|s: &str| s.parse()).parse_next(input)
}
