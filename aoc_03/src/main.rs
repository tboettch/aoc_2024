use std::{
    fs::File,
    num::ParseIntError,
    io::{self, prelude::*},
};

use winnow::{ascii::digit1, combinator::{delimited, separated_pair}, prelude::*};

fn main() -> io::Result<()> {
    let filename = "input.txt";
    let mut data = String::new();
    {
        File::open(filename)?.read_to_string(&mut data)?;
    }
    let instructions = parse_instructions(&data);
    let sum: u64 = instructions.iter().map(|instr| {
        match instr {
            Instruction::Mul(x,y) => (x * y) as u64,
        }
    })
    .sum();
    println!("sum={sum}");
    Ok(())
}

enum Instruction {
    Mul(u32, u32),
}


fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut input = input;
    let mut result = Vec::new();
    while !input.is_empty() {
        match instruction.parse_next(&mut input) {
            Ok(instr) => result.push(instr),
            Err(_) => input = &input[1..],
        }
    }
    result
}

fn instruction(input: &mut &str) -> PResult<Instruction> {
    delimited(
        "mul(",
        separated_pair(digit1, ',', digit1),
        ")"
    )
    .try_map(|(x, y): (&str, &str)| {
        Ok::<_, ParseIntError>(Instruction::Mul(x.parse()?, y.parse()?))
    })
    .parse_next(input)
}
