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

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
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

    /// Shrinks this vector down to the smallest integer-valued vector in the same direction
    fn shrink(&self) -> Self {
        let d = gcd(self.0, self.1);
        Self(self.0 / d, self.1 / d)
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
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
    antenna_positions: HashMap<Token, Vec<Position>>, // TODO: Tree map might be better here
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
            if y < self.height - 1 { writeln!(f, "")?; }
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
        // width + 1 accounts for the newlines added in the Display implementation. -1 reflects the lack of trailing newline on the final line.
        // This might fail on a platform with different line endings (e.g. Windows)
        assert_eq!(bytes.len(), (self.width + 1) * self.height - 1);
        for pos in antinodes {
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
    // let filename = "example2.txt";
    let filename = "input.txt";
    let board = read_data(filename)?;
    // println!("{board}");
    let annotated_board = annotate_board(board);
    // println!("antennas: {:?}", annotated_board.antenna_positions);
    let antinodes = compute_antinodes(&annotated_board);
    println!("antinodes count: {:?}", antinodes.len());
    println!("{}", annotated_board.board.render_with_antinodes(antinodes.iter()));
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

fn compute_antinodes(annotated_board: &AnnotatedBoard) -> HashSet<Position> {
    let mut result = HashSet::new();
    let board = &annotated_board.board;
    
    for (_, positions) in annotated_board.antenna_positions.iter() {
        for i in 0..(positions.len() - 1) {
            for j in (i+1)..positions.len() {
                let pos1 = &positions[i];
                let pos2 = &positions[j];
                let unit = pos1.diff(pos2).shrink();
                assert!(!unit.is_zero());
                for unit in [&unit, &unit.reverse()] {
                    for x in 0.. {
                        let off = unit.mul(x);
                        if let Some(pos) = pos1.add(&off) {
                            if board.within_bounds(&pos) {
                                // println!("pos1={pos1} pos2={pos2} unit={unit} off={off} antinode={pos}");
                                result.insert(pos);
                                // println!("{}", board.render_with_antinodes(result.iter()));
                                // println!("---------------------------------------------------");
                                continue;
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
    result
}

fn gcd(x: isize, y: isize) -> isize {
    _gcd(x.abs(), y.abs())
}

fn _gcd(x: isize, y: isize) -> isize {
    if x == 0 { return y; }
    _gcd(y % x, x)
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
                b'#' => panic!("Symbol '#' is reserved an cannot appear in input"),
                c    => data.push(Square::Antenna(c)),
            }
        }
    }
    Ok(Board { data, width: width.expect("non-empty board"), height })
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10000))]

        #[test]
        fn check_gcd(x: isize, y: isize) {
            let d = gcd(x,y);
            assert!(d > 0 || x == 0 && y == 0);
            assert_eq!(0, x % d);
            assert_eq!(0, y % d);
        }
    }

}
