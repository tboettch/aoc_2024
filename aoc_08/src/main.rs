use std::{
    collections::{HashMap, HashSet}, fmt::Display, fs::File, io::{self, prelude::*, BufReader}, ops::{Add, Index, IndexMut, Sub}
};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Hash)]
struct Position(usize, usize);

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Hash)]
struct Offset(isize, isize);

fn dist(x: usize, y: usize) -> usize {
    if x > y { x - y } else { y - x }
}

impl Position {
    fn dist(&self, rhs: &Self) -> usize {
        dist(self.0, rhs.0) + dist(self.1, rhs.1)
    }

    fn diff(&self, rhs: &Self) -> Offset {
        Offset(self.0 as isize - rhs.0 as isize, self.1 as isize - rhs.1 as isize)
    }

    fn add(&self, offset: &Offset) -> Option<Self> {
        Some(Self(self.0.checked_add_signed(offset.0)?, self.1.checked_add_signed(offset.1)?))
    }
}

impl Offset {
    fn mul(&self, scale: isize) -> Self {
        Self(self.0 * scale, self.1 * scale)
    }

    fn reverse(&self) -> Self {
        self.mul(-1)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }
}

type Token = u8;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum Square {
    Empty,
    Antenna(Token),
}

#[derive(Debug, Clone)]
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
    antenna_positions: HashMap<Token, HashSet<Position>>, // TODO: Tree map might be better here
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

    fn within_bounds(&self, index: &Position) -> bool {
        self.at(index).is_some()
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

    fn render_with_antinodes<'a, Iter: Iterator<Item = &'a Position>>(&self, antinodes: Iter) -> String {
        let mut bytes = self.to_string().into_bytes();
        for pos in antinodes {
            // width + 1 accounts for the newlines added in the Display implementation
            let i = pos.0 + pos.1 * (self.width + 1);
            if bytes[i] == b'.' {
                bytes[i] = b'#';
            }
        }
        String::from_utf8(bytes.to_vec()).unwrap()
    }
}

fn main() -> io::Result<()> {
    // let filename = "example.txt";
    let filename = "input.txt";
    let board = read_data(filename)?;
    // println!("{board}");
    let annotated_board = annotate_board(board);
    // println!("antennas: {:?}", annotated_board.antenna_positions);
    let antinodes = compute_antinodes(&annotated_board);
    println!("antinodes count: {:?}", antinodes.len());
    // println!("{}", annotated_board.board.render_with_antinodes(antinodes.iter()));
    Ok(())
}

fn annotate_board(board: Board) -> AnnotatedBoard {
    let mut map: HashMap<u8, HashSet<Position>> = HashMap::new();
    for (i, square) in board.data.iter().enumerate() {
        if let Square::Antenna(c) = square {
            let pos = board.position_from_index(i);
            map.entry(*c).or_default().insert(pos);
        }
    }
    AnnotatedBoard { board, antenna_positions: map }
}

fn compute_antinodes(annotated_board: &AnnotatedBoard) -> HashSet<Position> {
    let mut result = HashSet::new();
    let board = &annotated_board.board;
    
    for y in 0..board.height {
        for x in 0..board.width {
            let pos = Position(x,y);
            'antenna_loop: for (_, positions) in annotated_board.antenna_positions.iter() {
                for antenna_pos in positions {
                    let diff = antenna_pos.diff(&pos);
                    if diff.is_zero() { continue; }
                    if let Some(double_pos) = pos.add(&diff.mul(2)) {
                        if positions.contains(&double_pos) {
                            result.insert(pos.clone());
                            break 'antenna_loop;
                        }
                    }
                }
            }
        }
    }
    result
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
