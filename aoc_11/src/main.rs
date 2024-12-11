use std::{collections::{HashMap}, fs::File, io::{self, prelude::*}};

fn main() -> io::Result<()> {
    // let filename = "example.txt";
    // let filename = "example2.txt";
    let filename = "input.txt";
    let mut data = read_data(filename)?;
    // println!("{data:?}");
    // blink_n(&mut data, 25);
    // println!("stone count: {}", data.len());
    // println!("stone25 count: {}", count_blink_all_n(&data, 25));
    println!("stone75 count: {}", count_blink_all_n(&data, 75));
    Ok(())
}

fn blink_n(data: &mut Vec<u64>, count: u32) {
    for i in 0..count {
        blink(data);
        // println!("{data:?}");
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

fn count_blink_all_n(data: &Vec<u64>, count: u32) -> u64 {
    let mut memo: HashMap<(u64, u32), u64> = HashMap::new();
    let mut sum: u64 = 0;
    for stone in data {
        sum += count_blink_n(*stone, count, &mut memo);
    }
    sum
}

fn count_blink_n(stone: u64, count: u32, memo: &mut HashMap<(u64, u32), u64>) -> u64 {
    if count == 0 {
        return 1;
    }
    if let Some(r) = memo.get(&(stone, count)) {
        return *r;
    }
    let r = match stone {
        0 => {
            count_blink_n(1, count - 1, memo)
        },
        x if digits(x) % 2 == 0 => {
            let (j, k) = split_stone(x);
            count_blink_n(j, count - 1, memo) + count_blink_n(k, count - 1, memo)
        },
        x => {
            count_blink_n(x * 2024, count - 1, memo)
        },
    };
    memo.insert((stone, count), r);
    r
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
