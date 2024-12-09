use std::{fmt::Display, fs::File, io::{self, Read}, mem::swap};

use winnow::{ascii::dec_uint, combinator::{repeat, terminated}, token::{literal, one_of}, PResult, Parser};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
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

    fn get_space(&mut self) -> &mut usize {
        match self {
            Block::Free(l) => l,
            Block::File(_, l) => l,
        }
    }

    /// Modifies this block to be of at most size threshold. If there is more data, it is returned as a separate block.
    fn split_over(&mut self, threshold: usize) -> Option<Block> {
        if threshold >= self.len() {
            None
        } else {
            let total = self.len();
            let mut clone = self.clone();
            *clone.get_space() = self.len() - threshold;
            *self.get_space() = threshold;
            debug_assert_eq!(total, self.len() + clone.len());
            Some(clone)
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

    /// Compacts the disk similarly to the algorithm described in the problem description. This implementation moves entire blocks at once, rather than one piece at a time.
    fn compact(&mut self) {
        let mut i: usize = 0;
        while i < self.0.len() {
            match self.0[i] {
                Block::Free(free_space) => {
                    if let Some(file_index) = Self::last_file_lte(&self.0[i+1..], free_space) {
                        let file_index = file_index + i + 1;
                        debug_assert!(file_index > i && file_index < self.0.len());
                        let file_size = self.0[file_index].len();

                        let leftover = self.0[i].split_over(file_size);
                        self.0.swap(i, file_index);
                        if let Some(leftover) = leftover {
                            debug_assert!(leftover.len() > 0);
                            self.0.insert(i + 1, leftover);
                        } else {
                            // Because the file was chosen to be at most the size of the free space
                            // a lack of leftover in the free space means they must be exactly equal.
                            debug_assert_eq!(file_size, free_space);
                        }
                    }
                },
                Block::File(_, _) => (),
            }
            i += 1;
        }
        self.consolidate();
    }

    /// Merge adjancent blocks of the same type. This is not needed to solve the problem.
    ///
    /// Note that adjancent File blocks with the same ID should not occur from the above algorithms. These could only occur through
    /// alternate instantiation of the data structure.
    fn consolidate(&mut self) {
        let mut i = 0;
        while i < self.0.len() - 1 {
            match (&self.0[i], &self.0[i+1]) {
                (Block::Free(_), Block::Free(_)) => {
                    *self.0[i].get_space() += self.0[i+1].len();
                    self.0.remove(i+1);
                },
                (Block::File(a, _), Block::File(b, _)) if a == b => {
                    *self.0[i].get_space() += self.0[i+1].len();
                    self.0.remove(i+1);
                },
                _ => i += 1,
            }
        }
    }

    /// Finds the index of the last file with size less than or equal to the specified threshold.
    fn last_file_lte(blocks: &[Block], threshold: usize) -> Option<usize> {
        blocks.iter().enumerate().rev().find(|(_, block)| {
            if let Block::File(_,size) = block { *size <= threshold } else { false }
        })
        .map(|(i,_)| i)
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
        }).max().unwrap();
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
        }).max().unwrap() as usize;
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
    // let filename = "example2.txt";
    let filename = "input.txt";
    let digits = read_input(filename)?;
    // println!("{digits:?}");
    let mut disk = Disk::parse(&digits);
    // println!("{disk}");
    // println!("disk: {disk:?}");
    disk.compact();
    // println!("disk: {disk}");
    // println!("disk: {disk:?}");
    println!("checksum: {}", disk.checksum());

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

        #[test]
        fn check_compact(mut disk: Disk) {
            // No real test here, just check that it doesn't crash
            disk.compact();
            disk.checksum();
        }
    }
}
