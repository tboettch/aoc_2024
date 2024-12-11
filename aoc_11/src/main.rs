use std::{fs::File, io::{self, prelude::*}};

fn main() -> io::Result<()> {
    let filename = "example.txt";
    // let filename = "input.txt";
    let data = read_data(filename)?;
    println!("{data:?}");
    Ok(())
}

fn read_data(filename: &str) -> io::Result<Vec<u64>> {
    let mut buf = String::new();
    File::open(filename)?.read_to_string(&mut buf)?;
    buf.split_ascii_whitespace().map(|s| s.parse::<u64>().map_err(io::Error::other)).collect::<io::Result<Vec<u64>>>()
}
