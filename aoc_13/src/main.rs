use std::{fs::File, io::{self, Read}};

#[derive(Debug, Clone)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Button(u32, u32);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Prize(u32, u32);

fn main() -> io::Result<()>{
    let filename = "example.txt";
    // let filename = "input.txt";
    let machines = read_data(filename)?;
    println!("machines: {machines:?}");
    Ok(())
}

fn read_data(filename: &str) -> io::Result<Vec<Machine>> {
    let mut buf = String::new();
    File::open(filename)?.read_to_string(&mut buf)?;
    parser::parse(&buf)
}

mod parser {
    use winnow::{ascii::digit1, combinator::{separated, separated_pair, terminated}, prelude::*, token::{literal, one_of}};
    use super::*;

    pub fn parse(input: &str) -> io::Result<Vec<Machine>> {
        machines.parse(input).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, e.to_string())
        })
    }

    fn machines(input: &mut &str) -> PResult<Vec<Machine>> {
        separated(0.., machine, literal('\n')).parse_next(input)
    }

    fn machine(input: &mut &str) -> PResult<Machine> {
        let button_a = terminated(button, literal('\n')).parse_next(input)?;
        let button_b = terminated(button, literal('\n')).parse_next(input)?;
        let prize = terminated(prize, literal('\n')).parse_next(input)?;
        Ok(Machine { button_a, button_b, prize })
    }

    fn button(input: &mut &str) -> PResult<Button> {
        literal("Button ").parse_next(input)?;
        one_of(('A', 'B')).parse_next(input)?;
        literal(": ").parse_next(input)?;
        separated_pair(offset, literal(", "), offset)
            .map(|(x,y)| Button(x,y))
            .parse_next(input)
    }

    fn offset(input: &mut &str) -> PResult<u32> {
        one_of(('X', 'Y')).parse_next(input)?;
        literal('+').parse_next(input)?;
        parse_u32.parse_next(input)
    }

    fn prize(input: &mut &str) -> PResult<Prize> {
        literal("Prize: ").parse_next(input)?;
        separated_pair(target, literal(", "), target)
            .map(|(x,y)| Prize(x,y))
            .parse_next(input)
    }

    fn target(input: &mut &str) -> PResult<u32> {
        one_of(('X', 'Y')).parse_next(input)?;
        literal('=').parse_next(input)?;
        parse_u32.parse_next(input)
    }

    fn parse_u32(input: &mut &str) -> PResult<u32> {
        digit1.try_map(|s: &str| s.parse()).parse_next(input)
    }
}

