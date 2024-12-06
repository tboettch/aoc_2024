use std::{fmt::Display, fs::File, io::{ self, prelude::*, BufReader}};

fn main() -> io::Result<()> {
    // let filename = "example.txt";
    let filename = "input.txt";
    let board = read_data(filename)?;
    // assert_eq!(140, board.width);
    assert_eq!(board.width * board.height, board.data.len());
    let count = count_word(&board, b"XMAS", false);
    println!("count={count}");
    Ok(())
}

type Token = u8;
type Dimension = usize;

#[derive(Debug, Clone)]
struct Board {
    data: Vec<Token>,
    width: Dimension,
    height: Dimension,
}

impl Board {
    fn get(&self, index: Idx) -> Option<&Token> {
        let Idx(x, y) = index;
        if x >= self.width || y >= self.height {
            return None;
        }
        self.data.get(x + y * self.width)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.data.chunks(self.width) {
            let s = core::str::from_utf8(line).map_err(|_| std::fmt::Error)?;
            writeln!(f, "{s}")?;
        }
        Ok(())
    }
}

#[derive(PartialOrd, PartialEq, Clone, Copy, Debug)]
struct Idx(Dimension, Dimension);

impl Idx {
    fn add(&self, offset: &(isize, isize)) -> Option<Self> {
       Some(Idx(self.0.checked_add_signed(offset.0)?, self.1.checked_add_signed(offset.1)?))
    }
}

fn count_word(board: &Board, target: &[Token], debug: bool) -> u32 {
    let directions = init_directions();
    let mut count = 0;
    for y in 0..board.height {
        for x in 0..board.width {
            let index = Idx(x,y);
            for direction in directions.iter() {
                if check_direction(board, target, index, direction).is_some() {
                    count += 1;
                    if debug {println!("index={index:?}, direction={direction:?}");}
                }
            }
        }
    }
    count
}

fn init_directions() -> Vec<(isize, isize)> {
    let mut directions: Vec<(isize, isize)> = Vec::new();
    for x in [-1,0,1] {
        for y in [-1,0,1] {
            if x != 0 || y != 0 {
                directions.push((x,y));
            }
        }
    }
    assert_eq!(8, directions.len());
    directions
}

fn check_direction(board: &Board, target: &[Token], start: Idx, direction: &(isize, isize)) -> Option<()> {
    let mut pos: Option<Idx> = Some(start);
    for c in target {
        if *board.get(pos?)? != *c {
            return None;
        }
        pos = pos.unwrap().add(direction);
    }
    Some(())
}

fn read_data(filename: &str) -> io::Result<Board> {
    let reader = BufReader::new(File::open(filename)?);
    let mut width = None;
    let mut height = 0;
    let mut data = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let bytes = line.as_bytes();
        data.extend(bytes);
        if let Some(width) = width {
            assert_eq!(width, bytes.len());
        } else {
            width = Some(bytes.len());
        }
        height += 1;
    }
    match width {
        None => Err(io::Error::new(io::ErrorKind::Other, "Empty board")),
        Some(width) => Ok(Board { data, width, height }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::{prelude::*};

    fn ascii_string(length: usize) -> impl Strategy<Value = String> {
        proptest::string::string_regex(&format!("[a-zA-Z0-9]{{{}}}", length)).unwrap()
    }

    impl Arbitrary for Board {
        type Parameters = ();
        type Strategy = BoxedStrategy<Board>;
        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            (1..100usize, 1..100usize).prop_flat_map(|(width, height)| {
                ascii_string(width * height).prop_map(move|data: String| {
                    Board {data: data.bytes().collect(), width, height}
                })
            }).boxed()
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10000))]

        #[test]
        fn check_indexing(board: Board) {
            use std::fmt::Write;
            let display = board.to_string();
            let mut manual = String::new();
            for y in 0..board.height {
                for x in 0..board.width {
                    write!(manual, "{}", *board.get(Idx(x,y)).unwrap() as char).unwrap();
                }
                write!(manual, "\n").unwrap();
            }
            assert_eq!(display, manual);
        }

        #[test]
        fn check_count(board: Board, target in ascii_string(3)) {
            let forward_count = count_word(&board, target.as_bytes(), false);
            let mut backwards = Vec::new();
            backwards.extend(target.as_bytes().iter().rev());
            let backward_count = count_word(&board, &backwards, false);
            assert_eq!(forward_count, backward_count);
        }
    }

    #[test]
    fn basic_board() {
        let word = b"XMAS";
        for i in 1..100 {
            println!("i={i}");
            let data = word.repeat(i);
            let board = Board { data, width: word.len(), height: i };
            println!("{board}");
            let count = count_word(&board, b"XMAS", true);
            assert_eq!(count, (i + 2 * i.saturating_sub(3)) as u32);
        }
    }

    #[test]
    fn double_board() {
        let word = b"XMASAMX";
        for i in 1..100 {
            println!("i={i}");
            let data = word.repeat(i);
            let board = Board { data, width: word.len(), height: i };
            println!("{board}");
            let count = count_word(&board, b"XMAS", true);
            assert_eq!(count, 2 * (i + 2 * i.saturating_sub(3)) as u32);
        }
    }

    #[test]
    fn double_board2() {
        let word = b"XMASSAMX";
        for i in 1..100 {
            println!("i={i}");
            let data = word.repeat(i);
            let board = Board { data, width: word.len(), height: i };
            println!("{board}");
            let count = count_word(&board, b"XMAS", true);
            assert_eq!(count, 2 * (i + 2 * i.saturating_sub(3)) as u32);
        }
    }
}
