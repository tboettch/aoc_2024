use std::fmt::Display;
use std::fs::File;
use std::ops::Index;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up, Right, Down, Left
}

impl Direction {
    fn turn(&self) -> Self {
        use Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up
        }
    }
    
    fn offset(&self) -> Offset {
        use Direction::*;
        match self {
            Up => Offset(0, -1),
            Right => Offset(1, 0),
            Down => Offset(0, 1),
            Left => Offset(-1, 0),
        }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
struct Position(usize, usize);

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
struct Offset(isize, isize);

impl Position {
    fn apply_offset(&self, off: Offset) -> Option<Self> {
        Some(Position(self.0.checked_add_signed(off.0)?, self.1.checked_add_signed(off.1)?))
    }
}

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

impl Index<&Position> for Board {
    type Output = Square;
    fn index(&self, index: &Position) -> &Self::Output {
        &self.data[self.raw_index(&index)]
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
                    match self[&pos] {
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
    fn at(&self, index: &Position) -> Option<&Square> {
        if index.0 >= self.width || index.1 >= self.height {
            return None;
        }
        Some(&self[index])
    }

    fn raw_index(&self, index: &Position) -> usize {
        index.0 + self.width * index.1
    }

    fn render_visited(&self, visited: &Vec<bool>) -> String {
        use std::fmt::Write;
        let mut r = String::with_capacity(self.data.len());
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position(x,y);
                match self[&pos] {
                    Square::Empty => {
                        if visited[self.raw_index(&pos)] {
                            write!(r, "X").unwrap();
                        } else {
                            write!(r, ".").unwrap();
                        }
                    },
                    Square::Obstacle => write!(r, "#").unwrap(),
                }
            }
            writeln!(r, "").unwrap();
        }
        r
    }
}

fn main() -> io::Result<()> {
    // let filename = "example.txt";
    let filename = "input.txt";
    let board = read_data(filename)?;
    // println!("{board}");
    println!("count: {}", count_walk_board(&board));
    Ok(())
}

fn count_walk_board(board: &Board) -> u32 {
    let visited = walk_board(board);
    // println!("{board}");
    // println!("----------------");
    // println!("{}", board.render_visited(&visited));
    visited.iter().filter(|x| **x).count() as u32
}

fn walk_board(board: &Board) -> Vec<bool> {
    let mut visited = vec![false; board.data.len()];
    let mut pos = board.guard_init.pos.clone();
    let mut dir = board.guard_init.dir;

    loop {
        visited[board.raw_index(&pos)] = true;
        let next_pos = pos.apply_offset(dir.offset());
        if next_pos.is_none() {
            break;
        }
        let next_pos = next_pos.unwrap();
        match board.at(&next_pos) {
            Some(Square::Empty) => pos = next_pos,
            Some(Square::Obstacle) => dir = dir.turn(),
            None => break,
        }
    }

    visited
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
