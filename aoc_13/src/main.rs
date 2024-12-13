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
        // Basic algebra gives the closed form solution below.

        // Note: No colinear vectors actually occur in the input from the site
        if Button::colinear(&self.button_a, &self.button_b) {
            return self.solve_colinear();
        }

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

    fn solve_colinear(&self) -> Option<(u64, u64)> {
        debug_assert!(Button::colinear(&self.button_a, &self.button_b));
        // Because the vectors are colinear, we can compare their magnitude by choosing one of the components arbitrarily.
        // Per the problem description, button B has a three-times cost advantage.
        let mag_a = self.button_a.0;
        let mag_b = self.button_b.0;
        let (large_button, small_button) = if mag_a > mag_b * 3 { (&self.button_a, &self.button_b) } else { (&self.button_b, &self.button_a) };
        let lcm = lcm(large_button.0 as i64, small_button.0 as i64);
        let target = self.prize.to_offset();
        let (mut d, _) = target.div_mod_max(&large_button.to_offset());
        loop {
            if d < 0 {
                return None;
            }
            let rem = &target - (large_button.to_offset() * d);
            // TODO: Is this condition correct?
            if rem.x() > lcm as isize {
                return None;
            }
            let (d2, rem2) = rem.div_mod_max(&small_button.to_offset());
            if rem2.is_zero() {
                return Some(if large_button == &self.button_a { (d as u64, d2 as u64) } else { (d2 as u64, d as u64) });
            }
            d -= 1;
        }
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
    // let filename = "colinear.txt";
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

fn gcd(x: i64, y: i64) -> i64 {
    _gcd(x.abs(), y.abs())
}

fn _gcd(x: i64, y: i64) -> i64 {
    if x == 0 { return y; }
    _gcd(y % x, x)
}

fn lcm(x: i64, y: i64) -> i64 {
    let d = gcd(x,y);
    let x_component = x / d;
    let y_component = y / d;
    x_component * y_component * d 
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

    fn check_solution(solution: (u64, u64), machine: &Machine) {
        assert_eq!(machine.prize.0, solution.0 * machine.button_a.0 + solution.1 * machine.button_b.0, "{:?}", solution);
        assert_eq!(machine.prize.1, solution.0 * machine.button_a.1 + solution.1 * machine.button_b.1, "{:?}", solution);
    }

    #[test]
    fn manual_colinear() {
        let machines = read_data("colinear.txt").unwrap();
        assert_eq!(3, machines.len());
        for machine in machines.iter() {
            assert!(Button::colinear(&machine.button_a, &machine.button_b));
        }

        let sol1 = machines[0].find_solution().unwrap();
        assert_eq!(sol1.0, 0);
        check_solution(sol1, &machines[0]);
        let sol2 = machines[1].find_solution().unwrap();
        assert_eq!(sol2.1, 0);
        check_solution(sol2, &machines[1]);
        assert!(Machine::cost(sol1) > Machine::cost(sol2));

        let sol3 = machines[2].find_solution().unwrap();
        check_solution(sol3, &machines[2]);
    }

    fn base_button() -> impl Strategy<Value = Button> {
        (1u64..1000, 1u64..1000).prop_map(|(x,y)| Button(x,y))
    }

    fn colinear_pair() -> impl Strategy<Value = (Button, Button)> {
        (base_button(), 1u64..10).prop_flat_map(|(button, scale1)| {
            (scale1+1..20).prop_map(move |scale2| {
                (Button(button.0 * scale1, button.1 * scale1), Button(button.0 * scale2, button.1 * scale2))
            })
        })
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000000))]

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

        #[test]
        fn check_solve_colinear((button_b, button_a) in colinear_pair(), count_a in 1u64..1000, count_b in 1u64..1000) {
            assert!(Button::colinear(&button_a, &button_b));
            let machine = Machine {
                prize: Prize(button_a.0 * count_a + button_b.0 * count_b, button_a.1 * count_a + button_b.1 * count_b),
                button_a,
                button_b,
            };

            check_solution((count_a, count_b), &machine);
            let naive_cost = Machine::cost((count_a, count_b));
            let solution = machine.find_solution().unwrap();
            check_solution(solution, &machine);
            let solution_cost = Machine::cost(solution);
            assert!(solution_cost <= naive_cost);
        }
    }
}

