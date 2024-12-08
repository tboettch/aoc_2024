use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::hash::RandomState;
use std::ops::{Index, IndexMut};
use std::io::{self, BufRead, BufReader};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up, Right, Down, Left
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Direction::Up => "Up",
            Direction::Right => "Right",
            Direction::Down => "Down",
            Direction::Left => "Left",
        };
        write!(f, "{}", s)
    }
}

impl Direction {
    const ALL: [Direction; 4] = [Direction::Up, Direction::Right, Direction::Down, Direction::Left];

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

    const fn bit_index(&self) -> u8 {
        match self {
            Direction::Up => 2u8.pow(0),
            Direction::Right => 2u8.pow(1),
            Direction::Down => 2u8.pow(2),
            Direction::Left => 2u8.pow(3),
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
struct DirSet(u8);

impl Default for DirSet {
    fn default() -> Self {
        DirSet(0)
    }
}

impl DirSet {
    fn get(&self, dir: &Direction) -> bool {
        (self.0 & dir.bit_index()) != 0
    }

    fn set(&mut self, dir: &Direction) {
        self.0 |= dir.bit_index();
    }

    fn clear(&mut self, dir: &Direction) {
        self.0 &= !dir.bit_index()
    }

    /// Sets the flag for the specified value, returning the previous value.
    fn get_and_set(&mut self, dir: &Direction) -> bool {
        let prev = self.get(dir);
        self.set(dir);
        prev
    }

    fn is_empty(&self) -> bool {
        self.0 == 0
    }

    fn to_symbol(&self) -> char {
        let horizontal = self.get(&Direction::Left) || self.get(&Direction::Right);
        let vertical = self.get(&Direction::Up) || self.get(&Direction::Down);
        if vertical && horizontal {'+'} else if vertical {'|'} else if horizontal {'-'} else {'.'}
    }
}

impl Display for DirSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for dir in Direction::ALL {
            if first { first = false } else { write!(f, ",")?; }
            write!(f, "{dir}")?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
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

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
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

impl IndexMut<&Position> for Board {
    fn index_mut(&mut self, index: &Position) -> &mut Self::Output {
        let i = self.raw_index(&index);
        &mut self.data[i]
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

    // FIXME: Needs visited data. Could choose to return that again from walk_path(...), or could build it from path data.
    fn render_visited(&self, visited: &Vec<DirSet>) -> String {
        use std::fmt::Write;
        let mut r = String::with_capacity(self.data.len());
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position(x,y);
                match self[&pos] {
                    Square::Empty => {
                        let directions = &visited[self.raw_index(&pos)];
                        write!(r, "{}", directions.to_symbol()).unwrap();
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
    // let filename = "test.txt";
    let filename = "input.txt";
    let board = read_data(filename)?;
    // println!("{board}");
    println!("count: {}", count_walk_board(&board));
    println!("potential loops: {}", count_potential_loops(&board));
    Ok(())
}

fn count_walk_board(board: &Board) -> u32 {
    let (_, path) = walk_board(board, None, None);
    let path = path.unwrap();
    // println!("{board}");
    // println!("----------------");
    // println!("{}", board.render_visited(&visited));
    let set: HashSet<Position, RandomState> = HashSet::from_iter(path.iter().map(|g| g.pos));
    set.len() as u32
}

fn count_potential_loops(board: &Board) -> u32 {
    let (is_loop, path) = walk_board(board, None, None);
    let path = path.unwrap();
    assert!(!is_loop);
    assert!(!path.is_empty());

    let mut board = board.clone();
    let mut visited = vec![DirSet::default(); board.data.len()];
    let mut count: u32 = 0;
    
    for (i, guard) in path.iter().enumerate().skip(1) {
        // Each step of the loop considers one of the positions in the guard's recorded path as the location of a potential obstacle.
        // The guard's "current" position for the purpose of walking the board is actually the previous entry in the path.
        let obstacle_pos = &guard.pos;
        let guard = path[i-1];
        if *obstacle_pos != board.guard_init.pos && visited[board.raw_index(obstacle_pos)].is_empty() {
            debug_assert!(board[obstacle_pos] == Square::Empty);
            board[obstacle_pos] = Square::Obstacle;
            let (is_loop, _) = walk_board(&board, Some(visited.clone()), Some(&guard));
            if is_loop { count += 1; }
            board[obstacle_pos] = Square::Empty;
        }
        visited[board.raw_index(&guard.pos)].set(&guard.dir);
    }
    count
}

/// Walks the board, recording position visited, including multiple directions at each position. If an initial position is provided, then no path will be returned.
/// If an initial position or visited squares data is provided, then both must be provided. 
///
/// The returned boolean is true if the path is a loop, and false if not (i.e. the guard leaves the board).
fn walk_board(board: &Board, visited: Option<Vec<DirSet>>, start: Option<&Guard>) -> (bool, Option<Vec<Guard>>) {
    assert!(visited.is_some() == start.is_some());
    let mut visited = visited.unwrap_or_else(|| vec![DirSet::default(); board.data.len()]);
    let mut guard: Guard;
    let mut path: Option<Vec<Guard>>;
    if let Some(start) = start {
        guard = start.clone();
        path = None;
    } else {
        guard = board.guard_init.clone();
        path = Some(Vec::new());
    }

    loop {
        if visited[board.raw_index(&guard.pos)].get_and_set(&guard.dir) {
            return (true, path)
        }
        let next_pos = guard.pos.apply_offset(guard.dir.offset());
        if next_pos.is_none() {
            break;
        }
        let next_pos = next_pos.unwrap();
        match board.at(&next_pos) {
            Some(Square::Empty) => guard.pos = next_pos,
            Some(Square::Obstacle) => guard.dir = guard.dir.turn(),
            None => break,
        }
        if let Some(ref mut path) = path {
            path.push(guard.clone());
        }
    }

    (false, path)
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
