use std::{collections::{HashMap, HashSet}, fmt::Display, fs::File, io::{self, BufRead, BufReader}};

use enumset::{EnumSet, EnumSetType};
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

    fn count_sides(&self) -> usize {
        let adjacencies: HashMap<&Position, EnumSet<Direction>> = self.members.iter()
            .map(|pos| {
                let mut set: EnumSet<Direction> = EnumSet::new();
                for dir in Direction::ALL {
                    if let Some(adj) = pos + &dir.to_offset() {
                        if self.members.contains(&adj) {
                            set.insert(dir);
                        }
                    }
                }
                (pos, set)
            })
            .collect();

        // A corner is formed when an adjacent tile in a perpendicular direction lacks the same edge as this tile (including tiles outside this region).
        // An edge is a direction pointing outside this region (including the edge of the map). This is the complement of the adjacency set.
        // If multiple such tiles exist, then multiple corners are formed.
        // The number of corners is equal to the number of sides.
        // This algorithm double-counts corners, so we divide by two at the end.
        let mut corners: usize = 0;
        for (pos, adj) in adjacencies.iter() {
            let non_adj = adj.complement();
            for edge in non_adj.iter() {
                for dir in edge.perpendicular() {
                    if let Some(adj_pos) = *pos + &dir.to_offset() {
                        if let Some(other_adj) = adjacencies.get(&adj_pos) {
                            if other_adj.complement().contains(edge) {
                                continue;
                            }
                        }
                    }
                    corners += 1;
                }
            }
        }
        corners / 2
    }

    fn area(&self) -> usize {
        self.members.len()
    }

    fn price_by_perimeter(&self) -> usize {
        self.perimeter() * self.area()
    }

    fn price_by_sides(&self) -> usize {
        // self.perimeter() * self.area()
        self.count_sides() * self.area()
    }
}

#[derive(Debug, EnumSetType)]
enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    const ALL: [Direction; 4] = [Direction::Up, Direction:: Down, Direction::Left, Direction::Right];

    fn to_offset(&self) -> Offset {
        match self {
            Direction::Up => Offset::new(0, -1),
            Direction::Down => Offset::new(0, 1),
            Direction::Left => Offset::new(-1, 0),
            Direction::Right => Offset::new(1, 0),
        }
    }

    fn perpendicular(&self) -> [Direction; 2] {
        match self {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
        }
    }
}

fn main() -> io::Result<()> {
    // let filename = "example.txt";
    // let filename = "example2.txt";
    // let filename = "example3.txt";
    let filename = "input.txt";
    let board = read_data(filename)?;
    let regions = find_regions(&board);
    let perimeter_price: usize = regions.iter().map(Region::price_by_perimeter).sum();
    println!("perimeter price: {perimeter_price}");
    let sides_price: usize = regions.iter().map(Region::price_by_sides).sum();
    println!("sides price: {sides_price}");
    assert!(perimeter_price >= sides_price);
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
    Direction::ALL.iter()
        .filter_map(|d| pos + &d.to_offset())
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
