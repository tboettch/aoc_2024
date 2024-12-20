use std::{fs::File, io::{self, prelude::*}};
use winnow::{ascii::digit1, combinator::{repeat, separated, terminated}, prelude::*, token::literal};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Equation {
    total: u64,
    components: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Op {Add, Mul, Concat}

impl Op {
    const ALL: [Op; 3] = [Op::Add, Op::Mul, Op::Concat];

    fn apply(&self, l: u64, r: u64) -> u64 {
        match self {
            Op::Add => l + r,
            Op::Mul => l * r,
            Op::Concat => {
                l * 10u64.pow(mag(r) + 1) + r
            }
        }
    }
}

fn mag(x: u64) -> u32 {
    (x as f64).log10().trunc() as u32
}

fn main() -> io::Result<()> {
    // let filename = "example.txt";
    let filename = "input.txt";
    let equations = read_input(filename)?;
    // println!("{equations:?}");
    println!("sum solveable: {}", sum_solveable(&equations));
    Ok(())
}

fn sum_solveable(equations: &[Equation]) -> u64 {
    equations.iter()
        .filter(|e| solveable(e))
        .map(|e| e.total)
        .sum()
}

fn solveable(equation: &Equation) -> bool {
    let totals = totals(&equation.components);
    // totals.iter().for_each(|total| println!(" equation: {equation:?}, total: {total}"));
    for total in totals.iter() {
        if *total == equation.total {
            return true;
        }
    }
    false
}

fn totals(vals: &[u64]) -> Vec<u64> {
    if vals.len() == 1 {
        return vals.to_vec();
    }
    let subtotals = totals(&vals[..vals.len() - 1]);
    let mut result = Vec::with_capacity(subtotals.len() * Op::ALL.len());
    for op in Op::ALL {
        for subtotal in &subtotals {
            let rhs = vals[vals.len() - 1];
            let val = op.apply(*subtotal, rhs);
            // println!("{:?}({},{})={}", op, *subtotal, rhs, val);
            result.push(val);
        }
    }
    result
}

fn read_input(filename: &str) -> io::Result<Vec<Equation>> {
    let mut buf = String::new();
    File::open(filename)?.read_to_string(&mut buf)?;
    equations.parse(&buf).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, e.to_string())
    })
}

fn equations(input: &mut &str) -> PResult<Vec<Equation>> {
    repeat(1.., terminated(equation, literal('\n'))).parse_next(input)
}

fn equation(input: &mut &str) -> PResult<Equation> {
    let (total, _, components) = (parse_u64, literal(": "), separated(2.., parse_u64, " ")).parse_next(input)?;
    Ok(Equation { total, components })
}

fn parse_u64(input: &mut &str) -> PResult<u64> {
    digit1.try_map(|s: &str| s.parse()).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10000))]

        // Because this is throw-away code, these tests exclude integer overflow cases.

        #[test]
        fn check_concat(l in (0..1000000u64), r in (0..1000000u64)) {
            let by_string: u64 = (l.to_string() + r.to_string().as_ref()).parse().unwrap();
            let by_op = Op::Concat.apply(l, r);
            assert_eq!(by_op, by_string);
        }

        #[test]
        fn check_concat_zero_left(r in (0..u64::MAX / 2)) {
            assert_eq!(Op::Concat.apply(0, r), r);
        }

        #[test]
        fn check_concat_zero_right(l in (0..u64::MAX / 10)) {
            assert_eq!(Op::Concat.apply(l, 0), l * 10);
        }

        #[test]
        fn check_mag(x: u64) {
            assert_eq!(mag(x) + 1, x.to_string().chars().count() as u32);
        }
    }
}