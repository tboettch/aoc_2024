use std::{fs::File, io::{self, Read}};

use grid::Offset;

#[derive(Debug, Clone)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

impl Machine {
    fn find_solution(&self) -> Option<(u64, u64)> {
        // This problem is the solution to the system of linear equations:
        // ⌈x1 x2⌉ ⌈a⌉ = ⌈x3⌉
        // ⌊y1 y2⌋ ⌊b⌋   ⌊y3⌋
        //
        // where "a" and "b" are the counts for each button press and the x and y values are as specified below.
        //
        // Unless the two equations are colinear, this will have exactly one solution.
        // However, the problem calls for only considering non-negative integer solutions.
        // Furthermore, the input data does not contain any colinear vectors, so we will exclude that
        // case from consideration. Basic algebra gives the closed form solution below.
        assert!(!Button::colinear(&self.button_a, &self.button_b), "unexpected colinear vectors: {:?} and {:?}", &self.button_a, &self.button_b);

        let (x1, y1) = (self.button_a.0 as i64, self.button_a.1 as i64);
        let (x2, y2) = (self.button_b.0 as i64, self.button_b.1 as i64);
        let (x3, y3) = (self.prize.0 as i64, self.prize.1 as i64);

        let b = int_divide(x1 * y3 - x3 * y1, x1 * y2 - x2 * y1)?;
        let a = int_divide(x3 - b * x2, x1)?;
        if a < 0 || b < 0 {
            return None
        }
        Some((a as u64, b as u64))
    }

    fn min_cost_solve_all(machines: &[Machine]) -> u64 {
        machines.iter()
            .filter_map(|m|
                m.find_solution().map(Self::cost)
            )
            .sum()
    }

    fn cost(counts: (u64, u64)) -> u64 {
        3 * counts.0 + counts.1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Button(u64, u64);

impl Button {
    fn to_offset(&self) -> Offset {
        Offset::new(self.0 as isize, self.1 as isize)
    }

    fn colinear(lhs: &Button, rhs: &Button) -> bool {
        // For the special case where a component is zero, the corresponding component in the other vector must also be zero.
        if (lhs.0 == 0) ^ (rhs.0 == 0) || (lhs.1 == 0) ^ (rhs.1 == 0) {
            return false;
        }
        // The vectors are colinear if they have the same slope=y/x. Solving this equation yields the test below.
        lhs.0 * rhs.1 == lhs.1 * rhs.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Prize(u64, u64);

impl Prize {
    fn to_offset(&self) -> Offset {
        Offset::new(self.0 as isize, self.1 as isize)
    }
}

fn main() -> io::Result<()>{
    // let filename = "example.txt";
    let filename = "input.txt";
    let machines = read_data(filename)?;
    // println!("machines: {machines:?}");
    println!("part 1 min cost: {}", Machine::min_cost_solve_all(&machines));

    const TARGET_OFFSET: u64 = 10000000000000;
    let machines: Vec<Machine> = machines.into_iter().map(|mut m| {
        m.prize.0 += TARGET_OFFSET;
        m.prize.1 += TARGET_OFFSET;
        m
    }).collect();
    println!("part 2 min cost: {}", Machine::min_cost_solve_all(&machines));
    
    Ok(())
}

fn int_divide(numerator: i64, denominator: i64) -> Option<i64> {
    if numerator % denominator == 0 { Some(numerator / denominator)} else {None}
}

fn read_data(filename: &str) -> io::Result<Vec<Machine>> {
    let mut buf = String::new();
    File::open(filename)?.read_to_string(&mut buf)?;
    parser::parse(&buf)
}

mod parser {
    use winnow::{ascii::digit1, combinator::{separated, separated_pair, terminated}, prelude::*, token::{literal, one_of}};
    use super::*;

    pub fn parse(input: &str) -> io::Result<Vec<Machine>> {
        machines.parse(input).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, e.to_string())
        })
    }

    fn machines(input: &mut &str) -> PResult<Vec<Machine>> {
        separated(0.., machine, literal('\n')).parse_next(input)
    }

    fn machine(input: &mut &str) -> PResult<Machine> {
        let button_a = terminated(button, literal('\n')).parse_next(input)?;
        let button_b = terminated(button, literal('\n')).parse_next(input)?;
        let prize = terminated(prize, literal('\n')).parse_next(input)?;
        Ok(Machine { button_a, button_b, prize })
    }

    fn button(input: &mut &str) -> PResult<Button> {
        literal("Button ").parse_next(input)?;
        one_of(('A', 'B')).parse_next(input)?;
        literal(": ").parse_next(input)?;
        separated_pair(offset, literal(", "), offset)
            .map(|(x,y)| Button(x,y))
            .parse_next(input)
    }

    fn offset(input: &mut &str) -> PResult<u64> {
        one_of(('X', 'Y')).parse_next(input)?;
        literal('+').parse_next(input)?;
        parse_u64.parse_next(input)
    }

    fn prize(input: &mut &str) -> PResult<Prize> {
        literal("Prize: ").parse_next(input)?;
        separated_pair(target, literal(", "), target)
            .map(|(x,y)| Prize(x,y))
            .parse_next(input)
    }

    fn target(input: &mut &str) -> PResult<u64> {
        one_of(('X', 'Y')).parse_next(input)?;
        literal('=').parse_next(input)?;
        parse_u64.parse_next(input)
    }

    fn parse_u64(input: &mut &str) -> PResult<u64> {
        digit1.try_map(|s: &str| s.parse()).parse_next(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10000))]

        #[test]
        fn check_colinear(x in 0u64..100, y in 0u64..100, scale in 1u64..100) {
            let a = Button(x,y);
            let b = Button(x*scale, y*scale);
            let c = Button(x+1, y);
            assert!(Button::colinear(&a, &b));
            if y != 0 {
                assert!(!Button::colinear(&a, &c));
            }
        }
    }
}

