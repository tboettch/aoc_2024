use std::{fmt::Display, fs::File, io::{self, Read}};

use winnow::{ascii::dec_uint, combinator::{repeat, terminated}, token::{literal, one_of}, PResult, Parser};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Block {
    Free(usize),
    File(u32, usize),
}

impl Block {
    fn len(&self) -> usize {
        match self {
            Block::Free(l) => *l,
            Block::File(_, l) => *l,
        }
    }
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

    fn checksum(&self) -> usize {
        let mut i = 0;
        let mut sum = 0;
        for block in self.0.iter() {
            let len = block.len();
            match block {
                Block::Free(_) => (),
                Block::File(id, _) => {
                    // TODO: Closed form
                    for x in i..i+len {
                        sum += (*id as usize) * x;
                    }
                },
            }
            i += len;
        }
        sum
    }

    #[cfg(test)]
    fn string_checksum(&self) -> usize {
        let width = self.0.iter().filter_map(|block| {
            match block {
                Block::Free(_) => None,
                Block::File(c, _) => Some(digits(*c)),
            }
        }).last().unwrap();
        assert!(width < 2, "string_checksum only works for 1-digit IDs");
        self.to_string().chars().enumerate().map(|(i,c)| {
            match c {
                '.' => 0,
                _ => c.to_string().parse::<usize>().unwrap() * i,
            }
        }).sum()
    }
}

fn digits(x: u32) -> u32 {
    ((x as f64).log10() as u32) + 1
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() { return Ok(()); }
        let width = self.0.iter().filter_map(|block| {
            match block {
                Block::Free(_) => None,
                Block::File(c, _) => Some(digits(*c)),
            }
        }).last().unwrap() as usize;
        for block in self.0.iter() {
            let len = block.len();
            let width = width * len;
            match block {
                Block::Free(_) => write!(f, "{:width$}", ".".repeat(len))?,
                Block::File(id, _) => write!(f, "{:width$}", id.to_string().repeat(len))?,
            }
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    // let filename = "example.txt";
    let filename = "example2.txt";
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

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    impl Arbitrary for Disk {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;
        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            "[0-9]+\n".prop_map(|s| {
                let digits = parse_digits.parse(&s).unwrap();
                Disk::parse(&digits)
            }).boxed()
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10000))]

        /// Max length = 10 * 2 because the string version only works with 1 digit IDs
        #[test]
        fn check_checksum(s in "[0-9]{1,20}\n") {
            let digits = parse_digits.parse(&s).unwrap();
            let disk = Disk::parse(&digits);
            assert_eq!(disk.checksum(), disk.string_checksum());
        }
    }
}
