use std::{fs::File, io::{self, prelude::*}};

fn main() -> io::Result<()> {
    // let filename = "example.txt";
    // let filename = "example2.txt";
    let filename = "input.txt";
    let mut data = read_data(filename)?;
    // println!("{data:?}");
    blink_n(&mut data, 25);
    println!("stone count: {}", data.len());
    Ok(())
}

fn blink_n(data: &mut Vec<u64>, count: u32) {
    for i in 0..count {
        blink(data);
        // println!("{data:?}"); // FIXME: Remove
    }
}

fn blink(data: &mut Vec<u64>) {
    let mut i = 0;
    while i < data.len() {
        match data[i] {
            0 => data[i] = 1,
            x if digits(x) % 2 == 0 => {
                let (j, k) = split_stone(x);
                data.insert(i, j);
                data[i+1] = k;
                i += 1;
            },
            x => data[i] = x * 2024,
        }
        i += 1;
    }
}

fn split_stone(x: u64) -> (u64, u64) {
    let divisor = 10u64.pow(digits(x) / 2);
    (x / divisor, x % divisor)
}

fn digits(x: u64) -> u32 {
    (x as f64).log10().trunc() as u32 + 1
}

fn read_data(filename: &str) -> io::Result<Vec<u64>> {
    let mut buf = String::new();
    File::open(filename)?.read_to_string(&mut buf)?;
    buf.split_ascii_whitespace().map(|s| s.parse::<u64>().map_err(io::Error::other)).collect::<io::Result<Vec<u64>>>()
}
