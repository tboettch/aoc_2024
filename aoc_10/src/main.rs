use std::{fmt::Display, fs::File, io::{self, BufRead, BufReader}};

use grid::Grid;

type Token = u8;

struct Board {
    grid: Grid<Token>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid)
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
