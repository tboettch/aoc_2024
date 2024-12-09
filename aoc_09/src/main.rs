use std::{fmt::Display, fs::File, io::{self, Read}};

use winnow::{ascii::dec_uint, combinator::{repeat, terminated}, token::{literal, one_of}, PResult, Parser};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Block {
    Free(usize),
    File(u8, usize),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Disk(Vec<Block>);

impl Disk {
    fn parse(data: &Vec<u8>) -> Self {
        let mut is_file = true;
        let mut r = Vec::with_capacity(data.len());
        let mut i = 0;
        for d in data {
            if is_file {
                r.push(Block::File(i, *d as usize));
                i += 1;
            } else {
                r.push(Block::Free(*d as usize));
            }
            is_file = !is_file;
        }
        Disk(r)
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in self.0.iter() {
            match block {
                Block::Free(len) => write!(f, "{}", ".".repeat(*len))?,
                Block::File(id, len) => write!(f, "{}", id.to_string().repeat(*len))?,
            }
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let filename = "example.txt";
    // let filename = "input.txt";
    let digits = read_input(filename)?;
    println!("{digits:?}");
    let disk = Disk::parse(&digits);
    println!("{disk}");
    Ok(())
}

fn read_input(filename: &str) -> io::Result<Vec<u8>> {
    let mut buf = String::new();
    File::open(filename)?.read_to_string(&mut buf)?;
    parse_digits.parse(&buf).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, e.to_string())
    })
}

fn parse_digits(input: &mut &str) -> PResult<Vec<u8>> {
    terminated(
        repeat(1.., one_of('0'..='9').try_map(|s: char| s.to_string().parse::<u8>())),
        literal('\n')
    ).parse_next(input)
}
