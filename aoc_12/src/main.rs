use std::{collections::HashSet, fmt::Display, fs::File, io::{self, BufRead, BufReader}};

use grid::{Grid, Offset, Position};

type Token = u8;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Board {
    grid: Grid<Token>
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid.map(|b| *b as char))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Region {
    members: HashSet<Position>,
}

impl Region {
    fn new() -> Self {
        Region { members: HashSet::new() }
    }

    fn perimeter(&self) -> usize {
        let mut perimeter = 0;
        for pos in self.members.iter() {
            let adjacencies = adjacent(pos).iter()
                .filter(|adj| self.members.contains(adj))
                .count();
            perimeter += 4 - adjacencies;
        }
        perimeter
    }

    fn area(&self) -> usize {
        self.members.len()
    }

    fn price(&self) -> usize {
        self.perimeter() * self.area()
    }
}

fn main() -> io::Result<()> {
    // let filename = "example.txt";
    // let filename = "example2.txt";
    let filename = "input.txt";
    let board = read_data(filename)?;
    // println!("{board}");
    let regions = find_regions(&board);
    // println!("{regions:?}");
    let total_price: usize = regions.iter().map(Region::price).sum();
    println!("total price: {total_price}");
    Ok(())
}

fn find_regions(board: &Board) -> Vec<Region> {
    let mut r = Vec::new();
    let mut visited = board.grid.map(|_| false);
    for y in 0..board.grid.height() {
        for x in 0..board.grid.width() {
            let pos = Position::new(x,y);
            if !visited[&pos] {
                let mut region = Region::new();
                fill_region(board, &mut region, &mut visited, &pos);
                r.push(region);
            }
        }
    }
    r
}

fn fill_region(board: &Board, region: &mut Region, visited: &mut Grid<bool>, pos: &Position) {
    if visited[&pos] {
        return;
    }
    region.members.insert(pos.clone());
    visited[&pos] = true;
    for next_pos in adjacent(pos) {
        if board.grid.get(&next_pos) == Some(&board.grid[pos]) {
            fill_region(board, region, visited, &next_pos);
        }
    }
}

fn adjacent(pos: &Position) -> Vec<Position> {
    [Offset::new(-1, 0), Offset::new(1, 0), Offset::new(0, 1), Offset::new(0, -1)].iter()
        .filter_map(|p| pos + &p)
        .collect()
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
        data.extend(line.bytes());
    }
    Ok(Board{ grid: Grid::new(data, width.ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Expected non-empty board"))?, height) })
}
