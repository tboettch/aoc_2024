use std::{fmt::Display, ops::{Add, Index, IndexMut, Mul, Sub}};
#[cfg(feature = "arbitrary")] use proptest::arbitrary::Arbitrary;
#[cfg(feature = "arbitrary")]use proptest_derive::Arbitrary;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position(usize, usize);

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        if x > isize::MAX as usize || y > isize::MAX as usize {
            panic!("Rejecting large index ({x},{y})");
        }
        Self(x,y)
    }

    pub fn to_offset(&self) -> Option<Offset> {
        Some(Offset(self.0.try_into().ok()?, self.1.try_into().ok()?))
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }
}

impl Add<&Offset> for &Position {
    type Output = Option<Position>;
    fn add(self, rhs: &Offset) -> Self::Output {
        let x = self.0.checked_add_signed(rhs.0)?;
        let y = self.1.checked_add_signed(rhs.1)?;
        Some(Position(x,y))
    }
}

impl Sub<&Offset> for &Position {
    type Output = Option<Position>;
    fn sub(self, rhs: &Offset) -> Self::Output {
        let x = self.0.checked_add_signed(rhs.0 * -1)?;
        let y = self.1.checked_add_signed(rhs.1 * -1)?;
        Some(Position(x,y))
    }
}

impl Sub<&Position> for &Position {
    type Output = Offset;
    fn sub(self, rhs: &Position) -> Self::Output {
        Offset(self.0 as isize - rhs.0 as isize, self.1 as isize - rhs.1 as isize)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

#[cfg(feature = "arbitrary")]
impl Arbitrary for Position {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        (0..(isize::MAX as usize), 0..(isize::MAX as usize)).prop_map(|(x,y)| Position::new(x,y)).boxed()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct Offset(isize, isize);

impl Offset {
    pub fn new(x: isize, y: isize) -> Offset {
        Self(x,y)
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }
}

impl Add<&Offset> for &Offset {
    type Output = Offset;
    fn add(self, rhs: &Offset) -> Self::Output {
        Offset(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<&Offset> for &Offset {
    type Output = Offset;
    fn sub(self, rhs: &Offset) -> Self::Output {
        Offset(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<isize> for &Offset {
    type Output = Offset;
    fn mul(self, rhs: isize) -> Self::Output {
        Offset(self.0 * rhs, self.1 * rhs)
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:+},{:+})", self.0, self.1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl <T> Grid<T> {
    pub fn new(data: Vec<T>, width: usize, height: usize) -> Grid<T> {
        Grid {data, width, height}
    }

    pub fn in_bounds(&self, pos: &Position) -> bool {
        pos.0 < self.width && pos.1 < self.height
    }

    fn index_of(&self, pos: &Position) -> usize {
        if !self.in_bounds(pos) {
            panic!("Out of bounds index: {pos}");
        }
        self.to_index(pos.0, pos.1)
    }

    fn to_index(&self, x: usize, y: usize) -> usize {
        if x >= self.width || y >= self.height {
            panic!("Out of bounds index: ({x},{y})");
        }
        x + y * self.width
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn map<F, S>(&self, mut f: F) -> Grid<S>
    where
        F: FnMut(&T) -> S,
    {
        let r = self.data.iter().map(|x| f(x)).collect::<Vec<_>>();
        Grid { data: r, width: self.width, height: self.height }
    }

    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }

    pub fn get(&self, pos: &Position) -> Option<&T> {
        if self.in_bounds(pos) {
            Some(&self[pos])
        } else {
            None
        }
    }
}

impl <T> Index<&Position> for Grid<T> {
    type Output = T;

    fn index(&self, pos: &Position) -> &Self::Output {
        &self.data[self.index_of(pos)]
    }
}

impl <T> IndexMut<&Position> for Grid<T> {
    fn index_mut(&mut self, pos: &Position) -> &mut Self::Output {
        let index = self.index_of(pos);
        &mut self.data[index]
    }
}

impl <T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.data[self.to_index(x,y)])?;
            }
            if y < self.height - 1 {
                writeln!(f, "")?;
            }
        }
        Ok(())
    }
}

#[cfg(feature = "arbitrary")]
impl <T: Arbitrary> Arbitrary for Grid<T> {
    type Parameters = ();
    type Strategy = proptest::prelude::BoxedStrategy<Self>;
    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        (1..100usize, 1..100usize).prop_flat_map(|(width, height)| {
            proptest::collection::vec(any::<T>(), width * height).prop_map(move |data| {
                Grid { data, width, height }
            })
        }).boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig {
            max_shrink_iters: 100000,
            ..ProptestConfig::default()
        })]

        #[test]
        fn check_position_eq_reflexive(x: Position) {
            assert_eq!(x, x);
            assert_eq!(x, x.clone());
        }

        #[test]
        fn check_offset_eq_reflexive(x: Offset) {
            assert_eq!(x, x);
            assert_eq!(x, x.clone());
        }

        #[test]
        fn check_position_ord(mut vals: Vec<Position>) {
            vals.sort();
            assert!(vals.is_sorted());
        }

        #[test]
        fn check_offset_ord(mut vals: Vec<Offset>) {
            vals.sort();
            assert!(vals.is_sorted());
        }

        #[test]
        fn check_offset_zero(offset: Offset) {
            assert_eq!(offset, &offset + &Offset::new(0,0));
            assert_eq!(offset, &offset - &Offset::new(0,0));
            assert_eq!(Offset::new(0,0), &offset * 0);
        }

        #[test]
        fn check_position_sub(x: Position, y: Position) {
            let diff = &x - &y;
            assert_eq!(x, (&y + &diff).unwrap());
            assert_eq!(y, (&x - &diff).unwrap());
            assert_eq!(diff.is_zero(), x == y);
        }

        #[test]
        fn check_transitivity(x: Position, y: Position, z: Position) {
            let diff1 = &x - &y;
            let diff2 = &y - &z;
            let diff3 = &diff1 + &diff2;
            assert_eq!(x, (&z + &diff3).unwrap());
        }

        #[test]
        fn check_grid_index(mut grid: Grid<u32>) {
            for y in 0..grid.height() {
                for x in 0..grid.width() {
                    let pos = Position::new(x,y);
                    assert!(grid.in_bounds(&pos));
                    grid[&pos] = 0;
                }
            }
        }
    }
}