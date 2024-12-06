use std::{
    fs::File,
    num::ParseIntError,
    io::{self, prelude::*},
};

use winnow::{ascii::digit1, combinator::{alt, delimited, separated_pair}, prelude::*, token::literal};

fn main() -> io::Result<()> {
    let filename = "input.txt";
    let mut data = String::new();
    {
        File::open(filename)?.read_to_string(&mut data)?;
    }
    let instructions = parse_instructions(&data);
    let sum: u64 = sum_unconditional(&instructions);
    println!("sum={sum}");
    let sum_conditional = sum_conditional(&instructions);
    println!("sum_conditional={sum_conditional}");
    Ok(())
}

enum Instruction {
    Mul(u32, u32),
    Do(),
    Dont(),
}

fn sum_unconditional(instructions: &[Instruction]) -> u64 {
    instructions.iter().map(|instr| {
        match instr {
            Instruction::Mul(x,y) => (x * y) as u64,
            _ => 0,
        }
    })
    .sum()
}

fn sum_conditional(instructions: &[Instruction]) -> u64 {
    let mut enabled = true;
    let mut sum: u64 = 0;
    for instr in instructions {
        match instr {
            Instruction::Do() => enabled = true,
            Instruction::Dont() => enabled = false,
            Instruction::Mul(x, y) => if enabled { sum += (x * y) as u64 },
        }
    }
    sum
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
    alt((
        delimited(
            "mul(",
            separated_pair(digit1, ',', digit1),
            ")"
        )
        .try_map(|(x, y): (&str, &str)| {
            Ok::<_, ParseIntError>(Instruction::Mul(x.parse()?, y.parse()?))
        }),
        literal("do()").map(|_| Instruction::Do()),
        literal("don't()").map(|_| Instruction::Dont())
    ))
    .parse_next(input)
}
