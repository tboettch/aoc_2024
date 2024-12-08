use std::{
    collections::HashMap, fmt::Display, fs::File, io::{self, prelude::*, BufReader}, ops::{Index, IndexMut}
};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Hash)]
struct Position(usize, usize);

type Token = u8;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum Square {
    Empty,
    Antenna(Token),
}

#[derive(Debug,  Clone)]
struct Board {
    data: Vec<Square>,
    width: usize,
    height: usize,
}

impl Index<&Position> for Board {
    type Output = Square;
    fn index(&self, index: &Position) -> &Self::Output {
        &self.data[self.raw_index(&index)]
    }
}

impl IndexMut<&Position> for Board {
    fn index_mut(&mut self, index: &Position) -> &mut Self::Output {
        let i = self.raw_index(&index);
        &mut self.data[i]
    }
}

struct AnnotatedBoard {
    board: Board,
    antenna_positions: HashMap<Token, Vec<Position>>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position(x,y);
                match self[&pos] {
                    Square::Empty => write!(f, ".")?,
                    Square::Antenna(c) => write!(f, "{}", c as char)?,
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Board {
    fn at(&self, index: &Position) -> Option<&Square> {
        if index.0 >= self.width || index.1 >= self.height {
            return None;
        }
        Some(&self[index])
    }

    fn raw_index(&self, index: &Position) -> usize {
        index.0 + self.width * index.1
    }

    fn position_from_index(&self, index: usize) -> Position {
        let y = index / self.width;
        if y >= self.height {
            panic!("Out-of-bounds index: {index}");
        }
        let x = index % self.width;
        Position(x,y)
    }
}

fn main() -> io::Result<()> {
    let filename = "example.txt";
    // let filename = "input.txt";
    let board = read_data(filename)?;
    println!("{board}");
    let annotated_board = annotate_board(board);
    println!("antennas: {:?}", annotated_board.antenna_positions);

    Ok(())
}

fn annotate_board(board: Board) -> AnnotatedBoard {
    let mut map: HashMap<u8, Vec<Position>> = HashMap::new();
    for (i, square) in board.data.iter().enumerate() {
        if let Square::Antenna(c) = square {
            let pos = board.position_from_index(i);
            map.entry(*c).or_default().push(pos);
        }
    }
    AnnotatedBoard { board, antenna_positions: map }
}

fn read_data(filename: &str) -> io::Result<Board> {
    let mut data = Vec::new();
    let mut width: Option<usize> = None;
    let mut height = 0;
    for line in BufReader::new(File::open(filename)?).lines() {
        let line = line?;
        assert!(line.is_ascii());
        if let Some(x) = width {
            assert_eq!(x, line.len());
        } else {
            width = Some(line.len());
        }
        height += 1;
        for c in line.bytes() {
            match c {
                b'.' => data.push(Square::Empty),
                c    => data.push(Square::Antenna(c)),
            }
        }
    }
    Ok(Board { data, width: width.expect("non-empty board"), height })
}
