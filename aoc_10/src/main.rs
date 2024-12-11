use std::{collections::VecDeque, fmt::Display, fs::File, io::{self, BufRead, BufReader}};
use bit_set::BitSet;
use grid::{Grid, Offset, Position};

type Token = u8;

struct Board {
    grid: Grid<Token>,
}

impl Board {
    fn find_digit(&self, digit: Token) -> Vec<Position> {
        let mut r = Vec::new();
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                let pos = Position::new(x,y);
                if self.grid[&pos] == digit {
                    r.push(pos);
                }
            }
        }
        r
    }

    fn count_trailhead_scores(&self) -> usize {
        let directions = [Offset::new(0,1), Offset::new(0,-1), Offset::new(1,0), Offset::new(-1,0)];
        let summits = self.find_digit(9);
        let mut visited: Grid<BitSet> = self.grid.map(|_| BitSet::with_capacity(summits.len()));

        let mut queue: VecDeque<(usize, Position)> = VecDeque::from_iter(summits.iter().enumerate().map(|(i, pos)| (i, pos.clone())));
        while let Some((i, pos)) = queue.pop_front() {
            if visited[&pos].insert(i) {
                let elevation = self.grid[&pos];
                for diff in directions.iter() {
                    if let Some(next_pos) = &pos + &diff {
                        if self.grid.in_bounds(&next_pos) && self.grid[&next_pos] + 1 == elevation {
                            queue.push_back((i, next_pos));
                        }
                    }
                }
            }
        }

        let trailheads = self.find_digit(0);
        trailheads.iter().map(|pos| visited[&pos].len()).sum()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid)
    }
}

fn main() -> io::Result<()> {
    // let filename = "example.txt";
    let filename = "input.txt";
    let board = read_data(filename)?;
    // println!("{board}");
    println!("trailheads sum: {}", board.count_trailhead_scores());

    Ok(())
}

fn read_data(filename: &str) -> io::Result<Board> {
    let mut data: Vec<Token> = Vec::new();
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
        for c in line.chars() {
            data.push(c.to_string().parse().map_err(io::Error::other)?);
        }
    }
    Ok(Board{ grid: Grid::new(data, width.ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Expected non-empty board"))?, height) })
}
