use std::fmt::Display;
use std::fs::File;
use std::ops::Index;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up, Right, Down, Left
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
struct Position(usize, usize);

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum Square {
    Empty,
    Obstacle
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
struct Guard {
    pos: Position,
    dir: Direction,
}

#[derive(Debug,  Clone)]
struct Board {
    data: Vec<Square>,
    width: usize,
    height: usize,
    guard_init: Guard,
}

impl Index<Position> for Board {
    type Output = Square;
    fn index(&self, index: Position) -> &Self::Output {
        &self.data[index.0 + self.width * index.1]
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assert_eq!(self.guard_init.dir, Direction::Up);
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position(x,y);
                if pos == self.guard_init.pos {
                    write!(f, "^")?;
                } else {
                    match self[pos] {
                        Square::Empty => write!(f, ".")?,
                        Square::Obstacle => write!(f, "#")?,
                    }
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Board {
    fn at(&self, index: Position) -> Option<&Square> {
        if index.0 >= self.width || index.1 >= self.height {
            return None;
        }
        Some(&self[index])
    }
}

fn main() -> io::Result<()> {
    let filename = "example.txt";
    // let filename = "input.txt";
    let board = read_data(filename)?;
    println!("{board}");
    Ok(())
}

fn read_data(filename: &str) -> io::Result<Board> {
    let mut data = Vec::new();
    let mut width: Option<usize> = None;
    let mut height = 0;
    let mut guard_init: Option<Guard> = None;
    let mut pos = Position(0,0);
    for line in BufReader::new(File::open(filename)?).lines() {
        pos.0 = 0;
        pos.1 = height;
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
                b'#' => data.push(Square::Obstacle),
                // Could parse other orientations here, but it doesn't seem necessary
                b'^' => {
                    data.push(Square::Empty);
                    if guard_init.is_some() {
                        panic!("Multiple guard positions specified in board");
                    } else {
                        guard_init = Some(Guard { pos: pos.clone(), dir: Direction::Up });
                    }

                },
                x => panic!("Unexpected map character: {x}"),
            }
            pos.0 += 1;
        }
    }
    Ok(Board { data, width: width.expect("non-empty board"), height, guard_init: guard_init.expect("to find a guard position in the map") })
}
